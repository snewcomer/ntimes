use std::process::Command;
use std::error::Error;
use std::process;

/**
 * ntimes 100 -- curl 'https://..'
 */
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let config: Config = Config::new(args);

    if let Err(e) = ntimes(config.count, &config.cmd_name, &config.positional_args) {
        // ensure if redirect to a new file that error shows up onscreen
        eprintln!("Error {}", e);
        process::exit(1);
    }
}

fn ntimes(count: usize, cmd_name: &str, positional_args: &[String]) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new(cmd_name);
    cmd.args(positional_args);

    for _i in 0..count {
        // child process concurrent.  spawn give stdout and stderror to parent process
        cmd.spawn()?;
    }

    Ok(())
}

#[derive(Debug)]
struct Config {
    cmd_name: String,
    count: usize,
    command_args: Vec<String>,
    positional_args: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Self {
        let mut iter = args.iter();

        // we consumed `--` so it wont show up in positional_args
        let command_args: Vec<String> = iter.by_ref().take_while(|input| *input != "--").cloned().collect();
        assert!(command_args.len() > 0, "Must provide number of times you want this comand to run");

        let positional_args: Vec<String> = iter.cloned().collect();
        assert!(positional_args.len() > 0, "Positional args after -- were not provided");

        let count: usize = command_args[0].parse().expect("A number was not provided");
        let cmd_name = &positional_args[0];

        Self {
            cmd_name: cmd_name.clone(),
            count,
            command_args: command_args[1..].to_owned(),
            positional_args: positional_args[1..].to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn config_works() {
        let v = vec_of_strings!["2", "--", "curl", "https://"];
        let config = Config::new(v);
        assert_eq!(config.cmd_name, "curl".to_string());
        assert_eq!(config.count, 2);
        assert_eq!(config.positional_args, vec_of_strings!["https://"]);
        assert_eq!(config.command_args, Vec::<String>::new());
    }

    #[test]
    #[should_panic]
    fn config_errors_without_args() {
        let v = vec_of_strings!["2", "--"];
        let config = Config::new(v);
    }
}
