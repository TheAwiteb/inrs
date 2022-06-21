<div align="center">

# Inrs
Simple CLI to (add, delete, update, create) i18n translation file

</div>

```
    Copyright (C) 2020-2022  TheAwiteb
    https://github.com/TheAwiteb/inrs

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

## Requirements
 * [Rust](https://www.rust-lang.org/)

## Features
- Write in Rust 🦀.
- Sort translations by key (in json file).
- Add the missing keys with an empty translation (in json file).
- Create new language with `create` command.
- Add/Update translation on specified language with `update` command.
- Delete translation by key in all languages with `delete` command.

## Disadvantages
- Only support json files

## Install
### With Cargo
```bash
cargo install inrs
inrs --version
```
### From source
```bash
# Clone the repo
git clone https://github.com/theawiteb/inrs.git
# Change directory to it
cd inrs
# Build it with cargo
cargo build --release
# Move the binary to `/usr/bin` (Unix like system) (need permission to move in `/usr/bin`)
# You can change binary directory to `~/.cargo/bin` if its exists and its in `$PATH`
sudo mv ./target/release/inrs /usr/bin/inrs
# Print the version
inrs --version
```

## Using
```
USAGE:
    inrs --path <PATH> <SUBCOMMAND>

OPTIONS:
    -h, --help           Print help information
    -p, --path <PATH>    Path of i18n directory 📂
    -V, --version        Print version information

SUBCOMMANDS:
    create    Create new language file 🔤
    delete    Delete translations by key 🚧
    help      Print this message or the help of the given subcommand(s)
    update    Add/Update translation 🆕
```

## Images

|Left|Right|
|:-:|:-:|
|![](https://i.suar.me/yWA4n/l)|![](https://i.suar.me/AW1le/l)|
|![](https://i.suar.me/Kgroy/l)|![](https://i.suar.me/mw0BN/l)|


## License
GNU General Public License version 3 of the license for more see <https://www.gnu.org/licenses/>
