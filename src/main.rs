mod args;
mod printer;
use clap::ArgMatches;

pub fn run_relationships<'a>(matches: &ArgMatches<'a>) {
    let relationships = psh_config::get_json_from_var("PLATFORM_RELATIONSHIPS").unwrap();
    if matches.is_present("relation") {
        if !psh_config::has_relationship(matches.value_of("relation").unwrap()) {
            println!("This relationship does not exist.");
            std::process::exit(1);
        }
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

pub fn run_routes<'a>(matches: &ArgMatches<'a>) {
    let routes = psh_config::get_json_from_var("PLATFORM_ROUTES").unwrap();
    dbg!(routes);
}

fn main() {
    if !psh_config::is_valid_platform() {
        println!("This is not a valid Platform.sh environment.");
        std::process::exit(1)
    }
    let commands = args::arg_parser();
    if let Some(matches) = commands.subcommand_matches("relationships") {
        run_relationships(matches);
    } else if let Some(matches) = commands.subcommand_matches("routes") {
        run_routes(matches);
    }
}
