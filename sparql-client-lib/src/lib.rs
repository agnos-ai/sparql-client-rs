//! A crate for writing sparql clients in safe Rust
//!
#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
trivial_numeric_casts, unused_import_braces, unused_qualifications
)]

use std::time::Duration;

use actix_web::client::{Client, ClientRequest};
use actix_web::client::Connector;
use actix_web::http::{header, Uri};
use mime::Mime;
use openssl::ssl::{SslConnector, SslMethod};
use serde::Serialize;

/// TODO: Documentation for json module
pub mod json;
/// Some example SPARQL queries
pub mod query_template;
/// Structs for StructOpt-based SPARQL CLI
pub mod cli_model;

/// Default timeout for sparql_client()
pub const TIMEOUT: u64 = 1000;

/// Return a actix_web::client::Client that's configured for a SPARQL query
pub fn sparql_client(timeout: u64) -> Client {

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();

    Client::builder()
        .connector(
            Connector::new()
                .timeout(Duration::from_millis(timeout))
                .ssl(builder.build())
                .finish(),
        )
        .finish()
}

/// Struct used by serde to serialize the ?query= part to the URL
#[derive(Serialize, Debug)]
pub struct GetRequestParams {
    /// The ?query= port of the SPARQL GET URL
    pub query: String
}

/// Set up a ClientRequest to perform a SPARQL query
pub fn sparql_get(client: Client, host: Uri, accept: Mime, query: &str) -> ClientRequest {

    let params = GetRequestParams {
        query: (&query).parse().unwrap()
    };

    client
        .get(host)
        .header("User-Agent", "agnos-ai/sparql-client-rs")
        .header(header::ACCEPT, accept)
        .query(&params)
        .unwrap()
}
