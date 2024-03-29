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

use colored::Colorize;
use exitcode;
use std::process::{ExitCode, Termination};

/// [`i32`] to [`ExitCode`]
fn to_exit_code(exitcode: i32) -> ExitCode {
    (exitcode as u8).into()
}

#[derive(Debug)]
pub enum I18nError {
    NonExistingLanguage(String),
    NonExistingKey(String),
    AlreadyExistingLanguage(String),
    ReadI18nDirectory(String),
    ReadLanguageFile(String),
    NonUtf8LanguageName(String),
    ParseJson(String),
    WriteOnFile(String),
    ThereIsNoLanguages(String),
    ThereIsNoTranslations(String),
    DeleteFile(String),
}

impl I18nError {
    /// Returns the error message
    pub fn msg(&self) -> &str {
        match self {
            Self::NonExistingLanguage(s) => s,
            Self::NonExistingKey(s) => s,
            Self::AlreadyExistingLanguage(s) => s,
            Self::ReadI18nDirectory(s) => s,
            Self::ReadLanguageFile(s) => s,
            Self::NonUtf8LanguageName(s) => s,
            Self::ParseJson(s) => s,
            Self::WriteOnFile(s) => s,
            Self::ThereIsNoLanguages(s) => s,
            Self::ThereIsNoTranslations(s) => s,
            Self::DeleteFile(s) => s,
        }
    }

    /// Returns the error name
    pub fn name(&self) -> &str {
        match self {
            Self::NonExistingLanguage(_) => "NonExistingLanguage",
            Self::NonExistingKey(_) => "NonExistingKey",
            Self::AlreadyExistingLanguage(_) => "AlreadyExistingLanguage",
            Self::ReadI18nDirectory(_) => "ReadI18nDirectory",
            Self::ReadLanguageFile(_) => "ReadLanguageFile",
            Self::NonUtf8LanguageName(_) => "NonUtf8LanguageName",
            Self::ParseJson(_) => "ParseJson",
            Self::WriteOnFile(_) => "WriteOnFile",
            Self::ThereIsNoLanguages(_) => "ThereIsNoLanguages",
            Self::ThereIsNoTranslations(_) => "ThereIsNoTranslations",
            Self::DeleteFile(_) => "DeleteFile",
        }
    }

    /// Print the error
    pub fn print(&self) {
        eprintln!("{}: {} 🚫", self.name().red(), self.msg());
    }
}

impl Termination for I18nError {
    fn report(self) -> ExitCode {
        match self {
            Self::DeleteFile(_) => to_exit_code(exitcode::NOPERM),
            Self::ReadLanguageFile(_) => to_exit_code(exitcode::NOPERM),
            Self::ReadI18nDirectory(_) => to_exit_code(exitcode::NOPERM),
            Self::WriteOnFile(_) => to_exit_code(exitcode::NOPERM),
            Self::AlreadyExistingLanguage(_) => to_exit_code(exitcode::CANTCREAT),
            _ => {
                // NonExistingLanguage
                // NonExistingKey
                // ThereIsNoLanguages
                // ThereIsNoTranslations
                // NonUtf8LanguageName
                // ParseJson
                ExitCode::from(1)
            }
        }
    }
}

pub type I18nResult<T> = Result<T, I18nError>;
