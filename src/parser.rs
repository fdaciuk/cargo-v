#![allow(dead_code)]
#![allow(unused_variables)]

pub fn parse_toml(cargo_toml: &str) -> &str {
  let mut package_lines: Vec<&str> = Vec::new();

  let lines: Vec<_> = cargo_toml.lines().collect();
  let mut yank = false;

  for line in lines {
    // setup
    if line.starts_with("[") {
      yank = line.starts_with("[package]")
    }

    // copy
    if yank {
      if line.starts_with("version") {
        package_lines.push(line);
      }

      if line.starts_with("name") {
        package_lines.push(line);
      }
    }
    println!("Start-------------------");
    println!("{line}");
    println!("-------------------End");
  }
  println!("package lines:");
  dbg!(package_lines);
  ""
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn useless() {
    let cargo_toml = "\
[dependencies]
tokio = { version = \"1.1.1\" }

[package]
name = \"cargo-v\"
version = \"0.1.6\"
edition = \"2021\"
description = \"Update the version of your package easily\"
license = \"MIT\"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
[dependencies.dev]
other = { version = \"1.1.8\" }

    ";
    let actual = parse_toml(cargo_toml);
    let expected = "";
    assert_eq!(actual, expected);
  }
}
