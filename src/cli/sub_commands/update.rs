use super::utils::{Translation, Translations};
use colored::Colorize;

pub fn update(i18n_dir: &str, translation: Translation) {
    match Translations::new(i18n_dir) {
        Ok(mut translations) => {
            if let Err(err) = translations.update_translation(&translation) {
                eprintln!("{}: {} 🚫", err.name().red(), err.msg())
            } else if let Err(err) = translations.export() {
                eprintln!("{}: {} 🚫", err.name().red(), err.msg())
            } else {
                println!(
                    "Updates {} to {} successfully in {}",
                    translation.key.green(),
                    translation.translation.green(),
                    translation.lang_name.green()
                );
            }
        }
        Err(err) => eprintln!("{}: {} 🚫", err.name().red(), err.msg()),
    };
}
