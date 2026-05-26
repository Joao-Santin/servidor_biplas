use serde::{Deserialize, Deserializer};

pub fn deserialize_string_or_number<'de, D>(
    deserializer: D,
) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(n) => Ok(n.to_string()),
        _ => Err(serde::de::Error::custom("tipo inválido")),
    }
}
