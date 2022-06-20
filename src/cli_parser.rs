use crate::help;

pub enum Operation {
  Help,
  Version(String),
}

pub fn parse_cli(argument: String) -> Operation {
  if argument.starts_with('-') || argument.starts_with("--") {
    let arg_name = argument
      .chars()
      .filter(|char| char != &'-')
      .collect::<String>();

    operation_of_string(&arg_name)
  } else {
    Operation::Version(argument)
  }
}

fn operation_of_string(arg: &str) -> Operation {
  match arg {
    "h" | "help" => Operation::Help,
    _ => {
      eprintln!();
      help::usage_error(&format!("invalid argument \"{arg}\""));
    }
  }
}
