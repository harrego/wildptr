mod cli;
use crate::cli::*;

fn list_arg(cli: &CLI, active_cli_flags: &Vec<&Flag>) {
  println!("list argument called, with following flags:");
  for flag in active_cli_flags {
    println!("  - {}", flag.description.to_string());
  }
}

fn main() {
  let args = vec![Argument { name: "list".to_string(), description: "list all RFCs".to_string(), callback: list_arg } ];
  let flags = vec![Flag { flags: vec!["v".to_string(), "verbose".to_string()], description: "verbose mode".to_string() }];
  let cli = CLI { app_name: "wildptr".to_string(), app_version: "1.0.0".to_string(), app_description: "(c) harrego".to_string(), arguments: args, flags: flags }; 
  cli.parse();
}
