use clap::{Parser, Subcommand};

mod auth;
mod client;
mod config;
mod master;
mod worker;

const LOGO: &'static str = r#"

██████  ██████  ███████ ███████
██   ██ ██   ██ ██      ██
██████  ██   ██ █████   ███████
██   ██ ██   ██ ██           ██
██   ██ ██████  ██      ███████

 a toy distributed file system
"#;

#[derive(Parser, Default, Debug)]
#[clap(
    author = "Wat The Doodle <watthedoodle@gmail.com>",
    version,
    about=LOGO
)]
#[command(help_template(
    "\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
))]
struct Arguments {
    /// use commands: List, Get, Add, and Remove
    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    /// List all remote files e.g rdfs list
    List { path: Option<String> },
    /// Get a remote file e.g rdfs get foo.txt
    Get { file: String },
    /// Add a remote file e.g rdfs add foo.txt
    Add { file: String },
    /// Remove a remote file e.g rdfs remove foo.txt
    Remove { file: String },
    /// Mode: run the binary in either as a "Master" or "Worker" node
    Mode {
        /// kind: allowed values are "master" or "worker"
        kind: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    match &args.cmd {
        Some(Commands::List { path }) => {
            client::list(path);
        }
        Some(Commands::Get { file }) => {
            client::get(&file);
        }
        Some(Commands::Add { file }) => {
            client::add(&file);
        }
        Some(Commands::Remove { file }) => client::remove(&file),
        Some(Commands::Mode { kind }) => match kind.as_ref() {
            "master" => {
                master::init();
            }
            "worker" => {
                let _ = worker::init().await;
            }
            _ => {
                println!("illegal mode, please select option master or worker!");
            }
        },
        None => {
            println!("Unknown command!");
        }
    }
}
