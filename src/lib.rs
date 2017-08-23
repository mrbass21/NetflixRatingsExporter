use std::error::Error;

pub struct Config {
    pub ignore_removed_movies: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {

        let mut config = Config {ignore_removed_movies:false};
        
        for (i, arg) in args.iter().enumerate() {
            if i == 0 {
                //Skip the EXE name
                continue;
            }
            match arg.as_ref() {
                "-i" => config.ignore_removed_movies = true,
                _ => {
                    return Err(format!("Unknown argument: {}", arg))
                },
            }
        }

        Ok(config)
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

        assert!(Config::new(&args).is_ok(), true);
    }

    #[test]
    fn invalid_args() {
        let args: Vec<String> = vec![String::from("ProgramName"), String::from("-f")];

        assert!(Config::new(&args).is_err(), true);
    }
}