use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};

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
