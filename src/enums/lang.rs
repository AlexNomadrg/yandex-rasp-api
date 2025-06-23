#[derive(Debug, Default)]
pub enum Lang {
    #[default]
    RU,
    UA,
}

impl ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            Lang::RU => "ru_RU".to_owned(),
            Lang::UA => "ua_UA".to_owned(),
        }
    }
}
