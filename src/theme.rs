use seed_styles::*;

// ------ ------
//    Global
// ------ ------

pub fn init_global_styles() {
    GlobalStyle::default()
        .style(
            "a,ul,li,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
            s().font_family("'Helvetica Neue',Arial,'Hiragino Kaku Gothic ProN','Hiragino Sans',Meiryo,sans-serif")
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
