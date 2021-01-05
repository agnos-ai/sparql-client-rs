
#[test]
fn test_sparql_json1() {

    let j = r#"
        {
            "head": {
                "link": [],
                "vars": ["s", "p", "o"]
            },
            "results": {
                "distinct": false,
                "ordered": true,
                "bindings": [
                    {
                        "s": {
                            "type": "uri",
                            "value": "http://www.openlinksw.com/virtrdf-data-formats#default-iid"
                        },
                        "p": {
                            "type": "uri",
                            "value": "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
                        },
                        "o": {
                            "type": "uri",
                            "value": "http://www.openlinksw.com/schemas/virtrdf#QuadMapFormat"
                        }
                    }
                ]
            }
        }
    "#;

    println!("JSON {:?}", j);

    let deserialized: sparql_client::json::SparqlResultObject = serde_json::from_str(&j).unwrap();

    println!("deserialized = {:#?}", deserialized);
}

#[test]
fn test_sparql_json2() {

    let j = r#"
        {
           "head": {
               "link": [
                   "http://www.w3.org/TR/rdf-sparql-XMLres/example.rq"
               ],
               "vars": [
                   "x",
                   "hpage",
                   "name",
                   "mbox",
                   "age",
                   "blurb",
                   "friend"
               ]
           },
           "results": {
               "bindings": [
                       {
                           "x" : { "type": "bnode", "value": "r1" },
                           "hpage" : { "type": "uri", "value": "http://work.example.org/alice/" },
                           "name" : {  "type": "literal", "value": "Alice" },
                           "mbox" : {  "type": "literal", "value": "" },
                           "blurb" : {
                             "datatype": "http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral",
                             "type": "literal",
                             "value": "<p xmlns=\"http://www.w3.org/1999/xhtml\">My name is <b>alice</b></p>"
                           },
                           "friend" : { "type": "bnode", "value": "r2" }
                       },
                       {
                           "x" : { "type": "bnode", "value": "r2" },
                           "hpage" : { "type": "uri", "value": "http://work.example.org/bob/" },
                           "name" : { "type": "literal", "value": "Bob", "xml:lang": "en" },
                           "mbox" : { "type": "uri", "value": "mailto:bob@work.example.org" },
                           "friend" : { "type": "bnode", "value": "r1" }
                       }
                   ]
               }
        }
    "#;

    println!("JSON {:?}", j);

    let deserialized: sparql_client::json::SparqlResultObject = serde_json::from_str(&j).unwrap();

    println!("deserialized = {:#?}", deserialized);
}