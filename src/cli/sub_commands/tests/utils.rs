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

use crate::cli::sub_commands::utils::Translations;
use std::collections::BTreeMap;
use std::fs::{create_dir, read_dir, remove_dir_all};
use std::io::Result as IOResult;

pub fn to_json_list(names: Vec<&str>) -> Vec<String> {
    let mut names: Vec<String> = names
        .iter()
        .map(|name| name.to_string() + ".json")
        .collect();
    names.sort();
    names
}

pub fn create_i18n(i18n_path: &str) -> IOResult<()> {
    create_dir(i18n_path)
}

pub fn delete_i18n(i18n_path: &str) -> IOResult<()> {
    remove_dir_all(i18n_path)
}

pub fn list_i18n(i18n_path: &str) -> IOResult<Vec<String>> {
    let mut entrys = Vec::new();
    for entry in read_dir(i18n_path)? {
        entrys.push(
            entry?
                .file_name()
                .to_str()
                .expect("Cannot convert file name to str")
                .to_owned(),
        )
    }
    Ok(entrys)
}

pub fn list_translations(i18n_path: &str, lang_name: &str) -> BTreeMap<String, String> {
    let trans = Translations::new(i18n_path).unwrap();
    trans
        .languages
        .iter()
        .find(|lang| lang.lang_name == lang_name)
        .unwrap()
        .translations
        .clone()
}
