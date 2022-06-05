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

use std::path::Path;

type VResult = Result<(), String>;

/// validate i18n path
pub fn validate_i18n_path(path: &str) -> VResult {
    let i18n_dir = Path::new(path);
    if !i18n_dir.exists() {
        Err("There is no directory with this name 🚫".to_owned())
    } else if !i18n_dir.is_dir() {
        Err("It should be a directory 📂".to_owned())
    } else {
        Ok(())
    }
}
