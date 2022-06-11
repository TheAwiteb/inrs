use super::errors::{I18nError, I18nResult};
use serde_json;
use std::collections::BTreeMap;
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

#[derive(Debug, Clone)]
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
        let mut translations = Self {
            i18n_dir: i18n_dir.to_string(),
            languages,
        };
        translations.fill_missing_keys();
        Ok(translations)
    }

    /// Fill the missing keys for each language
    fn fill_missing_keys(&mut self) {
        let cloned_language = self.languages.clone();
        for main_lang in self.languages.iter_mut() {
            for lang in cloned_language.iter() {
                for key in lang.translations.keys() {
                    if !main_lang.translations.contains_key(key) {
                        main_lang
                            .translations
                            .insert(key.to_string(), String::new());
                    }
                }
            }
        }
    }

    /// Add new translation
    pub fn add_translation(&mut self, translation: Translation) -> I18nResult<()> {
        // Check if language already exists
        if let Some(lang) = self // &mut Language
            .languages
            .iter_mut()
            .find(|lang| lang.lang_name == translation.lang_name)
        {
            // Check if key not exists
            if !lang.translations.contains_key(translation.key) {
                lang.translations.insert(
                    translation.key.to_string(),
                    translation.translation.to_string(),
                );
                Ok(())
            } else {
                Err(I18nError::AlreadyExistingKey(format!(
                    "'{}' is already exists",
                    translation.key
                )))
            }
        } else {
            Err(I18nError::NonExistingLanguage(format!(
                "'{}' is not exists",
                translation.lang_name
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
                Err(I18nError::WriteOnFileError(err.to_string()))
            } else {
                let language = Language::new(&self.i18n_dir, lang_name)?;
                self.languages.push(language);
                self.fill_missing_keys();
                Ok(self.languages.last().unwrap())
            }
        } else {
            Err(I18nError::AlreadyExistingLanguage(format!(
                "'{lang_name}' is already exists"
            )))
        }
    }

    /// Exports translations to files
    pub fn export(&self) -> I18nResult<()> {
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
