use std::path::PathBuf;
use actix_web::http::Uri;
use structopt::{clap::ArgGroup, StructOpt};
use mime::{TEXT_CSV, APPLICATION_JSON, Mime};

#[derive(StructOpt, Debug)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
#[structopt(rename_all = "kebab-case")]
#[structopt(about = "a CLI for executing SPARQL statements")]
pub struct Cli {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Activate verbose mode
    #[structopt(short, long)]
    pub verbose: bool,

    /// The SPARQL endpoint URL
    #[structopt(short, long, env = "EKG_SPARQL_ENDPOINT")]
    pub endpoint: Uri,

    /// The path to the SPARQL file to read
    #[structopt(parse(from_os_str), help = "path of the SPARQL file to execute")]
    pub path: PathBuf,

    /// The timeout in milliseconds to use for the SPARQL statement
    #[structopt(short, long, default_value = "1000")]
    pub timeout: u64,

    /// The type of output, either json or csv
    #[structopt(flatten)]
    pub output_mime: OutputMime,

    #[structopt(subcommand)]
    pub cmd: Command
}

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Run a SPARQL statement
    Run,
    /// TODO: Pretty print SPARQL
    Info
}

#[derive(StructOpt, Debug)]
#[structopt(group = ArgGroup::with_name("output_mime").required(true))]
pub struct OutputMime {
    /// JSON output
    #[structopt(long, group = "output_mime")]
    pub json: bool,
    /// CSV output
    #[structopt(long, group = "output_mime")]
    pub csv: bool
}

pub(crate) fn output_mime(args: &Cli) -> Mime {
    match args.output_mime {
        OutputMime { csv: true, .. } => TEXT_CSV,
        OutputMime { json: true, .. } => APPLICATION_JSON,
        _ => APPLICATION_JSON
    }
}