// Simple CLI to (add, delete, update, create) i18n translation file
//     Copyright (C) 2020-2022  TheAwiteb
//     https://github.com/TheAwiteb/inrs
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

use super::utils::Translations;
use colored::Colorize;

/// Delete the translations
pub fn delete_key(i18n_dir: &str, key: &str) {
    match Translations::new(i18n_dir) {
        Ok(mut translations) => {
            if let Err(err) = translations.delete_translation(key) {
                err.print();
            } else if let Err(err) = translations.export() {
                err.print();
            } else {
                println!("'{}' deleted successfully from languages ✅", key.green(),);
            }
        }
        Err(err) => err.print(),
    }
}

/// Delete the language
pub fn delete_language(i18n_dir: &str, lang_name: &str) {
    match Translations::new(i18n_dir) {
        Ok(mut translations) => {
            if let Err(err) = translations.delete_language(lang_name) {
                err.print();
            } else if let Err(err) = translations.export() {
                err.print();
            } else {
                println!("'{}' deleted successfully ✅", lang_name.green(),);
            }
        }
        Err(err) => err.print(),
    }
}
