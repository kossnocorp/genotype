use crate::prelude::internal::*;

ui_select_options!(StarterResponse {
    Blank => "Blank",
    Tour => "Language tour",
    Demo => "Demo project",
});

impl StarterResponse {
    pub fn files(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Tour => STARTER_TOUR_FILES,
            Self::Demo => STARTER_DEMO_FILES,
            Self::Blank => &[],
        }
    }
}

const STARTER_TOUR_FILES: &[(&str, &str)] = &[
    (
        "guide.type",
        include_str!("../../../examples/guide/guide.type"),
    ),
    (
        "module.type",
        include_str!("../../../examples/guide/module.type"),
    ),
];

const STARTER_DEMO_FILES: &[(&str, &str)] = &[
    (
        "card.type",
        include_str!("../../../examples/demo/card.type"),
    ),
    (
        "pokemon.type",
        include_str!("../../../examples/demo/pokemon.type"),
    ),
];
