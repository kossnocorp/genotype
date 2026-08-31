use crate::prelude::internal::*;

ui_select_options!(StarterResponse {
    Blank => "Blank",
    Tour => "Language tour",
    Demo => "Demo project",
});

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarterTipBlock {
    pub title: &'static str,
    pub code: String,
}

impl StarterResponse {
    pub fn files(self) -> &'static [(&'static str, &'static str)] {
        match self {
            Self::Tour => STARTER_TOUR_FILES,
            Self::Demo => STARTER_DEMO_FILES,
            Self::Blank => &[],
        }
    }

    pub fn tip_block(self, success_path: &str) -> Option<StarterTipBlock> {
        match self {
            Self::Tour => Some(StarterTipBlock {
                title: "See the language tour source file",
                code: editor_command(success_path, cfg!(windows)),
            }),
            Self::Blank | Self::Demo => None,
        }
    }
}

fn editor_command(success_path: &str, windows: bool) -> String {
    let path = project_file(success_path, "src/guide.type", windows);
    if windows {
        format!("& $env:EDITOR {}", powershell_quote(&path))
    } else {
        format!(r#""$EDITOR" {}"#, posix_shell_quote(&path))
    }
}

fn project_file(project_path: &str, file: &str, windows: bool) -> String {
    let project_path = project_path.trim_end_matches(['/', '\\']);
    let file = if windows {
        file.replace('/', "\\")
    } else {
        file.into()
    };
    match project_path {
        "" | "." => file,
        path if windows => format!("{}\\{file}", path.replace('/', "\\")),
        path => format!("{path}/{file}"),
    }
}

fn posix_shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || matches!(char, '/' | '.' | '_' | '-'))
    {
        value.into()
    } else {
        format!("'{}'", value.replace('\'', r#"'"'"'"#))
    }
}

fn powershell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || matches!(char, '/' | '\\' | '.' | '_' | '-'))
    {
        value.into()
    } else {
        format!("'{}'", value.replace('\'', "''"))
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
    use std::fs;
    use tempfile::tempdir_in;

    #[test]
    fn tour_tip_block() {
        let tip = StarterResponse::Tour.tip_block("./").unwrap();
        assert_eq!(tip.title, "See the language tour source file");
        assert_eq!(tip.code, editor_command("./", cfg!(windows)));
    }

    #[test]
    fn tour_tip_block_editor_commands() {
        assert_eq!(editor_command("./", false), r#""$EDITOR" src/guide.type"#);
        assert_eq!(
            editor_command("my project's types", false),
            r#""$EDITOR" 'my project'"'"'s types/src/guide.type'"#
        );
        assert_eq!(
            editor_command("./", true),
            r#"& $env:EDITOR src\guide.type"#
        );
        assert_eq!(
            editor_command("my project's types", true),
            r#"& $env:EDITOR 'my project''s types\src\guide.type'"#
        );
    }

    #[test]
    fn other_starters_have_no_tip_block() {
        assert_eq!(StarterResponse::Blank.tip_block("./"), None);
        assert_eq!(StarterResponse::Demo.tip_block("./"), None);
    }

    #[test]
    fn tour_starter_builds_for_all_targets() {
        let tempdir = tempdir_in(".").unwrap();
        let project_dir = tempdir.path();
        let src_dir = project_dir.join("src");
        fs::create_dir(&src_dir).unwrap();

        for (name, contents) in StarterResponse::Tour.files() {
            fs::write(src_dir.join(name), contents).unwrap();
        }

        fs::write(
            project_dir.join(GTCONFIG_FILE),
            "[ts]\nenabled = true\npackage = false\n\
             [py]\nenabled = true\npackage = false\n\
             [rs]\nenabled = true\npackage = false\n",
        )
        .unwrap();

        let project_path = project_dir.to_string_lossy();
        let base_path: GtpCwdRelativeOrAbsoluteStringPath = project_path.as_ref().into();
        let meta = block_on(GtcSystem::compile_once((&base_path, None))).unwrap();

        assert_eq!(meta.exit_code, 0);
    }
}
