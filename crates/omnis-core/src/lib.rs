pub mod ir;
pub mod parser;

pub use ir::OmnisCollection;
pub use parser::parse_raw_yaml;

use tera::{Context, Tera};

pub fn render_emitter(
    ir: OmnisCollection,
    template_str: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut tera = Tera::default();

    tera.add_raw_template("emitter", template_str)?;

    let context = Context::from_serialize(&ir)?;

    let result = tera.render("emitter", &context)?;
    Ok(result)
}
