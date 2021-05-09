// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

extern crate etoal_com as et;

use seed::{prelude::*, *};
use seed_styles::*;
use crate::et::about;
use crate::et::hello_cube::hello_cube;
use crate::et::theme::*;

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    init_global_styles();
    use_themed_styles();
    
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_base_url(),
        page: Page::init(url),
    }
}


// ------ ------
//     Model
// ------ ------

struct Model {
    base_url: Url,
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
        header(&model.base_url),
        match &model.page {
            Page::Home => div![C!["page-content"],
                "I'm Home.",
            ],
            Page::About => about::view(),
            Page::NotFound => div!["404"],
        },
        hello_cube(),
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    div![C!["header"],
        h1!["EtoAl.com"],
        ul![
            li![a![
                attrs! { At::Href => Urls::new(base_url).home() },
                "Home",
            ]],
            li![a![
                attrs! { At::Href => Urls::new(base_url).about() },
                "About",
            ]],
        ],
    ]
}


// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
