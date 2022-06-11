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

use colored::Colorize;
use std::fs::read_dir;
use std::path::Path;

type VResult = Result<(), String>;

/// validate i18n path
pub fn validate_i18n_path(path: &str) -> VResult {
    let i18n_dir = Path::new(path);
    if !i18n_dir.exists() {
        Err("There is no directory with this name 🚫".to_owned())
    } else if !i18n_dir.is_dir() {
        Err("It should be a directory 📂".to_owned())
    } else if let Ok(entries) = read_dir(i18n_dir) {
        // Check if all files are json file
        for entry in entries {
            match entry {
                Ok(entry) => {
                    if let Ok(file_type) = entry.file_type() {
                        if !file_type.is_dir() {
                            if let Some(str_entry) = entry.file_name().to_str() {
                                if str_entry.strip_suffix(".json").is_some() {
                                    // Accept ✔️
                                } else {
                                    return Err(
                                            format!("'{str_entry}' is not a json file ( translation file should be json file) 📁")
                                        );
                                }
                            } else {
                                return Err(format!(
                                    "Invalid json file name '{:?}' 🚫",
                                    entry.path()
                                ));
                            }
                        } else {
                            return Err(format!(
                                "i18n directory should not contain directory but {:?} is directory 🚫",
                                entry.path()
                            ));
                        }
                    } else {
                        return Err(format!("Cannot get file type of {:?} 🚫", entry.path()));
                    }
                }
                Err(err) => return Err(format!("Cannot get entry, {err}")),
            }
        }
        Ok(())
    } else {
        Err("Cannot read the i18n directory 📁".to_owned())
    }
}

/// Validate the name of language
pub fn validate_lang_name(lang_name: &str) -> VResult {
    if lang_name.contains('.') {
        return Err(format!(
            "'{}' is invalid name, did you means {}",
            lang_name.red(),
            lang_name.split('.').next().unwrap().green()
        ));
    }
    Ok(())
}
