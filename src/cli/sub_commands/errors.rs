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

#[derive(Debug)]
pub enum I18nError {
    NonExistingLanguage(String),
    AlreadyExistingLanguage(String),
    ReadI18nDirectoryError(String),
    ReadLanguageFileError(String),
    NonUtf8LanguageName(String),
    ParseJsonError(String),
    WriteOnFileError(String),
}

impl I18nError {
    /// Returns the error message
    pub fn msg(&self) -> &str {
        match self {
            Self::NonExistingLanguage(s) => s,
            Self::AlreadyExistingLanguage(s) => s,
            Self::ReadI18nDirectoryError(s) => s,
            Self::ReadLanguageFileError(s) => s,
            Self::NonUtf8LanguageName(s) => s,
            Self::ParseJsonError(s) => s,
            Self::WriteOnFileError(s) => s,
        }
    }

    /// Returns the error name
    pub fn name(&self) -> &str {
        match self {
            Self::NonExistingLanguage(_) => "NonExistingLanguage",
            Self::AlreadyExistingLanguage(_) => "AlreadyExistingLanguage",
            Self::ReadI18nDirectoryError(_) => "ReadI18nDirectoryError",
            Self::ReadLanguageFileError(_) => "ReadLanguageFileError",
            Self::NonUtf8LanguageName(_) => "NonUtf8LanguageName",
            Self::ParseJsonError(_) => "ParseJsonError",
            Self::WriteOnFileError(_) => "WriteOnFileError",
        }
    }

    /// Print the error
    pub fn print(&self) {
        eprintln!("{}: {} 🚫", self.name().red(), self.msg());
    }
}

pub type I18nResult<T> = Result<T, I18nError>;
