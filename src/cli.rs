use substring::Substring;
use std::process;

// argument used in cli, an argument is the main branch to a function e.g. `app help`
pub struct Argument {
  // name of the argument, used in the command
  pub name: &'static str,
  // description of the command in help
  pub description: &'static str,
  // does the argument accept a second argument? (e.g. list <argument>)
  pub arg: bool,

  // callback when argument is called
  // cli: the cli struct parent
  // active_flags: active flags when argument was called
  // arg: optional argument given with argument, will never be given if `arg` is false
  pub callback: fn(&CLI, &Vec<&Flag>, &Option<String>),
}

// flag used in cli, a flag is an additional option provided alongside a argument e.g. `app help -v`
pub struct Flag {
  // string values of accepted flags, one char flags can be chained with others prefixed with -
  // e.g. -va, longer flags can be prefixed with -- e.g. --verbose 
  pub flags: &'static[&'static str],
  // description of the flag displayed in the help
  pub description: &'static str,
}

// the main cli struct
pub struct CLI {
  // app name, displayed in the help
  pub app_name: &'static str,
  // app version, displayed in the help
  pub app_version: &'static str,
  // short description, displayed in the help e.g. (c) harrego
  pub app_description: &'static str,

  // arguments, see `Argument` for more info
  pub arguments: &'static[&'static Argument],
  // flags, see `Flag` for more info
  pub flags: &'static[&'static Flag],
}
impl CLI {
  // helper function to generate a help document and print to stdout
  pub fn print_help(&self) {
    // boilerplate description
    println!("{} v{}", self.app_name, self.app_version);
    println!("{}", self.app_description);

    // print the flags
    print!("\nusage: wildptr ");
    // loop for each flags
    for flag in self.flags {
      print!("[");
      for (index, accepted_flag) in flag.flags.iter().enumerate() {
        // check length of flag to determine prefix (- or --)
        if accepted_flag.chars().count() > 1 {
          print!("--{}", accepted_flag);
        } else {
          print!("-{}", accepted_flag);
        }
        // if not the last flag then print the separator
        if index != flag.flags.len() - 1 {
          print!(" | ");
        }
      }
      // end of flag printing
      print!("] ");
    }
    print!("\n");

    // print the arguments
    println!("\narguments:");
    for arg in self.arguments {
      if arg.arg {
        // if the argument accepts a second argument, display it in the document
        println!("    {} <arg> - {}", arg.name, arg.description);
      } else {
        // conventional argument w/o a second argument
        println!("    {} - {}", arg.name, arg.description);
      }
    }
  }

  // function that matches a flag name string with a Flag struct, is optional and is None if no match found
  fn handle_flag(&self, flag: &String) -> Option<&Flag> {
    // loop all cli flags
    for cli_flag in self.flags {
      // loop all the accepted flags (e.g. v or verbose) for that flag
      for str_flag in cli_flag.flags {
        // if a flag matches the string, return it and as such break the loop
        if str_flag == flag {
          return Some(cli_flag);
        }
      }
    }
    // loop completed w/o breaking and as such no flag was found
    return None;
  }

  // helper error function when an invalid flag is given, will exit w/ code 1
  fn invalid_flag(&self, flag: &String) {
    println!("error: invalid flag \"{}\"", flag);
    println!("       use argument \"help\" to display flags");
    process::exit(1);
  }

  // main cli function, parse the cli argument variables (argv) given
  pub fn parse(&self) {
    // setup initial state

    // the final selected cli argument, must be satisfied by end of loop
    let mut selected_cli_argument: Option<&Argument> = None;
    // any flags given alongside argument
    let mut active_cli_flags: Vec<&Flag> = Vec::new();
    // a second argument, an optional in all cases
    let mut second_argument: Option<String> = None;

    // get the argvs
    let args = std::env::args();
    for (index, arg) in args.enumerate() {
      // skip the first var, is always the program name
      if index == 0 {
        continue;
      }
      // split the var into chars
      let mut chars = arg.chars();
      // if starts w/ - then is a flag
      if chars.next() == Some('-') {
        let dash_suffix = chars.next();
        // if another - is found this is a long flag
        if dash_suffix == Some('-') {
          // double loaded long flag 

          // unsure if using .count() at the end of the substring
          // method is safe for unicode, are chars() ascii or unicode
          // based?

          // take the given arg (minus the prefixed dashes)
          let flag_str = &arg.substring(2, arg.chars().count()).to_string();
          // find a match
          if let Some(flag) = self.handle_flag(flag_str) {
            // if match found then push to state vec
            active_cli_flags.push(flag);
          } else {
            // invalid match, error and stop program
            self.invalid_flag(flag_str);
          }
        } else if let Some(unhandled_arg) = dash_suffix {
          // single dash multi loaded flag

          // because incremented char stream by looking for double dash, handle the already streamed
          // long character as a flag
          if let Some(flag) = self.handle_flag(&unhandled_arg.to_string()) {
            active_cli_flags.push(flag);
          } else {
            self.invalid_flag(&unhandled_arg.to_string());
          }

          // loop the remaining chars
          loop {
            // check if next char is available
            if let Some(val) = chars.next() {
              // same code as above, check for match, stop app if none found
              if let Some(flag) = self.handle_flag(&val.to_string()) {
                active_cli_flags.push(flag);
              } else {
                self.invalid_flag(&val.to_string());
              }
            } else {
              // if next char isn't available then stop loop
              break;
            }
          }
        }
      } else if let Some(selected_arg) = selected_cli_argument {
        // ^ variable is not a flag, check if selected argument has already been found

        // if the selected argument has been found and accepts a second arg that hasn't already been found...
        if selected_arg.arg && second_argument.is_none() {
          // ...satisfy the state variable
          second_argument = Some(arg);
        } else {
          // otherwise stop app, too much input given
          println!("error: more than one argument provided");
          process::exit(1);
        }
      } else {
        // ^ variable is a not a flag, and argument has not yet been found

        // hard coded argument for help
        if arg == "help" {
          // use the helper help print
          &self.print_help();
          return;
        }

        // loop all cli arguments
        for cli_argument in self.arguments {
          // look for a match
          if cli_argument.name == arg {
            // if a match is found then set state variable and break out of loop
            selected_cli_argument = Some(cli_argument);
            break;
          }
        }
        // if loop finds no match then an invalid argument has been given, error
        if selected_cli_argument.is_none() {
          println!("error: unrecognized argument \"{}\"", arg);
          println!("       use argument \"help\" to list available arguments");
          process::exit(1);
        }
      }
    }

    // end of loop, check if a final argument was found
    if let Some(arg) = selected_cli_argument {
      // call the callback of the found argument w/ the state variables
      (arg.callback)(&self, &active_cli_flags, &second_argument);
    } else {
      // no final cli argument was found, exit
      println!("error: no argument given");
      println!("       use \"help\" to list available arguments");
      process::exit(1);
    }
  }
}
