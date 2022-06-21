# Welcome to cargo-v 👋

[![Documentation](https://img.shields.io/badge/documentation-yes-brightgreen.svg)](https://github.com/fdaciuk/cargo-v#readme)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
[![Twitter: fdaciuk](https://img.shields.io/twitter/follow/fdaciuk.svg?style=social)](https://twitter.com/fdaciuk)

> An easy way to update the version of your package

### 🏠 [Homepage](https://github.com/fdaciuk/cargo-v)

## Install

```sh
cargo install cargo-v
```

This CLI is intended to update the version of your package using the [SemVer](https://semver.org).

## Important note

Before using this CLI, make sure you:
  - run the `cargo build` command to ensure your package doesn't have any errors;
  - have committed all important files (including `Cargo.toml` and `Cargo.lock`, that will be "git added" automatically by the CLI);

## Usage

You can update the version of your project using the command:

```sh
cargo v <version>
```

When `<version>` can be on of `patch`, `minor`, `major` or a string like `v1.1.0` or just `1.1.0`.

The above command will do:
  - update the string version of your package from `Cargo.toml`;
  - update the string version of your package from `Cargo.lock`;
  - create a git commit with new version;
  - and create a git tag with new version.

To see all possible options, just run `cargo v --help`.

## Author

👤 **Fernando Daciuk**

* Website: https://daciuk.dev
* Twitter: [@fdaciuk](https://twitter.com/fdaciuk)
* Github: [@fdaciuk](https://github.com/fdaciuk)
* LinkedIn: [@fdaciuk](https://linkedin.com/in/fdaciuk)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!

Feel free to check [issues page](https://github.com/fdaciuk/cargo-v/issues). 

## Show your support

Give a ⭐️ if this project helped you!


## 📝 License

Copyright © 2022 [Fernando Daciuk](https://github.com/fdaciuk).

This project is **MIT** licensed.

***

_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_
