use std::path::PathBuf;

use bpmn_schema::BpmnFile;

fn load_reference_file(file_name: &str) -> BpmnFile {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/examples");
    path.push(file_name);

    bpmn_schema::from_file(path).expect("Valid thread")
}

#[test]
fn test_pizza() {
    let _ = load_reference_file("triso - Order Process for Pizza V4.bpmn");
}

#[test]
fn test_voting() {
    let _ = load_reference_file("Email Voting 2.bpmn");
}

#[test]
fn test_nobel_prize() {
    let _ = load_reference_file("Nobel Prize Process.bpmn");
}

#[test]
fn test_travel_booking() {
    let _ = load_reference_file("Tavel Booking.bpmn");
}

#[test]
fn test_hardware_retailer() {
    let _ = load_reference_file("triso - Hardware Retailer v2.bpmn");
}
