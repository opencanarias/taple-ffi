use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeMap};

#[derive(Serialize, Deserialize, Clone)]
pub struct Policy {
  pub id: String,
  pub approve: Validation,
  pub evaluate: Validation,
  pub validate: Validation,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Validation {
  pub quorum: Quorum,
}

#[derive(Clone)]
pub enum Quorum {
    MAJORITY,
    FIXED { value: u64 },
    PERCENTAGE { value: f64 },
}


impl Serialize for Quorum {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: serde::Serializer,
  {
      match self {
          Quorum::FIXED { value } => {
              let mut map = serializer.serialize_map(Some(1))?;
              map.serialize_entry("FIXED", value)?;
              map.end()
          }
          Quorum::PERCENTAGE { value } => {
              let mut map = serializer.serialize_map(Some(1))?;
              map.serialize_entry("PERCENTAGE", value)?;
              map.end()
          }
          Quorum::MAJORITY => serializer.serialize_str("MAJORITY"),
      }
  }
}


impl<'de> Deserialize<'de> for Quorum {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: serde::Deserializer<'de>,
  {
      struct QuorumVisitor;
      impl<'de> Visitor<'de> for QuorumVisitor {
          type Value = Quorum;
          fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
              formatter.write_str("Quorum")
          }
          fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
          where
              A: serde::de::MapAccess<'de>,
          {
              // Solo deberían tener una entrada
              let Some(key) = map.next_key::<String>()? else {
                  return Err(serde::de::Error::missing_field("FIXED or PERCENTAGE"))
              };
              let result = match key.as_str() {
                  "FIXED" => {
                      let value: u64 = map.next_value()?;
                      Quorum::FIXED { value: value }
                  }
                  "PERCENTAGE" => {
                      let value: f64 = map.next_value()?;
                      Quorum::PERCENTAGE { value: value }
                  }
                  _ => return Err(serde::de::Error::unknown_field(&key, &["ID", "NAME"])),
              };
              let None = map.next_key::<String>()? else {
                  return Err(serde::de::Error::custom("Input data is not valid. The data contains unkown entries"));
              };
              Ok(result)
          }
          fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
          where
              E: serde::de::Error,
          {
              match v.as_str() {
                  "MAJORITY" => Ok(Quorum::MAJORITY),
                  other => Err(serde::de::Error::unknown_variant(
                      other,
                      &["MAJORITY"],
                  )),
              }
          }
          fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
          where
              E: serde::de::Error,
          {
              match v {
                  "MAJORITY" => Ok(Quorum::MAJORITY),
                  other => Err(serde::de::Error::unknown_variant(
                      other,
                      &["MAJORITY"],
                  )),
              }
          }
      }
      deserializer.deserialize_any(QuorumVisitor {})
  }
}
