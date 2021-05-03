// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use seed_styles::*;
use seed_styles::{px, pc, rem};
// use std::fmt;
// use serde::{Serialize, Deserialize};

const ABOUT: &str = "about";

// ------ ------
//     Init
// ------ ------

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    GlobalStyle::default()
        .style(
            "a,ul,li,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
            s().font_family("'Helvetica Neue',Arial,'Hiragino Kaku Gothic ProN','Hiragino Sans',Meiryo,sans-serif")
                .color(rgb(217, 217, 217))
                .webkit_font_smoothing_antialiased(),
        )
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        .style("body", s().background_color(rgb(0, 0, 0)))
        .activate_styles();

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
            Page::Home => div![C!["page-home"],
                "I'm Home.",
            ],
            Page::About => div![C!["page-about"],
                "About me."
            ],
            Page::NotFound => div!["404"],
        },
        hello_dice(),
    ]
}

fn header(base_url: &Url) -> Node<Msg> {
    div![
        C!["header"],
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

fn hello_dice() -> Node<Msg> {
    div![C!["dice-container"],
        s().position_absolute()
            .top("50%")
            .left("50%")
            .perspective("500px")
        ,
        div![C!["dice"],
            s().transform_style("preserve-3d")
                .animation("10000ms linear infinite")
                .keyframe(0, s().transform("rotateX(0deg) rotateY(0deg)"))
                .keyframe(100, s().transform("rotateX(720deg) rotateY(360deg)"))
            ,
            ol![C!["dice-list"],
                s().transform_style("preserve-3d")
                    .transform("translateY(-85px) translateX(-85px)")
                ,
                dice_surface(1),
                dice_surface(2),
                dice_surface(3),
                dice_surface(4),
                dice_surface(5),
                dice_surface(6),
            ],
        ],
    ]
}

fn dice_surface(id: i32) -> Node<Msg> {
    let base_style = s()
        .position_absolute()
        .width(px(150))
        .height(px(150))
        .background_color(CssColor::Rgba(150., 100., 255., 0.6))
        .display(CssDisplay::Flex)
        .justify_content(CssJustifyContent::Center)
        .align_items(CssAlignItems::Center)
        .font_size(rem(6))
        .color(CssColor::Rgba(255., 255., 255., 0.6))
        .text_shadow("0px 0px 10px rgba(255, 255, 255, 0.4)")
        .border_radius(CssBorderRadius::Length(px(10)))
        .box_shadow("0px 0px 15px rgba(200, 150, 255, 0.4)")
        ;
    let (s_char, s_style) = match id {
        1 => ("H", base_style.clone().transform("translateZ(85px)")),
        2 => ("E", base_style.clone().transform("translateY(85px) rotateX(270deg)")),
        3 => ("L", base_style.clone().transform("translateX(85px) rotateX(180deg) rotateY(90deg)")),
        4 => ("💚", base_style.clone().transform("translateX(-85px) rotateX(180deg) rotateY(-90deg)")),
        5 => ("L", base_style.clone().transform("translateY(-85px) rotateY(180deg) rotateX(90deg)")),
        6 => ("O", base_style.clone().font_size(rem(5)).transform("translateZ(-85px) rotateY(180deg)")),
        _ => ("_", base_style),
    };
    li![
        attrs! { At::from("surface-id") => id },
        s_style,
        s_char,
    ]
}


// ------ ------
//     Start
// ------ ------

pub fn main() {
    App::start("app", init, update, view);
}
