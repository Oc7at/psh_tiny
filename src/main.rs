mod args;
mod env_getter;
mod printer;
use clap::ArgMatches;

pub fn run_relationships<'a>(matches: &ArgMatches<'a>) {
    let relationships = env_getter::get_json_from_platform_var("PLATFORM_RELATIONSHIPS");
    if matches.is_present("relation") {
        if matches.is_present("element") {
            printer::print_relationship_elem(
                &relationships,
                matches.value_of("relation").unwrap(),
                matches.value_of("element").unwrap(),
            );
        } else {
            printer::print_relationship_full(&relationships, matches.value_of("relation").unwrap());
        }
    } else {
        printer::print_relationships(&relationships)
    }
}

fn main() {
    let matches = args::arg_parser();
    if let Some(matches) = matches.subcommand_matches("relationships") {
        run_relationships(matches);
    }
}
