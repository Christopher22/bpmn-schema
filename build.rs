use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use xsd_parser::{
    Config, Error,
    config::{GeneratorFlags, InterpreterFlags, OptimizerFlags, Schema},
    generate,
};

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=schema");

    let config = Config::default()
        .with_schema(Schema::File("schema/BPMN20.xsd".into()))
        .set_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .set_optimizer_flags(OptimizerFlags::all())
        .set_generator_flags(GeneratorFlags::all())
        .with_type_postfix("XType")
        .with_quick_xml()
        .with_generate([(
            xsd_parser::IdentType::Element,
            xsd_parser::config::NamespaceIdent::namespace(
                b"http://www.omg.org/spec/BPMN/20100524/MODEL",
            ),
            "definitions",
        )]);

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
