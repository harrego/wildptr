mod cli;
mod api;
use crate::cli::*;

// arguments
static list_arg: Argument = Argument { name: "list", description: "list all RFCs", arg: false, callback: list_arg_cb };
static rfc_arg: Argument = Argument { name: "rfc", description: "specify info on a specific rfc", arg: true, callback: rfc_arg_cb };
// flags
static verbose_flag: Flag = Flag { flags: &["v", "verbose"], description: "verbose mode" };

fn list_arg_cb(cli: &CLI, active_cli_flags: &Vec<&Flag>, arg: &Option<String>) {
  let mut verbose = false;
  for flag in active_cli_flags {
    // check if the ref is the verbose flag
    if std::ptr::eq(*flag, &verbose_flag) {
      verbose = true;
      // no other flags to look for, break the loop
      break;
    }
  }  

  let rfcs = api::all();
  for rfc in rfcs {
    println!("RFC {}", rfc.id);
    println!("Status: {}", rfc.status);
    println!("Info Provided: {}", rfc.info_provided);
    println!("Date: {}", rfc.date);
    if verbose {
      if let Some(last_updated) = rfc.last_updated {
        println!("Last updated: {}", last_updated);
      }
      if let Some(body) = rfc.body {
        println!("Body: {}", body);
      }
    }
    println!();
  }
}

fn rfc_arg_cb(cli: &CLI, active_cli_flags: &Vec<&Flag>, arg: &Option<String>) {
  if let Some(rfc_id) = arg {

  } else {
    println!("error: must provide rfc id");
  } 
}

fn main() {
  static args: &'static[&'static Argument] = &[&list_arg, &rfc_arg];
  static flags: &'static[&'static Flag] = &[&verbose_flag];
  let cli = CLI { app_name: "wildptr", app_version: "1.0.0", app_description: "(c) harrego", arguments: &args, flags: &flags }; 
  cli.parse();
}
