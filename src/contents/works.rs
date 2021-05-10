use seed::{prelude::*, *};

mod logic_gate_components;

const LOGIC_GATE_COMPONENTS: &str = "logic-gate-components";


// ------ ------
//     Init
// ------ ------

pub fn init(mut url: Url) -> Option<Model> {
    let work = match url.remaining_path_parts().as_slice() {
        [LOGIC_GATE_COMPONENTS] => Work::LogicGateComponents,
        _ => None?,
    };

    Some(Model {
        work,
    })
}


// ------ ------
//     Model
// ------ ------

pub struct Model {
    work: Work,
}

#[derive(Debug)]
enum Work {
    LogicGateComponents,
    NotFound,
}


// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn work_urls(self) -> Url {
        self.base_url().add_path_part("aaa")
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms>(model: &Model) -> Node<Ms> {
    match &model.work {
        Work::LogicGateComponents => logic_gate_components::view(),
        _ => div!["404"]
    }
}