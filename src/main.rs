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

mod cli;

use cli::sub_commands::{
    create, delete_key, delete_language, update, DeleteSubCommands, Subcommands,
};

fn main() {
    let app = cli::parse();
    match app.action {
        Subcommands::Create { lang } => create(app.path.as_str(), lang.as_str()),
        Subcommands::Update { lang, key, trans } => update(
            app.path.as_str(),
            (lang.as_str(), key.as_str(), trans.as_str()).into(),
        ),
        Subcommands::Delete { action } => match action {
            DeleteSubCommands::Lang { lang } => delete_language(app.path.as_str(), lang.as_str()),
            DeleteSubCommands::Trans { key } => delete_key(app.path.as_str(), key.as_str()),
        },
    };
}
