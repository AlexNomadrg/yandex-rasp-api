use serde::{Deserialize, Serialize, Serializer};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum CodeSystem {
    #[default]
    Yandex,
    Esr,
    All,
}

impl CodeSystem {
    pub fn to_string(&self) -> String {
        match self {
            CodeSystem::Yandex => String::from("yandex"),
            CodeSystem::All => String::from("all"),
            CodeSystem::Esr => String::from("esr"),
        }
    }
}

impl<'de> Deserialize<'de> for CodeSystem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "all" => Ok(CodeSystem::All),
            "yandex" => Ok(CodeSystem::Yandex),
            "esr" => Ok(CodeSystem::Esr),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown transport type: {}",
                s
            ))),
        }
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
