use seed_styles::*;

// ------ ------
//     Color
// ------ ------

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Color {
    Background,
    MainText,
    Primary,
    Secondary,
}

impl ColorTheme for Color {} // Allows you to use a `Color` variant as a CssColor alias in the theme.


// ------ ------
//     Style
// ------ ------

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum MyStyles {
    BodyText,
}

impl StyleTheme for MyStyles {} // Allows you to use a `MyStyles` variant as a Style alias in the theme.


// ------ ------
//     Theme
// ------ ------

pub fn dimmed_dark_theme() -> Theme {
    Theme::new("dimmed_dark")
        .set_color(Color::Background, rgb(34, 39, 46))
        .set_color(Color::MainText, rgb(217, 217, 217))
        .set_color(Color::Primary, rgb(146, 102, 204))
        .set_color(Color::Secondary, rgb(120, 177, 89))
        .set_style(
            MyStyles::BodyText,
            s().letter_spacing(rem(0.1)),
        )
}


// ------ ------
//    Global
// ------ ------

pub fn init_global_styles() {
    GlobalStyle::default()
        .style(
            "a,ul,li,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
            s().font_family("'Helvetica Neue', Arial, 'Hiragino Kaku Gothic ProN', 'Hiragino Sans', Meiryo, sans-serif")
                .color(rgb(217, 217, 217))
                .webkit_font_smoothing_antialiased(),
        )
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        .activate_init_styles();
}

pub fn use_themed_styles() {
    GlobalStyle::default()
        .style("body", s().background_color(rgb(0, 0, 0)))
        .activate_styles();
}

// ------ ------
//  Components
// ------ ------

pub fn nav_container_styles() -> Style {
    s().display(CssDisplay::Flex)
        .align_items(CssAlignItems::Center)
        .justify_content("space-between")
        .padding_x(px(10))
}

pub fn nav_logo_styles() -> Style {
    s().font_size(rem(2.5))
        .font_family("'Futura', 'Century Gothic', 'CenturyGothic', 'AppleGothic', sans-serif")
        .font_weight(CssFontWeight::Lighter)
        .text_decoration(CssTextDecoration::None)
        .padding_y(rem(0.5))
        .text_shadow(CssTextShadow::Shadow(px(0), px(0), px(0), "transparent".to_string()))
        .transition("text-shadow 0.3s")
}

pub fn nav_logo_hover_styles() -> Style {
    s().hover().text_shadow(CssTextShadow::Shadow(px(0), px(0), px(10), "#999".to_string()))
}

pub fn nav_menu_styles() -> Style {
    s().padding_x(rem(1))
        .padding_y(rem(0.75))
        .text_decoration(CssTextDecoration::None)
}

pub fn nav_menu_hover_styles() -> Style {
    s().hover().background_color(CssColor::Rgba(255., 255., 255., 0.2))
}