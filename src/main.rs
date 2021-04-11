// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::fmt;
use seed::{prelude::*, *};
use serde::{Serialize, Deserialize};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
type Model = i32;

#[derive(Serialize, Deserialize, Debug)]
struct DynalistDocumentApiResponse {
    _code: String,
    _msg: Option<String>,
    file_id: String,
    title: String,
    nodes: Vec<DynalistNode>,
    version: i32,
}

impl fmt::Display for DynalistDocumentApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file_id: {}, title: {}", self.file_id, self.title)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DynalistNode {
    id: String,
    content: String,
    note: String,
    children: Option<Vec<String>>,
    created: i64,
    modified: i64,
}

impl fmt::Display for DynalistNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}, content: {}", self.id, self.content)
    }
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Increment,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        "This is a counter: ",
        C!["counter"],
        button![model, ev(Ev::Click, |_| Msg::Increment),],
        dynalist_document(),
    ]
}

fn dynalist_document() -> Node<Msg> {
    section![
        read_sample_json(),
    ]
}

fn read_sample_json() -> String {
    let json_str = include_str!("../json/simplest_sample.json");
    let api_resp: DynalistDocumentApiResponse = serde_json::from_str(json_str).expect("Deserialization failed");
    let doc = format!("{}", api_resp);

    doc
}

// ------ ------
//     Start
// ------ ------

pub fn main() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
