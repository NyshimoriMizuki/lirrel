use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json as json;

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

const STD_CONFIG: &str = r#"{
  "name": "Usual Cicle",
  "alarm": "buzz",
  "cycling": true,
  "stages": [
    {
      "name": "Break",
      "duration": 300
    },
    {
      "name": "Work",
      "duration": 1500
    }
  ]
}"#;

#[derive(Serialize, Deserialize)]
pub struct TimerConfig {
    pub name: String,
    pub alarm: String,
    pub cycling: bool,
    pub stages: Vec<Stage>,
}
impl TimerConfig {
    pub fn from_file(path: &str) -> json::Result<Self> {
        match fs::read_to_string(path) {
            Ok(c) => json::from_str::<TimerConfig>(&c),
            Err(e) => {
                eprintln!("ERROR: {e}");
                json::from_str::<TimerConfig>(STD_CONFIG)
            }
        }
    }

    pub fn from_str(json_str: &str) -> json::Result<Self> {
        json::from_str::<TimerConfig>(json_str)
    }

    pub fn to_string(&self) -> String {
        json::to_string_pretty(self)
            .expect("Error: Something went wrong when parsed StagesFile to String.")
    }

    pub fn save_to_file(&self, path: &PathBuf) -> std::io::Result<()> {
        let contents = self.to_string();
        fs::write(path, contents)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stage {
    name: String,
    #[serde(with = "secs")]
    duration: Duration,
}
impl Stage {
    pub fn new(name: &str, secs: u64) -> Stage {
        Stage {
            name: name.to_string(),
            duration: Duration::from_secs(secs),
        }
    }
}

mod secs {
    use super::*;
    pub fn serialize<S: Serializer>(d: &Duration, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u64(d.as_secs())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        Ok(Duration::from_secs(u64::deserialize(d)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycles_file_reading() {
        let f = TimerConfig::from_str(STD_CONFIG).unwrap();
        assert_eq!(f.name, "Usual Cicle");
        assert_eq!(f.alarm, "buzz");
        assert_eq!(f.stages[0].name, "Break");
        assert_eq!(f.stages[0].duration, Duration::from_secs(300));
        assert_eq!(f.stages[1].name, "Work");
        assert_eq!(f.stages[1].duration, Duration::from_secs(1500));
    }

    #[test]
    fn test_cycles_file_writing() {
        let file = TimerConfig {
            name: "Usual Cicle".to_string(),
            alarm: "buzz".to_string(),
            cycling: true,
            stages: vec![Stage::new("Break", 300), Stage::new("Work", 1500)],
        };
        assert_eq!(file.to_string(), STD_CONFIG);
    }
}
