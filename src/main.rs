mod cli;
mod api;
use crate::cli::*;

// arguments
static LIST_ARG: Argument = Argument { name: "list", description: "list all RFCs", arg: false, callback: list_arg_cb };
static RFC_ARG: Argument = Argument { name: "rfc", description: "specify info on a specific rfc", arg: true, callback: rfc_arg_cb };
// flags
static VERBOSE_FLAG: Flag = Flag { flags: &["v", "verbose"], description: "verbose mode" };

fn print_rfc(rfc: &api::WildpointerRFC, minimal: bool) {
  println!("RFC {}", rfc.id);
  println!("Status: {}", rfc.status);
  println!("Info Provided: {}", rfc.info_provided);
  println!("Date: {}", rfc.date);
  if !minimal {
    if let Some(last_updated) = &rfc.last_updated {
      println!("Last updated: {}", last_updated);
    }
    if let Some(body) = &rfc.body {
      println!("Body: {}", body);
    }
  }
}

fn list_arg_cb(_cli: &CLI, active_cli_flags: &Vec<&Flag>, _arg: &Option<String>) {
  let mut verbose = false;
  for flag in active_cli_flags {
    // check if the ref is the verbose flag
    if std::ptr::eq(*flag, &VERBOSE_FLAG) {
      verbose = true;
      // no other flags to look for, break the loop
      break;
    }
  }  

  let rfcs = api::all();
  for rfc in rfcs {
    print_rfc(&rfc, !verbose);
    println!();
  }
}

fn rfc_arg_cb(_cli: &CLI, _active_cli_flags: &Vec<&Flag>, arg: &Option<String>) {
  if let Some(rfc_id) = arg {
    let rfcs = api::all();
    for rfc in rfcs {
      if rfc.id == *rfc_id {
        print_rfc(&rfc, false);
        break;
      }
    }
  } else {
    println!("error: must provide rfc id");
  } 
}

fn main() {
  static WILDPTR_ARGS: &'static[&'static Argument] = &[&LIST_ARG, &RFC_ARG];
  static WILDPTR_FLAGS: &'static[&'static Flag] = &[&VERBOSE_FLAG];
  static WILDPTR_CLI: CLI = CLI { app_name: "wildptr", app_version: "1.0.0", app_description: "(c) harrego", arguments: &WILDPTR_ARGS, flags: &WILDPTR_FLAGS }; 
  WILDPTR_CLI.parse();
}
