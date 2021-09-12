#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use seed_styles::{*, px, pc, rem};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    main![
        id!["aboutMe"],
        div![
            C!["page-content"],
            s().width(CssWidth::Percentage(pc(100)))
                .max_width(CssMaxWidth::Length(px(800)))
                .margin_x("auto")
                .margin_top(CssMarginTop::Length(rem(1)))
            ,
            h1!["About"],
            view_summary(),
            view_profile(),
        ],
    ]
}

fn view_summary<Ms>() -> Node<Ms> {
    div![
        C!["avator-container"],
        s().display(CssDisplay::Flex)
            .justify_content(CssJustifyContent::Center)
            .align_items(CssAlignItems::Center)
        ,
        img![
            C!["avator-image"],
            s().width(CssWidth::Length(px(200)))
                .height(CssHeight::Length(px(200)))
                .border_radius(CssBorderRadius::Percentage(pc(50)))
                .margin_x(px(24))
            ,
            attrs!{
                At::Src => "/public/images/icon_etoal_400x400.png",
                At::Alt => "An odd-eyed girl with black and short-bob hair."
            },
        ],
        div![
            s().margin_x(px(24)),
            div![
                C!["nickname"],
                s().font_size(rem(1.2))
                    .margin_y(rem(1))
                ,
                "EtoAl / えとある"
            ],
            div![
                C!["hitokoto"],
                "好奇心の塊。変幻自在な生きものになりたい。"
            ]
        ],
    ]
}

fn view_profile<Ms>() -> Node<Ms> {
    div![
        C!["profile-container"],
        empty![],
    ]
}
