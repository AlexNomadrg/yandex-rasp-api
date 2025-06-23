use serde::{Deserialize, Serialize, Serializer};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// Yandex API Docs: https://yandex.ru/dev/rasp/doc/ru/concepts/coding-system
pub enum CodeSystem {
    #[default]
    Yandex,
    Esr,
    Iata,
    Sirena,
    Express3,
    Undefined(String),
    All,
}

impl ToString for CodeSystem {
    fn to_string(&self) -> String {
        match self {
            CodeSystem::Yandex => String::from("yandex"),
            CodeSystem::All => String::from("all"),
            CodeSystem::Esr => String::from("esr"),
            CodeSystem::Iata => String::from("iata"),
            CodeSystem::Sirena => String::from("sirena"),
            CodeSystem::Express3 => String::from("express"),
            CodeSystem::Undefined(s) => s.to_owned(),
        }
    }
}

impl From<String> for CodeSystem {
    fn from(value: String) -> Self {
        match value.to_string().as_str() {
            "all" => CodeSystem::All,
            "yandex" => CodeSystem::Yandex,
            "esr" => CodeSystem::Esr,
            "iata" => CodeSystem::Iata,
            "sirena" => CodeSystem::Sirena,
            "express" => CodeSystem::Express3,
            _ => CodeSystem::Undefined(value),
        }
    }
}

impl<'de> Deserialize<'de> for CodeSystem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(CodeSystem::from(s))
    }
}

impl Serialize for CodeSystem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}
