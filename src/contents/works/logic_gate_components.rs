use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    div![C!["page-content"],
        "Logic gate components."
    ]
}
