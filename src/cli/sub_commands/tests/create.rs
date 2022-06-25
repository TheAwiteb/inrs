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

use super::utils::{create_i18n, delete_i18n, list_i18n, to_json_list};
use crate::cli::sub_commands::create;
use std::io::Result as IOResult;

#[test]
fn test_create() -> IOResult<()> {
    let i18n_path = "i18n-c";
    create_i18n(i18n_path)?;
    create(i18n_path, "en_US");
    assert_eq!(list_i18n(i18n_path)?, to_json_list(vec!["en_US"]));
    create(i18n_path, "ar_SA");
    assert_eq!(list_i18n(i18n_path)?, to_json_list(vec!["en_US", "ar_SA"]));
    delete_i18n(i18n_path)?;
    Ok(())
}
