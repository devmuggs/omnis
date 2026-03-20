use omnis_core::{OmnisCollection, parse_raw_yaml, render_emitter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_path = "omnis/core.yaml";
    let schema_content = std::fs::read_to_string(schema_path)?;

    let template_path = "emitters/typescript/native-v6/generator.tera";
    let template_content = std::fs::read_to_string(template_path)?;

    let ir = parse_raw_yaml(&schema_content)?;
    let omnis_collection = OmnisCollection {
        metadata: omnis_core::ir::ProjectMetadata {
            version: "0.1.0".to_string(),
            name: "hardcoded".to_string(),
        },
        entities: ir,
    };
    let output = render_emitter(omnis_collection, &template_content)?;

    println!("{}", output);
    Ok(())
}
