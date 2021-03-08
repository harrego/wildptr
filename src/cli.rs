use substring::Substring;
use std::process;

pub struct Argument {
  pub name: String,
  pub description: String,

  pub callback: fn(),
}

pub struct Flag {
  pub flags: Vec<String>,
  pub description: String,
}

pub struct CLI {
  pub app_name: String,
  pub app_version: String,
  pub app_description: String,

  pub arguments: Vec<Argument>,
  pub flags: Vec<Flag>,
}
impl CLI {
  pub fn print_help(&self) {
    println!("{} v{}", self.app_name, self.app_version);
    println!("{}", self.app_description);

    print!("\nusage: wildptr ");
    for flag in &self.flags {
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
    for arg in &self.arguments {
      println!("    {} - {}", arg.name, arg.description);
    }
  }

  fn handle_arg(&self, arg: &String) {
    println!("handling arg: {}", arg);
  }

  pub fn parse(&self) {
    let mut selected_cli_argument: Option<&Argument> = None;

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
          self.handle_arg(&arg.substring(2, arg.chars().count()).to_string());
        } else if let Some(unhandled_arg) = dash_suffix {
          // single dash multi loaded flag
          self.handle_arg(&unhandled_arg.to_string());
          loop {
            match chars.next() {
              Some(val) => self.handle_arg(&val.to_string()),
              None => break,
            }
          }
        }
      } else if !selected_cli_argument.is_none() {
        println!("error: more than one argument provided");
      } else {
        for cli_argument in &self.arguments {
          if cli_argument.name == arg {
            selected_cli_argument = Some(cli_argument);
            break
          } else {
            println!("error: unrecognized argument \"{}\"", arg);
            println!("       use -h to list available arguments");
            process::exit(1);
          }
        }
      }
    }
    if let Some(arg) = selected_cli_argument {
      (arg.callback)();
    } else {
      println!("error: no argument given");
      println!("       use -h to list available arguments");
    }
  }
}
