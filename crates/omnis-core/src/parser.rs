use indexmap::IndexMap;
use serde_yaml::Value;

use crate::{
    OmnisCollection,
    ir::{EnumDef, EnumVariant, FieldDef, ModelDef, OmnisEntity},
};

pub fn parse_raw_yaml(
    yaml_str: &str,
) -> Result<IndexMap<String, OmnisEntity>, Box<dyn std::error::Error>> {
    let raw: IndexMap<String, Value> = serde_yaml::from_str(yaml_str)?;

    let mut entities = IndexMap::new();

    for (name, value) in raw {
        let entity = match value {
            serde_yaml::Value::Sequence(seq) => {
                let variants = seq
                    .iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| {
                        if s.contains(',') {
                            // Handle "Okay, 200"
                            let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
                            let value = parts.get(1).unwrap_or(&"").to_string();
                            EnumVariant {
                                label: parts[0].to_string(),
                                value,
                            }
                        } else {
                            EnumVariant {
                                label: s.to_string(),
                                value: format!(r#""{}""#, s.to_string()),
                            }
                        }
                    })
                    .collect();

                OmnisEntity::Enum(EnumDef { variants })
            }
            serde_yaml::Value::Mapping(map) => {
                let mut fields = Vec::new();
                for (f_name, f_type) in map {
                    if let (Some(k), Some(v)) = (f_name.as_str(), f_type.as_str()) {
                        fields.push(FieldDef::from_yaml(k.to_string(), v));
                    }
                }
                OmnisEntity::Model(ModelDef { fields })
            }
            _ => continue,
        };

        entities.insert(name, entity);
    }

    Ok(entities)
}
