
/// Get the first 8 triples in the default graph
pub fn select_all_triples_limit_8() -> &'static str {
    "SELECT * WHERE { ?s ?p ?o } LIMIT 8"
}

/// Query that lists soccer players from Funchal, Madeira; alongside their kit/jersey numbers
///
/// See https://medium.com/virtuoso-blog/dbpedia-basic-queries-bc1ac172cc09
pub fn select_soccer_players() -> &'static str {
    r###"
    SELECT * WHERE {
        ?athlete a dbo:SoccerPlayer;
        dbo:birthPlace [rdfs:label "Funchal"@en; dbo:state dbr:Madeira];
        dbo:number ?number.
    } LIMIT 20
    "###
}
