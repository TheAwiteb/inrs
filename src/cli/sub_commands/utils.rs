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

use super::errors::{I18nError, I18nResult};
use serde_json;
use std::collections::{BTreeMap, HashSet};
use std::fs::{read_dir, read_to_string, write};
use std::path::{Path, PathBuf};

/// Returns all languages in i18n directory
pub fn list_languages(i18n_dir: &str) -> I18nResult<Vec<I18nResult<I18nResult<String>>>> {
    Ok(read_dir(i18n_dir)
        .map_err(|err| I18nError::ReadI18nDirectoryError(err.to_string()))?
        .map(|entry| {
            entry
                .map(|e| {
                    e.file_name()
                        .to_str()
                        .ok_or_else(|| {
                            I18nError::NonUtf8LanguageName(format!("'{:?}' is non-ut8", e))
                        })
                        .map(|file_name: &str| file_name.replace(".json", ""))
                })
                .map_err(|err| I18nError::ReadLanguageFileError(err.to_string()))
        })
        .collect())
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Language {
    pub lang_name: String,
    pub lang_file: PathBuf,
    pub translations: BTreeMap<String, String>,
}

#[derive(Debug)]
pub struct Translations {
    pub i18n_dir: String,
    pub languages: Vec<Language>,
}

pub struct Translation<'a> {
    pub lang_name: &'a str,
    pub key: &'a str,
    pub translation: &'a str,
}

impl Language {
    /// Create a new [`Language`] instance
    pub fn new(i18n_dir: &str, lang_name: &str) -> I18nResult<Self> {
        let lang_file = Path::new(i18n_dir).join(lang_name).with_extension("json");
        if lang_file.exists() {
            let translations: BTreeMap<String, String> = serde_json::from_str(
                &read_to_string(&lang_file)
                    .map_err(|err| I18nError::ReadLanguageFileError(err.to_string()))?,
            )
            .map_err(|err| I18nError::ParseJsonError(format!("'{lang_name}', {err}")))?;
            Ok(Self {
                lang_name: lang_name.into(),
                lang_file,
                translations,
            })
        } else {
            Err(I18nError::NonExistingLanguage(format!(
                "There is not language named '{lang_name}'"
            )))
        }
    }
}

impl Translations {
    /// Create a new ['Translations'] instance
    pub fn new(i18n_dir: &str) -> I18nResult<Self> {
        let mut languages: Vec<Language> = Vec::new();
        for lang in list_languages(i18n_dir)? {
            languages.push(Language::new(i18n_dir, &lang??)?)
        }
        Ok(Self {
            i18n_dir: i18n_dir.to_string(),
            languages,
        })
    }

    /// Fill the missing keys for each language
    fn fill_missing_keys(&mut self) {
        self.languages
            .clone()
            .iter()
            .flat_map(|lang| lang.translations.keys())
            .collect::<HashSet<&String>>()
            .iter()
            .for_each(|key| {
                self.languages
                    .iter_mut()
                    .filter(|lang| !lang.translations.contains_key(*key))
                    .for_each(|lang| {
                        lang.translations.insert(key.to_string(), String::new());
                    })
            })
    }

    /// Add/Update translation
    pub fn update_translation(&mut self, translation: &Translation) -> I18nResult<()> {
        // Check if language already exists
        if let Some(lang) = self
            .languages
            .iter_mut()
            .find(|lang| lang.lang_name == translation.lang_name)
        {
            lang.translations.insert(
                translation.key.to_string(),
                translation.translation.to_string(),
            );
            Ok(())
        } else {
            Err(I18nError::NonExistingLanguage(format!(
                "'{}' is not exists",
                translation.lang_name
            )))
        }
    }

    /// Delete translation
    pub fn delete_translation(&mut self, key: &str) -> I18nResult<()> {
        // delete the key from the translations
        self.languages.iter_mut().for_each(|lang| {
            lang.translations.remove(key);
        });
        Ok(())
    }

    /// Add new language
    pub fn add_language(&mut self, lang_name: &str) -> I18nResult<&Language> {
        let lang_file = Path::new(&self.i18n_dir)
            .join(lang_name)
            .with_extension("json");
        if !lang_file.exists() {
            if let Err(err) = write(lang_file, "{}") {
                Err(I18nError::WriteOnFileError(err.to_string()))
            } else {
                let language = Language::new(&self.i18n_dir, lang_name)?;
                self.languages.push(language);
                Ok(self.languages.last().unwrap())
            }
        } else {
            Err(I18nError::AlreadyExistingLanguage(format!(
                "'{lang_name}' is already exists"
            )))
        }
    }

    /// Exports translations to files
    pub fn export(&mut self) -> I18nResult<()> {
        self.fill_missing_keys();
        for lang in self.languages.iter() {
            write(
                &lang.lang_file,
                serde_json::to_string_pretty(&lang.translations).map_err(|err| {
                    I18nError::ParseJsonError(format!("'{}', {}", lang.lang_name, err))
                })?,
            )
            .map_err(|err| I18nError::WriteOnFileError(format!("'{}', {}", lang.lang_name, err)))?;
        }
        Ok(())
    }
}

impl<'a> From<(&'a str, &'a str, &'a str)> for Translation<'a> {
    fn from((lang_name, key, translation): (&'a str, &'a str, &'a str)) -> Self {
        Self {
            lang_name,
            key,
            translation,
        }
    }
}
