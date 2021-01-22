use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand};

const AFTER_HELP: &str = "Execute without any sub-commands \
to start an interactive prompt";

pub fn cgu_app<'a, 'b>(add: &str, select: &str, delete: &str, view: &str) -> App<'a, 'b> {
    let add = SubCommand::with_name(add)
        .about("Add a set of git config settings")
        .arg(
            Arg::with_name("user.name")
                .value_name("user.name")
                .takes_value(true)
                .required(true)
                .help("The name of the commit author"),
        )
        .arg(
            Arg::with_name("user.email")
                .value_name("user.email")
                .takes_value(true)
                .required(true)
                .help("The email of the commit author"),
        )
        .arg(
            Arg::with_name("user.signingkey")
                .value_name("user.signingkey")
                .takes_value(true)
                .help("A GPG key to sign commits with"),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .visible_alias("alias")
                .value_name("name/alias")
                .takes_value(true)
                .help("A name to give to the set of config settings"),
        );
    let select = SubCommand::with_name(select)
        .about("Choose a set of git config settings to change to")
        .arg(
            Arg::with_name("name")
                .takes_value(true)
                .value_name("config name")
                .help("Name of the set of config settings to change to"),
        );
    let delete = SubCommand::with_name(delete)
        .about("Remove one or more set of git config settings")
        .arg(
            Arg::with_name("names")
                .takes_value(true)
                .min_values(1)
                .value_name("config name")
                .help("Name(s) of the config settings to remove"),
        );
    let view = SubCommand::with_name(view)
        .about("View the details of a set of config settings")
        .arg(
            Arg::with_name("all")
                .short("a")
                .long("all")
                .visible_alias("list")
                .help("Show details of all sets of git config settings"),
        )
        .arg(
            Arg::with_name("name")
                .takes_value(true)
                .value_name("config name")
                .required_unless("all")
                .help("Name of the set of config settings to view"),
        );

    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .after_help(AFTER_HELP)
        .global_setting(AppSettings::ColoredHelp)
        .subcommand(add)
        .subcommand(select)
        .subcommand(delete)
        .subcommand(view)
}

pub mod add;
