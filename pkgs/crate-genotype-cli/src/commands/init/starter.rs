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
        "coordinates.type",
        include_str!("../../../examples/guide/coordinates.type"),
    ),
    (
        "destination.type",
        include_str!("../../../examples/guide/destination.type"),
    ),
    (
        "guide.type",
        include_str!("../../../examples/guide/guide.type"),
    ),
    (
        "location.type",
        include_str!("../../../examples/guide/location.type"),
    ),
    (
        "module.type",
        include_str!("../../../examples/guide/module.type"),
    ),
    (
        "place.type",
        include_str!("../../../examples/guide/place.type"),
    ),
    (
        "venue.type",
        include_str!("../../../examples/guide/venue.type"),
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
