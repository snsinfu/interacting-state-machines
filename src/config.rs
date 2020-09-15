use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    nodes: Vec<String>,
    transitions: Vec<Transition>,
    generations: Vec<Generation>,
    destructions: Vec<Destruction>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Input {
    #[serde(rename = "linear")]
    Linear { node: String },

    #[serde(rename = "step")]
    Step { node: String, threshold: f64 },
}

#[derive(Debug, Deserialize)]
struct Transition {
    source: String,
    destination: String,
    rate: f64,
    input: Option<Input>,
}

#[derive(Debug, Deserialize)]
struct Generation {
    node: String,
    rate: f64,
    input: Option<Input>,
}

#[derive(Debug, Deserialize)]
struct Destruction {
    node: String,
    rate: f64,
    input: Option<Input>,
}
