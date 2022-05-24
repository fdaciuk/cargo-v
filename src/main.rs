use std::{env, fs};

fn main() {
  let args: Vec<String> = env::args().collect();
  dbg!(args);

  let file = fs::read_to_string("./Cargo.toml");
  match file {
    Ok(data) => println!("{data}"),
    Err(err) => println!("{err}"),
  };
}
