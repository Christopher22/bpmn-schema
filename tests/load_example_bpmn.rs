use std::io::BufReader;

use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

use bpmn_schema::bpmn_20::Definitions;

#[test]
fn test_load_example_bpmn() {
    let input_file =
        std::fs::File::open("tests/examples/triso - Order Process for Pizza V4.bpmn").unwrap();
    let reader = BufReader::new(input_file);
    let mut reader = IoReader::new(reader).with_error_info();
    let _ = Definitions::deserialize(&mut reader).unwrap();
}
