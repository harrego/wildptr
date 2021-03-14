use substring::Substring;
use std::process;

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

pub struct Flag {
  pub flags: &'static[&'static str],
  pub description: &'static str,
}

pub struct CLI {
  pub app_name: &'static str,
  pub app_version: &'static str,
  pub app_description: &'static str,

  pub arguments: &'static[&'static Argument],
  pub flags: &'static[&'static Flag],
}
impl CLI {
  pub fn print_help(&self) {
    println!("{} v{}", self.app_name, self.app_version);
    println!("{}", self.app_description);

    print!("\nusage: wildptr ");
    for flag in self.flags {
      print!("[");
      for (index, accepted_flag) in flag.flags.iter().enumerate() {
        if accepted_flag.chars().count() > 1 {
          print!("--{}", accepted_flag);
        } else {
          print!("-{}", accepted_flag);
        }
        if index != flag.flags.len() - 1 {
          print!(" | ");
        }
      }
      print!("] ");
    }
    print!("\n");

    println!("\narguments:");
    for arg in self.arguments {
      if arg.arg {
        println!("    {} <arg> - {}", arg.name, arg.description);
      } else {
        println!("    {} - {}", arg.name, arg.description);
      }
    }
  }

  fn handle_flag(&self, flag: &String) -> Option<&Flag> {
    for cli_flag in self.flags {
      for str_flag in cli_flag.flags {
        if str_flag == flag {
          return Some(cli_flag);
        }
      }
    }
    return None;
  }

  fn invalid_flag(&self, flag: &String) {
    println!("error: invalid flag \"{}\"", flag);
    println!("       use argument \"help\" to display flags");
    process::exit(1);
  }

  pub fn parse(&self) {
    let mut selected_cli_argument: Option<&Argument> = None;
    let mut active_cli_flags: Vec<&Flag> = Vec::new();
    let mut second_argument: Option<String> = None;

    let args = std::env::args();
    for (index, arg) in args.enumerate() {
      if index == 0 {
        continue;
      }
      let mut chars = arg.chars();
      if chars.next() == Some('-') {
        let dash_suffix = chars.next();
        if dash_suffix == Some('-') {
          // double loaded long flag 

          // unsure if using .count() at the end of the substring
          // method is safe for unicode, are chars() ascii or unicode
          // based?
          let flag_str = &arg.substring(2, arg.chars().count()).to_string();
          if let Some(flag) = self.handle_flag(flag_str) {
            active_cli_flags.push(flag);
          } else {
            self.invalid_flag(flag_str);
          }
        } else if let Some(unhandled_arg) = dash_suffix {
          // single dash multi loaded flag

          if let Some(flag) = self.handle_flag(&unhandled_arg.to_string()) {
            active_cli_flags.push(flag);
          } else {
            self.invalid_flag(&unhandled_arg.to_string());
          }

          loop {
            if let Some(val) = chars.next() {
              if let Some(flag) = self.handle_flag(&val.to_string()) {
                active_cli_flags.push(flag);
              } else {
                self.invalid_flag(&val.to_string());
              }
            } else {
              break;
            }
          }
        }
      } else if let Some(selected_arg) = selected_cli_argument {
        if selected_arg.arg && second_argument.is_none() {
          second_argument = Some(arg);
        } else {
          println!("error: more than one argument provided");
          process::exit(1);
        }
      } else {
        if arg == "help" {
          &self.print_help();
          return;
        }
        for cli_argument in self.arguments {
          if cli_argument.name == arg {
            selected_cli_argument = Some(cli_argument);
            break
          } else {
            println!("error: unrecognized argument \"{}\"", arg);
            println!("       use argument \"help\" to list available arguments");
            process::exit(1);
          }
        }
      }
    }
    if let Some(arg) = selected_cli_argument {
      (arg.callback)(&self, &active_cli_flags, &second_argument);
    } else {
      println!("error: no argument given");
      println!("       use \"help\" to list available arguments");
      process::exit(1);
    }
  }
}
