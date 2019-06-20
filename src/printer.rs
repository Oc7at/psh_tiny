use serde_json::Value;
use std::str;
extern crate base64;

pub fn print_relationship_full(relationships: &Value, relation: &str) {
    println!("{}", &relationships[&relation][0]);
}
pub fn print_relationship_elem(relationships: &Value, relation: &str, elem: &str) {
    if relationships[&relation][0][&elem].is_string() {
        println!("{}", &relationships[&relation][0][&elem].as_str().unwrap());
    } else {
        println!("{}", &relationships[&relation][0][&elem]);
    }
}
pub fn print_relationships(relationships: &Value) {
    println!("{}", &relationships);
}
