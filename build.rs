use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use xsd_parser::{
    Config, Error,
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, RenderStep, Schema},
    generate,
    pipeline::renderer::NamespaceSerialization,
};

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=schema");

    let mut config = Config::default();
    config.parser.schemas = vec![Schema::File("schema/BPMN20.xsd".into())];
    config.interpreter.flags = InterpreterFlags::all();
    config.optimizer.flags = OptimizerFlags::all();
    config.generator.flags = GeneratorFlags::all();
    config.generator.type_postfix.type_ = "XType".into();
    config.generator.type_postfix.element = String::new();
    config.generator.type_postfix.element_type = "XElementType".into();

    // Add renderers for `quick-xml` serializer and deserializer.
    let config = config.with_render_steps([
        RenderStep::Types,
        RenderStep::Defaults,
        RenderStep::NamespaceConstants,
        RenderStep::QuickXmlDeserialize {
            boxed_deserializer: false,
        },
        RenderStep::QuickXmlSerialize {
            namespaces: NamespaceSerialization::Global,
            default_namespace: None,
        },
    ]);

    // Generate the code based on the configuration above.
    let code = generate(config)?;
    let code = code.to_string();

    let mut file = File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("schema.rs"))?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}
