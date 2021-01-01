use std::process::Command;
use std::error::Error;
use std::process;
use std::io::{self, Write};

/**
 * parallel
 * ntimes 100 -- curl 'https://..'
 *
 * sync
 * ntimes 100 -sync -- curl 'https://..'
 */
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing command line args: {}", err);
        process::exit(1);
    });

    let mut cmd = Command::new(config.cmd_name);
    cmd.args(config.positional_args);

    if config.command_args.first() == Some(&"-sync".to_string()) {
        if let Err(e) = ntimes_sync(config.count, &mut cmd) {
            // ensure if redirect to a new file that error shows up onscreen
            eprintln!("Error processing sync reqeusts: {}", e);
            process::exit(1);
        }
    } else {
        if let Err(e) = ntimes_parallel(config.count, &mut cmd) {
            // ensure if redirect to a new file that error shows up onscreen
            eprintln!("Error processing parallel reqeusts: {}", e);
            process::exit(1);
        }
    }
}

fn ntimes_parallel(count: usize, cmd: &mut Command) -> Result<(), Box<dyn Error>> {
    for _i in 0..count {
        // child process parallel.  spawn give stdout and stderror to parent process
        cmd.spawn()?;
    }

    Ok(())
}

fn ntimes_sync(count: usize, cmd: &mut Command) -> Result<(), Box<dyn Error>> {
    for _i in 0..count {
        // child process sync. stdout and stderr are captured by this child process
        let output = cmd.output()?;

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success());
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
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let mut iter = args.iter();

        // we consumed `--` so it wont show up in positional_args
        let command_args: Vec<String> = iter.by_ref().take_while(|input| *input != "--").cloned().collect();
        if command_args.len() < 1 {
            return Err("Must provide number of times you want this comand to run");
        }

        let positional_args: Vec<String> = iter.cloned().collect();
        if positional_args.len() < 1 {
            return Err("Positional args after `--` were not provided");
        }

        let count: usize = command_args[0].parse().expect("A number was not provided");
        let cmd_name = &positional_args[0];

        Ok(Self {
            cmd_name: cmd_name.clone(),
            count,
            command_args: command_args[1..].to_owned(),
            positional_args: positional_args[1..].to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn ntimes_parallel_works() {
        let mut cmd = Command::new("true");
        let result = ntimes_parallel(2, &mut cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn config_works() {
        let v = vec_of_strings!["2", "--", "curl", "https://"];
        let config = Config::new(v).unwrap();
        assert_eq!(config.cmd_name, "curl".to_string());
        assert_eq!(config.count, 2);
        assert_eq!(config.positional_args, vec_of_strings!["https://"]);
        assert_eq!(config.command_args, Vec::<String>::new());
    }

    #[test]
    #[should_panic]
    fn config_errors_without_args() {
        let v = vec_of_strings!["2", "--"];
        Config::new(v).unwrap();
    }
}
