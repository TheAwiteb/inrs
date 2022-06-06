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

use colored::{ColoredString, Colorize};
use std::fs::write;
use std::path::Path;

/// Create new translation file in i18n directory
pub fn create(i18n_path: &str, lang: &str) {
    let i18n_dir = Path::new(i18n_path);
    let lang_file = i18n_dir.join(lang).with_extension("json");
    let colored_file_path: ColoredString =
        lang_file.to_str().expect("The path it's non-UTF-8").bold();
    if lang_file.exists() {
        eprintln!(
            "{}: '{}' is already exists.",
            "Error".red(),
            colored_file_path
        );
    } else if let Err(err) = write(lang_file, "{}") {
        eprintln!("{}: {}", "Error".red(), err);
    } else {
        println!(
            "Creating '{}' successfully ✅",
            colored_file_path.normal().green()
        );
    };
}
