//! Migrator CLI utility

/// Migrator CLI subcommands
#[macro_export]
macro_rules! get_cli_subcommands {
    () => {
        [
            clap::SubCommand::with_name("fresh")
                .about("Drop all tables from the database, then reapply all migrations"),
            clap::SubCommand::with_name("refresh")
                .about("Rollback all applied migrations, then reapply all migrations"),
            clap::SubCommand::with_name("reset").about("Rollback all applied migrations"),
            clap::SubCommand::with_name("status").about("Check the status of all migrations"),
            clap::SubCommand::with_name("up")
                .about("Apply pending migrations")
                .arg(
                    clap::Arg::with_name("NUM_MIGRATION")
                        .long("num")
                        .short("n")
                        .help("Number of pending migrations to be applied")
                        .takes_value(true),
                ),
            clap::SubCommand::with_name("down")
                .about("Rollback applied migrations")
                .arg(
                    clap::Arg::with_name("NUM_MIGRATION")
                        .long("num")
                        .short("n")
                        .help("Number of pending migrations to be rolled back")
                        .takes_value(true)
                        .default_value("1"),
                ),
        ]
    };
}
