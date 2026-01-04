//! Fundamental structures to work with BPMN 2.0 files in Rust.

#[allow(dead_code)]
#[allow(unused)]
/// The Rust types derived from the BPMN 2.0 schema.
pub mod schema {
    include!(concat!(env!("OUT_DIR"), "/schema.rs"));
}

/// A re-export of the BPMN definitions. This type represents the root element of a BPMN 2.0 file and is likely to be the main interest for users of this crate.
pub use self::schema::bpmn_20::Definitions as BpmnFile;

/// Load a BPMN file from the specified path and deserialize it.
/// This convenience function spawns a new thread with an increased stack size to handle large BPMN files which may otherwise cause stack overflows during deserialization.
pub fn from_file<F: AsRef<std::path::Path>>(
    path: F,
) -> Result<BpmnFile, xsd_parser_types::quick_xml::Error> {
    use xsd_parser_types::quick_xml::{DeserializeSync, IoReader, XmlReader};

    let path = path.as_ref().to_path_buf();
    std::thread::Builder::new()
        .stack_size(32 * 1024 * 1024)
        .spawn(move || {
            let input_file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(input_file);
            let mut reader = IoReader::new(reader).with_error_info();
            BpmnFile::deserialize(&mut reader)
        })
        .expect("Failed to spawn thread")
        .join()
        .expect("Thread panicked unexpectedly")
}
