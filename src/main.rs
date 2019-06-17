fn main() {
    let matches = psh_tiny::arg_parser();
    if let Some(matches) = matches.subcommand_matches("relationships") {
        psh_tiny::run_relationships(matches);
    }
}
