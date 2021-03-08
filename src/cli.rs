pub struct Argument {
  pub name: String,
  pub description: String,

  pub callback: fn(String),
}

pub struct Flag {
  pub flags: Vec<String>,
  pub description: String,
}

pub struct CLI {
  pub app_name: String,
  pub app_version: String,

  pub arguments: Vec<Argument>,
  pub flags: Vec<Flag>,
}
impl CLI {
  pub fn print_help(&self) {
    println!("{} v{}", self.app_name, self.app_version);
    println!("(c) harrego");

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
}

