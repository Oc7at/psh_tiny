#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use serde_json::Value;
use std::env;
use std::str;
extern crate base64;

pub fn get_json_from_platform_var(var: &str) -> String {
    let b64_rel = env::var(var).unwrap();
    let rel = base64::decode(&b64_rel).unwrap();
    str::from_utf8(&rel).unwrap().to_owned()
}

pub fn print_relationship_full(relationships: &String, relation: &str) {
    let v: Value = serde_json::from_str(relationships).unwrap();
    println!("{}", &v[&relation][0]);
}
pub fn print_relationship_elem(relationships: &String, relation: &str, elem: &str) {
    let v: Value = serde_json::from_str(relationships).unwrap();
    println!("{}", &v[&relation][0][&elem]);
}
pub fn print_relationships(relationships: &String) {
    let v: Value = serde_json::from_str(relationships).unwrap();
    println!("{}", &v);
}

pub fn arg_parser() -> ArgMatches<'static> {
    App::new("psh-tiny")
        .version(crate_version!())
        .setting(AppSettings::InferSubcommands)
        .subcommand(
            SubCommand::with_name("relationships")
                .arg(
                    Arg::with_name("relation")
                        .help("the relationship to get")
                        .index(1)
                        .required(false),
                )
                .arg(
                    Arg::with_name("element")
                        .help("the element to get")
                        .index(2)
                        .required(false),
                ),
        )
        .get_matches()
}

pub fn run_relationships<'a>(matches: &ArgMatches<'a>) {
    let relationships = get_json_from_platform_var("PLATFORM_RELATIONSHIPS");
    if matches.is_present("relation") {
        if matches.is_present("element") {
            print_relationship_elem(
                &relationships,
                matches.value_of("relation").unwrap(),
                matches.value_of("element").unwrap(),
            );
        } else {
            print_relationship_full(&relationships, matches.value_of("relation").unwrap());
        }
    } else {
        print_relationships(&relationships)
    }
}
