// Simple CLI to (add, delete, update, create) i18n translation file
//     Copyright (C) 2020-2022  TheAwiteb
//     https://github.com/TheAwiteb/i18nrs
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::sub_commands::Subcommands;
use super::validator::validate_i18n_path;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, long_about = None)]
/// Simple CLI to (add, delete, update, create) i18n translation file 🔤 🦀
pub struct App {
    /// Path of i18n directory 📂
    #[clap(short, long, validator = validate_i18n_path)]
    // FEATURE/TODO: Make path optional and search in current directory or config file
    pub path: String,
    #[clap(subcommand)]
    pub action: Subcommands,
}

/// Parse the args
pub fn parse() -> App {
    App::parse()
}
