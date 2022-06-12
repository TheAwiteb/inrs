use super::utils::{Translation, Translations};
use colored::Colorize;

/// Add and update translations
pub fn update(i18n_dir: &str, translation: Translation) {
    match Translations::new(i18n_dir) {
        Ok(mut translations) => {
            if let Err(err) = translations.update_translation(&translation) {
                err.print()
            } else if let Err(err) = translations.export() {
                err.print()
            } else {
                println!(
                    "The translation of the '{}' key to '{}' has been successfully updated in '{}'",
                    translation.key.green(),
                    translation.translation.green(),
                    translation.lang_name.green()
                );
            }
        }
        Err(err) => err.print(),
    };
}
