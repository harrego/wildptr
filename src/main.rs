mod cli;
mod api;
use crate::cli::*;
use std::process;

// arguments
static LIST_ARG: Argument = Argument { name: "list", description: "list all RFCs", arg: false, callback: list_arg_cb };
static RFC_ARG: Argument = Argument { name: "rfc", description: "specify info on a specific rfc", arg: true, callback: rfc_arg_cb };
// flags
static VERBOSE_FLAG: Flag = Flag { flags: &["v", "verbose"], description: "verbose mode" };

// helper that prints all info about an rfc
// rfc: input rfc
// minimal: only list minimal data not to spam stdout
fn print_rfc(rfc: &api::WildpointerRFC, minimal: bool) {
  println!("RFC {}", rfc.id);
  println!("Status: {}", rfc.status);
  println!("Info Provided: {}", rfc.info_provided);
  println!("Date: {}", rfc.date);
  if !minimal {
    // check if optional values are present
    if let Some(last_updated) = &rfc.last_updated {
      println!("Last updated: {}", last_updated);
    }
    if let Some(body) = &rfc.body {
      println!("Body: {}", body);
    }
  }
}

// callback function for the "list" argument
fn list_arg_cb(_cli: &CLI, active_cli_flags: &Vec<&Flag>, _arg: &Option<String>) {
  // check if verbose flag was included
  let mut verbose = false;
  for flag in active_cli_flags {
    // check if the ref is the verbose flag
    if std::ptr::eq(*flag, &VERBOSE_FLAG) {
      verbose = true;
      // no other flags to look for, break the loop
      break;
    }
  }  

  // get all the rfcs from the api
  let rfcs = api::all();
  for rfc in rfcs {
    // use the helper function, with a new line suffix
    print_rfc(&rfc, !verbose);
    println!();
  }
}

// callback function for the "rfc" argument, this takes an argument in the form of the rfc id
fn rfc_arg_cb(_cli: &CLI, _active_cli_flags: &Vec<&Flag>, arg: &Option<String>) {
  // only run if rfc id is given
  if let Some(rfc_id) = arg {
    // get all rfcs from api
    let rfcs = api::all();
    for rfc in rfcs {
      // loop thru rfcs, find matching rfc id, display data, then break loop
      if rfc.id == *rfc_id {
        print_rfc(&rfc, false);
        break;
      }
    }
  } else {
    // exit if rfc id not given
    println!("error: must provide rfc id");
    process::exit(1);
  } 
}

fn main() {
  // wildptr arguments: "list" and "rfc"
  static WILDPTR_ARGS: &'static[&'static Argument] = &[&LIST_ARG, &RFC_ARG];
  // wildptr flags: (-v | --verbose)
  static WILDPTR_FLAGS: &'static[&'static Flag] = &[&VERBOSE_FLAG];
  // the main cli app
  static WILDPTR_CLI: CLI = CLI { app_name: "wildptr", app_version: "1.0.0", app_description: "(c) harrego", arguments: &WILDPTR_ARGS, flags: &WILDPTR_FLAGS }; 
  // run the main function, parsing the argument variables (argv) given
  WILDPTR_CLI.parse();
}
