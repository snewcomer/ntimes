use std::process::Command;
use std::io::{self, Write};

/**
 * ntimes 100 -- curl 'https://..'
 */
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut iter = args.iter();
    // we consumed -- so it wont show up in positional_args
    let command_args: Vec<String> = iter.by_ref().take_while(|input| *input != "--").cloned().collect();
    assert!(command_args.len() > 0, "Must provide number of times you want this comand to run");

    let positional_args: Vec<String> = iter.cloned().collect();
    assert!(positional_args.len() > 0, "Positional args after -- were not provided");

    let count: u32 = command_args[0].parse().expect("A number was not provided");
    let cmd_name = &positional_args[0];

    ntimes(count, &cmd_name, &positional_args);
}

fn ntimes(count: u32, cmd_name: &str, positional_args: &Vec<String>) {
    let mut cmd = Command::new(cmd_name);
    cmd.args(&positional_args[1..]);
    for _i in 0..count {
        let output = cmd.output().expect("failed to execute command");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        assert!(output.status.success());
    }
}
