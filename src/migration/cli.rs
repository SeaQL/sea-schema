//! Migrator CLI utility

use clap::{App, Arg, SubCommand};

/// Migrator CLI subcommands
pub fn get_subcommands() -> Vec<App<'static, 'static>> {
    vec![
        SubCommand::with_name("fresh")
            .about("Drop all tables from the database, then reapply all migrations"),
        SubCommand::with_name("refresh")
            .about("Rollback all applied migrations, then reapply all migrations"),
        SubCommand::with_name("reset").about("Rollback all applied migrations"),
        SubCommand::with_name("status").about("Check the status of all migrations"),
        SubCommand::with_name("up")
            .about("Apply pending migrations")
            .arg(
                Arg::with_name("NUM_MIGRATION")
                    .long("num")
                    .short("n")
                    .help("Number of pending migrations to be applied")
                    .takes_value(true),
            ),
        SubCommand::with_name("down")
            .about("Rollback applied migrations")
            .arg(
                Arg::with_name("NUM_MIGRATION")
                    .long("num")
                    .short("n")
                    .help("Number of pending migrations to be rolled back")
                    .takes_value(true)
                    .default_value("1"),
            ),
    ]
}
