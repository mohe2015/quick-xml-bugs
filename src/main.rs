extern crate serde;
use quick_xml::{DeError, de::from_str};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct NamespaceBug {
    #[serde(rename = "xmlns:d")]
    test: String,

    #[serde(rename = "$unflatten=d:test2")] // remove "d:" and it works
    test2: String
}

#[derive(Deserialize, Debug)]
struct InnerNestingBug {
}

#[derive(Deserialize, Debug)]
struct NestingBug {
    // comment out one of these fields and it works
    #[serde(rename = "$unflatten=outer1")]
    outer1: InnerNestingBug,

    #[serde(rename = "$unflatten=outer2")]
    outer2: String
}

#[derive(Deserialize, Debug)]
struct EmptyVecBug {
    #[serde(rename = "$unflatten=element")]
    outer2: Vec<String>
}

fn main() {
    println!("Hello, world!");

    let namespace_bug: Result<NamespaceBug, DeError> = from_str(r#"<?xml version="1.0" encoding="UTF-8"?><d:test xmlns:d="works"><d:test2>doesntwork</d:test2></d:test>"#);

    println!("{:?}", namespace_bug);

    let nesting_bug: Result<NestingBug, DeError> = from_str(r#"<?xml version="1.0" encoding="UTF-8"?><root><outer1></outer1><outer2></outer2></root>"#);

    println!("{:?}", nesting_bug);

    let empty_vec_bug: Result<EmptyVecBug, DeError> = from_str(r#"<?xml version="1.0" encoding="UTF-8"?><root></root>"#);

    println!("{:?}", empty_vec_bug);
}
