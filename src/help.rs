use std::process;

pub fn usage() -> ! {
  help_message();
  process::exit(0);
}

pub fn usage_error(msg: &str) -> ! {
  help_message();
  error_exit(msg);
}

pub fn error_exit(msg: &str) -> ! {
  eprintln!("ERROR: {msg}");
  std::process::exit(1);
}

fn help_message() {
  let usage = concat!(
    "USAGE:\n",
    "    cargo v <version>\n",
    "    cargo v [options]\n",
    "ARGUMENTS:\n",
    "    version       Can be one of \"patch\", \"minor\", \"major\" or a string like \"v1.2.5\"\n",
    "OPTIONS:\n",
    "    -h, --help    Prints this message."
  );

  println!("{usage}");
}
