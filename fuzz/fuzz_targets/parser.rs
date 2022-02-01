#![no_main]
use apollo_parser::Parser;
use apollo_rs_fuzz::generate_valid_document;
use libfuzzer_sys::{arbitrary::Unstructured, fuzz_target};
use std::panic;

fuzz_target!(|data: &[u8]| {
    let mut u = Unstructured::new(data);
    u.int_in_range(0..=1000).unwrap();
    let doc_generated = generate_valid_document(data).unwrap();

    let parser = panic::catch_unwind(|| Parser::new(&doc_generated));

    let parser = match parser {
        Err(_) => return,
        Ok(p) => p,
    };

    println!("Parsing...");
    let tree = parser.parse();
    println!("Parsed");
    // early return if the parser detected an error
    if tree.errors().next().is_some() {
        panic!("error detected");
    }
});
