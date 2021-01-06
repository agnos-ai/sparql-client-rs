
use sparql_client_rs::*;
use sparql_client_rs::query_template::select_soccer_players;
use mime::{APPLICATION_JSON};
use actix_web::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let request = sparql_get(
        sparql_client(),
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

    Ok(())
}