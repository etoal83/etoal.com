#![allow(clippy::wildcard_imports)]

extern crate etoal_com as et;

use seed::{prelude::*, *};
use seed_styles::*;
use seed_hooks::*;
use crate::et::{home, about};
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

#[topo::nested]
fn view(model: &Model) -> impl IntoNodes<Msg> {
    vec![
        header(&model.base_url),
        match &model.page {
            Page::Home => home::view(),
            Page::About => about::view(),
            Page::NotFound => div!["404"],
        },
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    nav![
        id!["appNav"],
        div![
            C!["nav-container"],
            nav_container_styles(),
            a![
                C!["nav-logo"],
                nav_logo_styles(),
                nav_logo_hover_styles(),
                attrs! { At::Href => Urls::new(base_url).home() },
                "Etoarium",
            ],
            div![
                C!["nav-menu"],
                a![
                    nav_menu_styles(),
                    nav_menu_hover_styles(),
                    attrs! { At::Href => Urls::new(base_url).home() },
                    "Home",
                ],
                a![
                    nav_menu_styles(),
                    nav_menu_hover_styles(),
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
