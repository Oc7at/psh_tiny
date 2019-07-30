mod args;
mod printer;
use clap::ArgMatches;

pub fn run_relationships<'a>(matches: &ArgMatches<'a>) -> Option<bool> {
    let services = psh_config::service::get_services()?;

    if matches.is_present("relation") {
        let relation = matches.value_of("relation").unwrap();
        let service = match services.get(relation) {
            Some(a) => a,
            None => return None,
        };
        printer::print_relationship(&service, relation);
        return Some(true);
    } else {
        printer::print_relationships(&services)
    }
    return None;
}

pub fn run_routes<'a>(matches: &ArgMatches<'a>) -> Option<bool> {
    let routes = psh_config::route::get_routes().unwrap();
    if matches.is_present("id") {
        let req_route = matches.value_of("id").unwrap();
        let route = match routes.get(req_route) {
            Some(a) => a,
            None => return None,
        };
        printer::print_route(route);
        return Some(true);
    } else {
        for (name, route) in routes {
            println!("{}: {}", name, route);
        }
    }
    None
}

fn main() {
    if !psh_config::config::is_valid_platform() {
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
