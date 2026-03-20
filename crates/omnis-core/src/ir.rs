use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OmnisCollection {
    pub metadata: ProjectMetadata,
    pub entities: IndexMap<String, OmnisEntity>,
}

#[derive(Debug, Serialize)]
pub struct ProjectMetadata {
    pub version: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct EnumDef {
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Serialize)]
pub struct EnumVariant {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct ModelDef {
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Serialize)]
pub struct FieldDef {
    pub name: String,
    pub base_type: String,
    pub is_nullable: bool,
    pub is_optional: bool,
}

impl FieldDef {
    pub fn from_yaml(name: String, raw_type: &str) -> Self {
        // split by the pipe character, and trim whitespace from each segment
        let parts: Vec<&str> = raw_type.split('|').map(|s| s.trim()).collect();

        let base_type = parts.get(0).unwrap_or(&"any").to_string();

        let is_nullable = parts.iter().any(|&p| p == "null");
        let is_optional = parts.iter().any(|&p| p == "undefined");

        Self {
            name,
            base_type,
            is_nullable,
            is_optional,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum OmnisEntity {
    Model(ModelDef),
    Enum(EnumDef),
}
