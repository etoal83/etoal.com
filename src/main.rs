// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

extern crate etoal_com as et;

use seed::{prelude::*, *};
use seed_styles::{*, px, rem};
use seed_hooks::*;
use crate::et::about;
use crate::et::contents::works;
// use crate::et::hello_cube::hello_cube;
use crate::et::theme::*;

const ABOUT: &str = "about";
const WORKS: &str = "works";

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
    Works(works::Model),
    NotFound,
}

impl Page {
    fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::Home,
            Some(ABOUT) => Self::About,
            Some(WORKS) => works::init(url).map_or(Self::NotFound, Self::Works),
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
    pub fn works(self) -> works::Urls<'a> {
        works::Urls::new(self.base_url().add_path_part(WORKS))
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
#[topo::nested]
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model.base_url),
        match &model.page {
            Page::Home => div![C!["page-content"],
                "I'm Home.",
            ],
            Page::About => about::view(),
            Page::Works(works_model) => works::view(works_model),
            Page::NotFound => div!["404"],
        },
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    nav![
        id!["appNav"],
        div![
            C!["nav-container"],
            s().display(CssDisplay::Flex)
                .align_items(CssAlignItems::Center)
                .justify_content("space-between")
                .padding_x(px(10)),
            a![
                C!["nav-logo"],
                s().font_size(rem(2))
                    .font_weight(CssFontWeight::Bold)
                    .text_decoration(CssTextDecoration::None)
                    .padding_y(rem(0.5)),
                attrs! { At::Href => Urls::new(base_url).home() },
                "Etoarium",
            ],
            div![
                C!["nav-menu"],
                a![
                    s().padding_x(rem(0.5))
                        .padding_y(rem(0.75))
                        .text_decoration(CssTextDecoration::None),
                    attrs! { At::Href => Urls::new(base_url).home() },
                    "Home",
                ],
                a![
                    s().padding_x(rem(0.5))
                        .padding_y(rem(0.75))
                        .text_decoration(CssTextDecoration::None),
                        attrs! { At::Href => Urls::new(base_url).about() },
                    "About",
                ],
            ]
        ],
    ]
}


// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
