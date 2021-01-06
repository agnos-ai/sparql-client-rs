extern crate sparql_client;

use std::time::Duration;

use actix_web::client::Client;
use actix_web::client::Connector;
use mime::{APPLICATION_JSON, TEXT_CSV};

use sparql_client::json::SparqlResultObject;
use sparql_client::query_template::{select_all_triples_limit_8, select_soccer_players};

mod common;

#[actix_rt::test]
async fn test_standard_actix_web_example1() {

    common::setup();

    let client = Client::default();

    let res = client.get("http://www.rust-lang.org") // <- Create request builder
        .header("User-Agent", "Actix-web")
        .send()                             // <- Send http request
        .await;                             // <- send request and wait for response

     println!("Response: {:?}", res);
}

#[actix_rt::test]
async fn test_standard_actix_web_example2() {

    common::setup();

    let host = "http://www.rust-lang.org";

    let client = Client::builder()
        .connector(
            Connector::new()
                .timeout(Duration::from_millis(300))
                .finish(),
        )
        .finish();

    let response = client.get(host.clone()) // <- Create request builder
        .header("User-Agent", "Actix-web")
        .send()                                            // <- Send http request
        .await;

    println!("Response: {:?}", response);
    // assert!(response.status().is_success());
}


#[actix_rt::test]
async fn test_dbpedia_get_root() {

    common::setup();

    let client = sparql_client::sparql_client();

    let host = "https://dbpedia.org/sparql";

    let response = client.get(host.clone()) // <- Create request builder
        .header("User-Agent", "Actix-web")
        .send()                                            // <- Send http request
        .await;

    println!("Response: {:?}", response);
    assert!(response.is_ok());
    assert!(response.unwrap().status().is_success());
}

#[actix_rt::test]
async fn test_dbpedia_sparql_query_as_csv() {

    common::setup();

    let request = sparql_client::sparql_get(
        sparql_client::sparql_client(),
        "https://dbpedia.org/sparql",
        TEXT_CSV,
        select_all_triples_limit_8()
    ).send();

    let mut response = request.await.unwrap();
    let body = response.body();

    println!("Response: {:?}", response);

    let status_code = response.status();
    println!(" - Status: {:?}", status_code);
    assert!(status_code.is_success());

    let bytes = body.await.unwrap();
    println!{"Body: {}", std::str::from_utf8(&bytes).unwrap()};
}

#[actix_rt::test]
async fn test_dbpedia_sparql_query_as_json_body() {

    common::setup();

    let request = sparql_client::sparql_get(
        sparql_client::sparql_client(),
        "https://dbpedia.org/sparql",
        APPLICATION_JSON,
        select_soccer_players()
    ).send();

    let mut response = request.await.unwrap();
    let body = response.body();

    //
    println!("Response: {:?}", response);

    let status_code = response.status();
    println!(" - Status: {:?}", status_code);
    assert!(status_code.is_success());

    let bytes = body.await.unwrap();
    println!{"Body: {}", std::str::from_utf8(&bytes).unwrap()};
}

#[actix_rt::test]
async fn test_dbpedia_sparql_query_as_json_stream1() {

    common::setup();

    let request = sparql_client::sparql_get(
        sparql_client::sparql_client(),
        "https://dbpedia.org/sparql",
        APPLICATION_JSON,
        select_all_triples_limit_8()
    ).send();

    let mut response = request.await.unwrap();

    //
    println!("Response: {:?}", response);

    let status_code = response.status();
    println!(" - Status: {:?}", status_code);
    assert!(status_code.is_success());

    let result = response.json::<SparqlResultObject>();

    for value in result.await {
        println!("Result: {:#?}", value)
    }
}

#[actix_rt::test]
async fn test_dbpedia_sparql_query_as_json_stream2() {

    common::setup();

    let request = sparql_client::sparql_get(
        sparql_client::sparql_client(),
        "https://dbpedia.org/sparql",
        APPLICATION_JSON,
        select_soccer_players()
    ).send();

    let mut response = request.await.unwrap();

    //
    println!("Response: {:?}", response);

    let status_code = response.status();
    println!(" - Status: {:?}", status_code);
    assert!(status_code.is_success());

    let result = response.json::<SparqlResultObject>();

    for value in result.await {
        println!("Result: {:#?}", value)
    }
}


