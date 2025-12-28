# bpmn-schema

**bpmn-schema** is a Rust crate that gives you a standard‑compliant, low-level interface for parsing, representing, and manipulating BPMN (Business Process Model and Notation) documents. It is built on top of the amazing [xsd-parser](https://github.com/Bergmann89/xsd-parser) crate, which converts the official BPMN XSD schemas into raw Rust types automatically at build time.

Currently, the crate supports the BPMN 2.0.2 specification, which is the most widely adopted version. The [XSD schemas](schema/) for BPMN 2.0.2 were obtained from the [official OMG (Object Management Group) website](https://www.omg.org/spec/BPMN).

## License
Licensed under the MIT license.
