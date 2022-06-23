use std::process;

pub fn usage() -> ! {
  help_message();
  process::exit(0);
}

pub fn usage_error(msg: &str) -> ! {
  eprintln!("{}\n", error_message(msg));
  help_message();
  std::process::exit(1);
}

pub fn error_exit(msg: &str) -> ! {
  eprintln!("{}", error_message(msg));
  std::process::exit(1);
}

fn error_message(msg: &str) -> String {
  format!("ERROR: {msg}")
}

fn help_message() {
  let usage = concat!(
    "USAGE:\n",
    "    cargo v <version>\n",
    "    cargo v <version> -y\n",
    "    cargo v [options]\n",
    "ARGUMENTS:\n",
    "    version       Can be one of \"patch\", \"minor\", \"major\" or a string like \"v1.2.5\"\n",
    "OPTIONS:\n",
    "    -h, --help    Prints this message.",
    "    -y, --yes     Answer \"yes\" for all questions",
  );

  println!("{usage}");
}
