#[derive(Debug)]
pub struct CliConfig {
  pub version: Option<String>,
  pub help: bool,
  pub yes_to_all: bool,
}

impl CliConfig {
  pub fn from_iter(args: &mut impl Iterator<Item = String>) -> Self {
    let mut config = Self {
      version: None,
      help: false,
      yes_to_all: false,
    };

    for arg in args.by_ref() {
      match arg.as_str() {
        "-h" | "--help" => config.help = true,
        "-y" | "--yes" => config.yes_to_all = true,
        other => config.version = Some(String::from(other)),
      };
    }

    config
  }
}
