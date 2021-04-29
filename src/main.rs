// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
// use std::fmt;
// use serde::{Serialize, Deserialize};

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        page: Page::init(url),
    }
}


// ------ ------
//     Model
// ------ ------

struct Model {
    page: Page,
}

// ------ Page ------

enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(ABOUT) => Self::About,
            Some(_) => Self::NotFound,
        }
    }
}


// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}


// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
    }
}


// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        match &model.page {
            Page::Home => div![
                C!["page-home"],
                h1!["EtoAl.com"],
                "I'm Home."
            ],
            Page::About => div![
                C!["page-about"],
                h1!["EtoAl.com"],
                "About me."
            ],
            Page::NotFound => div!["404"],
        },
    ]
}


// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
