use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub enum Lang {
    En,
    Fr,
    Mg,
}

pub struct I18n {
    pub lang: Lang,
}

impl I18n {
    pub fn translate(&self, key: &str) -> String {
        match self.lang {
            Lang::En => match key {
                "back" => "Back".to_string(),
                "home" => "Home".to_string(),
                "no_input" => "No input".to_string(),
                "good_bye" => "Good bye".to_string(),
                _ => key.to_string(),
            },
            Lang::Fr => match key {
                "back" => "Retour".to_string(),
                "home" => "Accueil".to_string(),
                "no_input" => "Aucune saisie".to_string(),
                "good_bye" => "Au revoir".to_string(),
                _ => key.to_string(),
            },
            Lang::Mg => match key {
                "back" => "Hiverina".to_string(),
                "home" => "Fandraisana".to_string(),
                "no_input" => "Tsy misy".to_string(),
                "good_bye" => "Veloma".to_string(),
                _ => key.to_string(),
            },
        }
    }
}

static I18N: OnceLock<I18n> = OnceLock::new();

pub fn init_i18n(lang: Lang) {
    I18N.set(I18n { lang }).ok();
}

pub fn t(key: &str) -> String {
    I18N.get()
        .map(|i| i.translate(key))
        .unwrap_or(key.to_string())
}
