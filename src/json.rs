
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fmt;

/// The value of the "head" key is an array of all variables projected in the query's SELECT clause.
///
/// See https://www.w3.org/TR/sparql11-results-json/#select-head
#[derive(Serialize, Deserialize, Debug)]
pub struct SparqlResultObjectHead {
    /// The "vars" member is an array giving the names of the variables used in the results.
    /// These are the projected variables from the query.
    /// A variable is not necessarily given a value in every query solution of the results.
    ///
    /// See https://www.w3.org/TR/sparql11-results-json/#select-vars
    pub vars: Vec<String>,
    /// The optional "link" member gives an array of URIs, as strings,
    /// to refer for further information.
    /// The format and content of these link references is not defined by the SPARQL standard.
    ///
    /// See https://www.w3.org/TR/sparql11-results-json/#select-link
    pub link: Option<Vec<String>>
}

/// An RDF term (IRI, literal or blank node) is encoded as a JSON object.
/// All aspects of the RDF term are represented.
/// The JSON object has a "type" member and other members depending on
/// the specific kind of RDF term.
///
/// See https://www.w3.org/TR/sparql11-results-json/#select-encode-terms
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SparqlResultObjectTerm {
    /// An IRI referencing a resource.
    ///
    /// TODO: Look into the use of Box<str>, is probably not the way to go, use sophia terms if possible
    #[serde(rename = "uri")]
    Iri(Box<str>),
    /// A blank node.
    ///
    /// Also known as existentially quantified variable.
    /// The blank node label is scoped to the results object.
    /// That is, two blank nodes with the same label in a single SPARQL Results JSON object
    /// are the same blank node. This is not an indication of any internal system identifier
    /// the SPARQL processor may use. Use of the same label in another SPARQL Results JSON
    /// object does not imply it is the same blank node.
    ///
    /// TODO: Tie BNodes with same identifier together as one
    #[serde(rename = "bnode")]
    BNode(Box<str>),
    /// An RDF literal.
    ///
    /// TODO: Support all detail fields as well such as xml:lang and datatype
    #[serde(rename = "literal")]
    Literal(Box<str>)
}

impl fmt::Debug for SparqlResultObjectTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SparqlResultObjectTerm::Iri(str) => write!(f, "<{}>", str),
            SparqlResultObjectTerm::BNode(str) => write!(f, "\"{}\"", str),
            SparqlResultObjectTerm::Literal(str) => write!(f, "\"{}\"", str),
        }
    }
}

/// The value of the "results" member is an object with a single key, "bindings".
///
/// See https://www.w3.org/TR/sparql11-results-json/#select-results
#[derive(Serialize, Deserialize, Debug)]
pub struct SparqlResultObjectResults {
    /// The value of the "bindings" member is an array with zero or more elements,
    /// one element per query solution. Each query solution is a JSON object.
    /// Each key of this object is a variable name from the query solution.
    /// The value for a given variable name is a JSON object that encodes the variable's
    /// bound value, an RDF term. There are zero elements in the array if the query returned
    /// an empty solution sequence.
    /// Variables names do not include the initial "?" or "$" character.
    /// Each variable name that appears as a key within the "bindings" array will have
    /// appeared in the "vars" array in the results header.
    ///
    /// A variable does not appear in an array element if it is not bound in that
    /// particular query solution.
    ///
    /// The order of elements in the bindings array reflects the order, if any,
    /// of the query solution sequence.
    ///
    /// See https://www.w3.org/TR/sparql11-results-json/#select-bindings
    pub bindings: Vec<HashMap<String, SparqlResultObjectTerm>>
}

/// The results of a SPARQL SELECT query are serialized as an array of bindings of variables.
/// The value of the "head" key is an array of all variables projected in the query's SELECT clause.
///
/// See https://www.w3.org/TR/sparql11-results-json/#select-results-form
#[derive(Serialize, Deserialize, Debug)]
pub struct SparqlResultObject {
    head: SparqlResultObjectHead,
    results: SparqlResultObjectResults
}