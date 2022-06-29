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
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Row, Table};
use serde_json;
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, read_dir, read_to_string, write};
use std::path::{Path, PathBuf};
use std::string::ToString;

/// Returns all languages in i18n directory
pub fn list_languages(i18n_dir: &str) -> I18nResult<Vec<I18nResult<I18nResult<String>>>> {
    Ok(read_dir(i18n_dir)
        .map_err(|err| I18nError::ReadI18nDirectory(format!("'{i18n_dir}', {err}")))?
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
                .map_err(|err| I18nError::ReadLanguageFile(err.to_string()))
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
                    .map_err(|err| I18nError::ReadLanguageFile(format!("'{lang_name}', {err}")))?,
            )
            .map_err(|err| I18nError::ParseJson(format!("'{lang_name}', {err}")))?;
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

    /// Make table from language translations
    fn to_table(&self, width: u16) -> String {
        let mut idx: u8 = 0;
        let mut counter: u8 = 0;
        let colors: [Color; 3] = [Color::DarkYellow, Color::DarkCyan, Color::DarkBlue];
        let mut table = Table::new();
        table
            .set_header([
                Cell::new("Key").set_alignment(CellAlignment::Center),
                Cell::new("Translation").set_alignment(CellAlignment::Center),
            ])
            .load_preset("     ──  ──        ")
            .set_width(width)
            .set_content_arrangement(ContentArrangement::DynamicFullWidth);

        self.translations.iter().for_each(|(key, translation)| {
            table.add_row(Row::from([
                Cell::new(key)
                    .fg(colors[idx as usize])
                    .set_alignment(CellAlignment::Center),
                Cell::new(translation).set_alignment(CellAlignment::Center),
            ]));
            counter += 1;
            if counter % 3 == 0 {
                idx = (idx + 1) % (colors.len() as u8);
                counter %= 3;
            }
        });
        table.to_string()
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

    /// Return table of translations for specific language
    pub fn to_table(&self, lang_name: &str, width: u16) -> I18nResult<String> {
        if !self.languages.is_empty() {
            if let Some(lang) = self
                .languages
                .iter()
                .find(|lang| lang.lang_name == lang_name)
            {
                if !lang.translations.is_empty() {
                    Ok(lang.to_table(width))
                } else {
                    Err(I18nError::ThereIsNoTranslations(format!(
                        "There is no translations in `{lang_name}`"
                    )))
                }
            } else {
                Err(I18nError::NonExistingLanguage(format!(
                    "There is no language named '{lang_name}'",
                )))
            }
        } else {
            Err(I18nError::ThereIsNoLanguages(format!(
                "There is no languages in '{}'",
                self.i18n_dir
            )))
        }
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
        if !self.languages.is_empty() {
            if self
                .languages
                .iter()
                .any(|lang| lang.translations.contains_key(key))
            {
                // delete the key from the translations
                self.languages.iter_mut().for_each(|lang| {
                    lang.translations.remove(key);
                });
                Ok(())
            } else {
                Err(I18nError::NonExistingKey(format!(
                    "There is no key named '{key}' in the translations"
                )))
            }
        } else {
            Err(I18nError::ThereIsNoLanguages(format!(
                "There is no languages in '{}'",
                self.i18n_dir
            )))
        }
    }

    /// Delete language
    pub fn delete_language(&mut self, lang_name: &str) -> I18nResult<()> {
        if !self.languages.is_empty() {
            if let Some((idx, _lang)) = self
                .languages
                .iter()
                .enumerate()
                .find(|(_idx, lang)| lang.lang_name == lang_name)
            {
                self.languages.remove(idx);
                fs::remove_file(
                    Path::new(self.i18n_dir.as_str())
                        .join(lang_name)
                        .with_extension("json"),
                )
                .map_err(|err| I18nError::DeleteFile(err.to_string()))
            } else {
                Err(I18nError::NonExistingLanguage(format!(
                    "There is no language named '{lang_name}'",
                )))
            }
        } else {
            Err(I18nError::ThereIsNoLanguages(format!(
                "There is no languages in '{}'",
                self.i18n_dir
            )))
        }
    }

    /// Add new language
    pub fn add_language(&mut self, lang_name: &str) -> I18nResult<&Language> {
        let lang_file = Path::new(&self.i18n_dir)
            .join(lang_name)
            .with_extension("json");
        if !lang_file.exists() {
            if let Err(err) = write(lang_file, "{}") {
                Err(I18nError::WriteOnFile(format!("'{lang_name}', {err}")))
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
                    I18nError::ParseJson(format!("'{}', {}", lang.lang_name, err))
                })?,
            )
            .map_err(|err| I18nError::WriteOnFile(format!("'{}', {}", lang.lang_name, err)))?;
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
