use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use xsd_parser::{DefaultsRenderStep, Error, TypesRenderStep};

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=schema");

    let parser = xsd_parser::pipeline::parser::Parser::new()
        .with_default_resolver()
        .resolve_includes(true)
        .generate_prefixes(false)
        .alternative_prefixes(false)
        .with_default_namespaces()
        .add_schema_from_file("schema/BPMN20.xsd")
        .expect("parser failed");
    let schemas = parser.finish();

    let interpreter = xsd_parser::pipeline::interpreter::Interpreter::new(&schemas)
        .with_buildin_types()
        .expect("interpreter: default typedefs")
        .with_default_typedefs()
        .expect("interpreter: default typedefs");
    let meta_types = interpreter.finish().expect("interpreter failed");

    let optimizer = xsd_parser::pipeline::optimizer::Optimizer::new(meta_types);
    let meta_types = optimizer.finish();

    let generator = xsd_parser::pipeline::generator::Generator::new(&meta_types);
    let output = generator
        .into_fixed()
        .generate_named_types()
        .expect("generator: failed")
        .finish();

    /*
    Debug output for tEventBasedGateway issue:

    panic!(
        "{:#?}",
        output
            .items
            .iter()
            .filter(
                |(ident, _)| ident.name.as_named_str() == Some("tEventBasedGateway")
                    || ident.name.as_named_str() == Some("tEventBasedGatewayType")
            )
            .collect::<Vec<_>>()
    );*/

    let renderer = xsd_parser::pipeline::renderer::Renderer::new(&output)
        .with_step(TypesRenderStep)
        .with_step(DefaultsRenderStep);

    let code = renderer.finish();
    let code = code.code.to_string();

    let mut file = File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("schema.rs"))?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
