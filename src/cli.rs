use substring::Substring;
use std::process;

pub struct Argument {
  pub name: &'static str,
  pub description: &'static str,

  pub callback: fn(&CLI, &Vec<&Flag>),
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
      println!("    {} - {}", arg.name, arg.description);
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
          // method is safe for unicode, are chars() byte or ascii
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
      } else if !selected_cli_argument.is_none() {
        println!("error: more than one argument provided");
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
      (arg.callback)(&self, &active_cli_flags);
    } else {
      println!("error: no argument given");
      println!("       use \"help\" to list available arguments");
      process::exit(1);
    }
  }
}
