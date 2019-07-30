use std::collections::HashMap;
use std::str;

pub fn print_relationship(service: &psh_config::service::Service, relation: &str) {
    println!("{}: {:#?}", relation, service);
}

pub fn print_relationships(relationships: &HashMap<String, psh_config::service::Service>) {
    for (name, service) in relationships {
        println!("{}: {}", name, service);
    }
}

pub fn print_route(route: &psh_config::route::Route) {
    dbg!(route);
}
