#[derive(Debug)]
pub enum I18nError {
    NonExistingLanguage(String),
    AlreadyExistingLanguage(String),
    ReadI18nDirectoryError(String),
    ReadLanguageFileError(String),
    NonUtf8LanguageName(String),
    ParseJsonError(String),
    WriteOnFileError(String),
    AlreadyExistingKey(String),
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
            Self::AlreadyExistingKey(s) => s,
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
            Self::AlreadyExistingKey(_) => "AlreadyExistingKey",
        }
    }
}

pub type I18nResult<T> = Result<T, I18nError>;
