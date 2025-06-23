use serde::Deserialize;

#[derive(Default)]
pub enum ScheduleEvent {
    #[default]
    Any,
    Arrival,
    Departure,
}

impl ToString for ScheduleEvent {
    fn to_string(&self) -> String {
        match self {
            &ScheduleEvent::Arrival => "arrival".to_string(),
            &ScheduleEvent::Departure => "departure".to_string(),
            &ScheduleEvent::Any => "".to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for ScheduleEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "arrival" => Ok(ScheduleEvent::Arrival),
            "departure" => Ok(ScheduleEvent::Departure),
            "" => Ok(ScheduleEvent::Any),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown event type: {}",
                s
            ))),
        }
    }
}
