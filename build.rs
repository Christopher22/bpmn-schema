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
        RenderStep::PrefixConstants,
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
    let code = rustfmt_pretty_print(code)?;

    let mut file = File::create(PathBuf::from(env::var("OUT_DIR").unwrap()).join("schema.rs"))?;
    file.write_all(code.to_string().as_bytes())?;

    Ok(())
}

pub fn rustfmt_pretty_print(code: String) -> Result<String, Error> {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().unwrap();

    write!(stdin, "{code}")?;
    stdin.flush()?;
    drop(stdin);

    let std::process::Output {
        status,
        stdout,
        stderr,
    } = child.wait_with_output()?;

    let stdout = String::from_utf8_lossy(&stdout);
    let stderr = String::from_utf8_lossy(&stderr);

    if !status.success() {
        let code = status.code();
        match code {
            Some(code) => {
                if code != 0 {
                    panic!("The `rustfmt` command failed with return code {code}!\n{stderr}");
                }
            }
            None => {
                panic!("The `rustfmt` command failed!\n{stderr}")
            }
        }
    }

    Ok(stdout.into())
}
