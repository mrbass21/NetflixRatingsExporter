use std::error::Error;

#[derive(PartialEq, Debug)]
pub enum ConfigResult {
    HelpMenu,
    Result(Config)
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub ignore_removed_movies: bool,
    pub session_cookie: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<ConfigResult, String> {

        let mut config = Config {ignore_removed_movies:false, session_cookie:String::new()};

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
                "-i" => config.ignore_removed_movies = true,
                x if (x == "-h" || x == "--help" || x == "-?") => {
                    return Ok(ConfigResult::HelpMenu)
                },
                x if (x == "-c" || x == "--cookie") => {
                    //Can I get the next item here?
                    config.session_cookie = (&args[i + 1]).to_owned();
                    skip_next = true;
                },
                _ => {
                    return Err(format!("Unknown argument: {}", arg))
                },
            }
        }

        Ok(ConfigResult::Result(config))
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
    fn helpMenu() {
        let args: Vec<String> = vec![String::from("ProgramName"), String::from("-h")];

        let config_result: ConfigResult = Config::new(&args).unwrap();
        assert_eq!(config_result, ConfigResult::HelpMenu);
    }
}
