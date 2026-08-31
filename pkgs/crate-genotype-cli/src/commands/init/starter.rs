use crate::prelude::internal::*;

ui_select_options!(StarterResponse {
    Blank => "Blank",
    Tour => "Language tour",
    Demo => "Demo project",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StarterTipBlock {
    pub title: &'static str,
    pub code: &'static str,
}

impl StarterResponse {
    pub fn files(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Tour => STARTER_TOUR_FILES,
            Self::Demo => STARTER_DEMO_FILES,
            Self::Blank => &[],
        }
    }

    pub fn tip_block(self) -> Option<StarterTipBlock> {
        match self {
            Self::Tour => Some(StarterTipBlock {
                title: "See the language tour source file",
                code: r#""$EDITOR" src/guide.type"#,
            }),
            Self::Blank | Self::Demo => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tour_tip_block() {
        assert_eq!(
            StarterResponse::Tour.tip_block(),
            Some(StarterTipBlock {
                title: "See the language tour source file",
                code: r#""$EDITOR" src/guide.type"#,
            })
        );
    }

    #[test]
    fn other_starters_have_no_tip_block() {
        assert_eq!(StarterResponse::Blank.tip_block(), None);
        assert_eq!(StarterResponse::Demo.tip_block(), None);
    }
}
