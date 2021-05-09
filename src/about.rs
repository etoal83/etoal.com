#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>() -> Node<Ms> {
    div![C!["page-content"],
        "About me: nothing."
    ]
}
