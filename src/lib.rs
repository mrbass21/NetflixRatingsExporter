use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum ConfigResult <E>{
    HelpMenu,
    Ok,
    Err(E),
}

#[derive(PartialEq, Debug)]
pub enum MovieFilter {
    None,
    IgnoreRemoved,
}


#[derive(PartialEq, Debug)]
pub struct Config {
    pub movie_filter: MovieFilter,
    pub session_token: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            movie_filter: MovieFilter::None,
            session_token: None,
        }
    }

    pub fn set_movie_filter(&mut self, new_value: MovieFilter) -> &mut Config {
        self.movie_filter = new_value;
        self
    }

    pub fn set_session_token(&mut self, provided_session_token: String) -> &mut Config {
        self.session_token = Some(provided_session_token);
        self
    }

    pub fn from_args(&mut self , args: &[String]) -> ConfigResult<String> {
        
        let mut skip_next = false;
        for (i, arg) in args.iter().enumerate() {
            if i == 0 || skip_next == true {
                //Skip
                if skip_next {
                    skip_next = false;
                }
                continue;
            }
            match arg.as_ref() {
                "-i" => self.movie_filter = MovieFilter::IgnoreRemoved,
                x if (x == "-h" || x == "--help" || x == "-?") => {
                    //TODO: Figure out how to handle this in a Builder pattern
                    return ConfigResult::HelpMenu
                },
                x if (x == "-c" || x == "--cookie") => {
                    self.session_token = Some((args[i + 1]).to_owned());
                    skip_next = true;
                },
                _ => {
                    return ConfigResult::Err(format!("Unknown argument: {}", arg))
                },
            }
        }

        ConfigResult::Ok
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_args() {
        let args: Vec<String> = vec![String::from("ProgramName"), String::from("-i")];

        assert!(Config::new(&args).is_ok());
    }

    #[test]
    fn invalid_args() {
        let args: Vec<String> = vec![String::from("ProgramName"), String::from("-f")];

        assert!(Config::new(&args).is_err());
    }

    #[test]
    fn help_menu() {
        let args: Vec<String> = vec![String::from("ProgramName"), String::from("-h")];

        let config_result: ConfigResult = Config::new(&args).unwrap();
        assert_eq!(config_result, ConfigResult::HelpMenu);
    }

    #[test]
    fn set_session_cookie() {
        let args: Vec<String> = vec![String::from("ProgramName"), String::from("-c"), String::from("THIS_IS_A_COOKIE")];

        let config_result = Config::new(&args);

        assert!(config_result.is_ok());
        
        if let ConfigResult::Result(config) = config_result.unwrap() {
            assert_eq!(config.session_cookie,"THIS_IS_A_COOKIE");
        }
    }
}
