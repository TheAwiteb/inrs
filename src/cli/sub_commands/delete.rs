use super::utils::Translations;
use colored::Colorize;

/// Delete the translations
pub fn delete(i18n_dir: &str, key: &str) {
    match Translations::new(i18n_dir) {
        Ok(mut translations) => {
            if let Err(err) = translations.delete_translation(key) {
                err.print();
            } else if let Err(err) = translations.export() {
                err.print();
            } else {
                println!("'{}' deleted successfully from languages ✅", key.green(),);
            }
        }
        Err(err) => err.print(),
    }
}
