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

use super::errors::I18nError;
use super::utils::Translations;
use colored::Colorize;

/// Create new translation file in i18n directory
pub fn create(i18n_path: &str, lang: &str) -> Option<I18nError> {
    match Translations::new(i18n_path) {
        Ok(mut translation) => {
            if let Err(err) = translation.add_language(lang) {
                err.print();
                Some(err)
            } else if let Err(err) = translation.export() {
                err.print();
                Some(err)
            } else {
                println!("Creating '{}' language successfully ✅", lang.green());
                None
            }
        }
        Err(err) => {
            err.print();
            Some(err)
        }
    }
}
