#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use seed_styles::{*, px, pc, rem};
use super::hello_cube::hello_cube;

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    main![
        id!["home"],
        div![
            C!["page-content"],
            s().width(CssWidth::Percentage(pc(100)))
                .max_width(CssMaxWidth::Length(px(800)))
                .margin_x("auto")
                .margin_top(CssMarginTop::Length(rem(1)))
            ,
            div![
                s().text_align_center()
                    .font_size(rem(1.1)),
                p!["Welcome to EtoAl's personal website."],
                p!["The whole of this site is powered by WebAssembly compiled from Rust!"],
            ],
        ],
        hello_cube(),
    ]
}
