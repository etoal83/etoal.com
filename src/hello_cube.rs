use seed::{prelude::*, *};
use seed_styles::*;
use seed_styles::{px, rem};

// ------ ------
//     View
// ------ ------

pub fn hello_cube<Ms>() -> Node<Ms> {
    div![C!["cube-container"],
        s().position_absolute()
            .top("50%")
            .left("50%")
            .perspective("500px")
        ,
        div![C!["cube"],
            s().transform_style("preserve-3d")
                .animation("10000ms linear infinite")
                .keyframe(0, s().transform("rotateX(0deg) rotateY(0deg)"))
                .keyframe(100, s().transform("rotateX(720deg) rotateY(360deg)"))
            ,
            ol![C!["cube-list"],
                s().transform_style("preserve-3d")
                    .transform("translateY(-85px) translateX(-85px)")
                ,
                cube_surface(1),
                cube_surface(2),
                cube_surface(3),
                cube_surface(4),
                cube_surface(5),
                cube_surface(6),
            ],
        ],
    ]
}

fn cube_surface<Ms>(id: i32) -> Node<Ms> {
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
        4 => ("💚", base_style.clone().font_size(rem(5)).transform("translateX(-85px) rotateX(180deg) rotateY(-90deg)")),
        5 => ("L", base_style.clone().transform("translateY(-85px) rotateY(180deg) rotateX(90deg)")),
        6 => ("O", base_style.clone().transform("translateZ(-85px) rotateY(180deg)")),
        _ => ("_", base_style),
    };
    li![
        attrs! { At::from("surface-id") => id },
        s_style,
        s_char,
    ]
}
