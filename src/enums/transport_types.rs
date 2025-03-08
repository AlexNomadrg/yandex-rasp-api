use serde::{Deserialize, Serialize, Serializer};

#[derive(Default, Debug, Clone)]
pub enum TransportType {
    #[default]
    All,
    Plane,
    Train,
    Suburban,
    Bus,
    Water,
    Helicopter,
    Sea,
}

impl TransportType {
    pub fn to_string(&self) -> String {
        match self {
            &TransportType::All => String::from("all"),
            &TransportType::Plane => String::from("plane"),
            &TransportType::Train => String::from("train"),
            &TransportType::Suburban => String::from("suburban"),
            &TransportType::Bus => String::from("bus"),
            &TransportType::Water => String::from("water"),
            &TransportType::Helicopter => String::from("helicopter"),
            &TransportType::Sea => String::from("sea"),
        }
    }
}

// Implement Deserialize for TransportTypes
impl<'de> Deserialize<'de> for TransportType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "all" => Ok(TransportType::All),
            "plane" => Ok(TransportType::Plane),
            "train" => Ok(TransportType::Train),
            "suburban" => Ok(TransportType::Suburban),
            "bus" => Ok(TransportType::Bus),
            "water" => Ok(TransportType::Water),
            "helicopter" => Ok(TransportType::Helicopter),
            "sea" => Ok(TransportType::Sea),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown transport type: {}",
                s
            ))),
        }
    }
}

impl Serialize for TransportType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        serializer.serialize_str(&s)
    }
}
