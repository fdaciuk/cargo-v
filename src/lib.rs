pub enum VersionLabel {
  Patch,
  Minor,
  Major,
}

pub fn update_version_by_label(
  cargo_toml_content: String,
  label: VersionLabel,
) -> String {
  let current_version = get_version(&cargo_toml_content);
  let (major, minor, patch) = current_version;
  let new_version = match label {
    VersionLabel::Major => format!("{}.0.0", major + 1),
    VersionLabel::Minor => format!("{}.{}.0", major, minor + 1),
    VersionLabel::Patch => format!("{}.{}.{}", major, minor, patch + 1),
  };

  cargo_toml_content
    .replace(&tuple_version_to_string(current_version), &new_version)
}

pub fn update_version(cargo_toml_content: String, version: String) -> String {
  let current_version_tuple = get_version(&cargo_toml_content);
  let current_version = tuple_version_to_string(current_version_tuple);
  cargo_toml_content.replace(&current_version, &version)
}

fn tuple_version_to_string(tuple_version: (u32, u32, u32)) -> String {
  format!(
    "{}.{}.{}",
    tuple_version.0, tuple_version.1, tuple_version.2,
  )
}

fn get_version(cargo_toml_content: &str) -> (u32, u32, u32) {
  let version = cargo_toml_content
    .lines()
    .filter(|line| line.contains("version"))
    .map(|line| {
      line
        .replace("version", "")
        .replace('=', "")
        .replace('"', "")
        .trim()
        .to_string()
    })
    .next();

  let version =
    version.expect("Your Cargo.toml file does not have a \"version\" entry");

  let mut version_split = version.split('.');
  let major = version_split.next().unwrap().parse().unwrap();
  let minor = version_split.next().unwrap().parse().unwrap();
  let patch = version_split.next().unwrap().parse().unwrap();
  (major, minor, patch)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn update_patch_version() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n");
    let new_version = String::from("0.0.2");
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n";

    assert_eq!(update_version(cargo_toml, new_version), expected);
  }

  #[test]
  fn update_minor_version() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n");
    let new_version = String::from("0.1.0");
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n";

    assert_eq!(update_version(cargo_toml, new_version), expected);
  }

  #[test]
  #[should_panic(
    expected = "Your Cargo.toml file does not have a \"version\" entry"
  )]
  fn should_fail_if_cargo_toml_does_not_have_version() {
    let cargo_toml = String::from("[package]\n name = \"cargo-v\"\n");
    let new_version = String::from("0.1.0");
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n";

    assert_eq!(update_version(cargo_toml, new_version), expected);
  }

  #[test]
  fn update_version_by_patch_label() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.1\"\n");
    let new_version = VersionLabel::Patch;
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n";

    assert_eq!(update_version_by_label(cargo_toml, new_version), expected);
  }

  #[test]
  fn update_version_by_minor_label() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.0.2\"\n");
    let new_version = VersionLabel::Minor;
    let expected = "[package]\n name = \"cargo-v\"\n version = \"0.1.0\"\n";

    assert_eq!(update_version_by_label(cargo_toml, new_version), expected);
  }

  #[test]
  fn update_version_by_major_label() {
    let cargo_toml =
      String::from("[package]\n name = \"cargo-v\"\n version = \"0.1.2\"\n");
    let new_version = VersionLabel::Major;
    let expected = "[package]\n name = \"cargo-v\"\n version = \"1.0.0\"\n";

    assert_eq!(update_version_by_label(cargo_toml, new_version), expected);
  }
}
