mod cli_model;

use structopt::StructOpt;
use sparql_client_rs::*;
use sparql_client_rs::json::SparqlResultObject;
use std::process::exit;
use std::fs;
use crate::cli_model::output_mime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let args = cli_model::Cli::from_args();

    if args.debug {
        eprintln!("args={:?}", args);
    }

    let statement: String = fs::read_to_string(&args.path)?;
    let output_mime = output_mime(&args);
    eprintln!("- Output type: {:?}", output_mime);

    if args.verbose {
        eprintln!("- Endpoint : {:#?}", args.endpoint);
        eprintln!("- File     : {:#?}", args.path);
        eprintln!("- Statement:\n{}", statement);
    }

    let request = sparql_get(
        sparql_client(TIMEOUT),
        args.endpoint,
        output_mime,
        &statement
    ).send();

    let mut response = request.await.unwrap();
    let status_code = response.status();

    //
    if args.verbose {
        eprintln!("- Response  : {:?}", response);
        eprintln!("- Status    : {:?}", status_code);
    }

    if ! status_code.is_success() {
        eprintln!("Could not execute SPARQL statement (error code: {})", status_code);
        exit(1);
    }

    if args.output_mime.csv {
        let bytes = response.body().await.unwrap();
        let csv_string = std::str::from_utf8(&bytes).unwrap();
        println!{"{}", csv_string};
    } else {
        let result = response.json::<SparqlResultObject>();

        for value in result.await {
            println!("Result: {}", serde_json::to_string_pretty(&value).unwrap())
        }
    }

    Ok(())
}