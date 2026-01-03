use std::io::BufReader;

use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

use bpmn_schema::bpmn_20::Definitions;

fn load_file(path: &str) -> Definitions {
    let path = path.to_string();
    let child = std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let input_file = std::fs::File::open(path).unwrap();
            let reader = BufReader::new(input_file);
            let mut reader = IoReader::new(reader).with_error_info();
            Definitions::deserialize(&mut reader).unwrap()
        })
        .unwrap();

    child.join().unwrap()
}

#[test]
fn test_load_example_bpmn() {
    let _ = load_file("tests/examples/triso - Order Process for Pizza V4.bpmn");
}
