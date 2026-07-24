use syn::spanned::Spanned;

pub(super) fn format_rs(code: &str) -> std::result::Result<String, String> {
    let file = syn::parse_file(code).map_err(|err| format!("Failed to parse Rust: {err}"))?;
    let blank_lines = code
        .lines()
        .map(|line| line.trim().is_empty())
        .collect::<Vec<_>>();
    let blank_separators = file
        .items
        .windows(2)
        .map(|items| has_blank_line_between(&blank_lines, &items[0], &items[1]))
        .collect::<Vec<_>>();
    let syn::File {
        shebang,
        frontmatter,
        attrs,
        items,
    } = file;

    if items.is_empty() {
        return Ok(prettyplease::unparse(&syn::File {
            shebang,
            frontmatter,
            attrs,
            items,
        }));
    }

    let mut shebang = shebang;
    let mut frontmatter = frontmatter;
    let mut attrs = attrs;
    let mut formatted = String::new();

    for (index, item) in items.into_iter().enumerate() {
        if index > 0 {
            formatted.push('\n');
            if blank_separators[index - 1] {
                formatted.push('\n');
            }
        }

        let item_file = syn::File {
            shebang: if index == 0 { shebang.take() } else { None },
            frontmatter: if index == 0 { frontmatter.take() } else { None },
            attrs: if index == 0 {
                std::mem::take(&mut attrs)
            } else {
                Vec::new()
            },
            items: vec![item],
        };
        let item_formatted = prettyplease::unparse(&item_file);
        formatted.push_str(item_formatted.trim_end_matches('\n'));
    }

    formatted.push('\n');
    Ok(formatted)
}

fn has_blank_line_between(blank_lines: &[bool], previous: &syn::Item, next: &syn::Item) -> bool {
    let previous_end_line = previous.span().end().line;
    let next_start_line = next.span().start().line;
    let gap_end = next_start_line.saturating_sub(1);

    blank_lines
        .get(previous_end_line..gap_end)
        .is_some_and(|lines| lines.iter().any(|is_blank| *is_blank))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::tests::*;

    #[test]
    fn formats_rust() {
        let formatted = format_rs("pub struct User{pub id:u64,pub name:String}").unwrap();

        assert_snapshot!(formatted, @"
        pub struct User {
            pub id: u64,
            pub name: String,
        }
        ");
    }

    #[test]
    fn preserves_and_collapses_top_level_rust_blank_lines() {
        let formatted =
            format_rs("pub struct First;\n\n\n\npub struct Second;\npub struct Third;\n").unwrap();

        assert_snapshot!(formatted, @"
        pub struct First;

        pub struct Second;
        pub struct Third;
        ");
    }

    #[test]
    fn preserves_rust_blank_line_before_attributed_item() {
        let formatted =
            format_rs("pub struct First;\n\n#[derive(Debug)]\npub struct Second;\n").unwrap();

        assert_snapshot!(formatted, @"
        pub struct First;

        #[derive(Debug)]
        pub struct Second;
        ");
    }

    #[test]
    fn rejects_invalid_rust() {
        assert!(format_rs("pub struct {").is_err());
    }

    #[test]
    fn formats_complex_rust() {
        let formatted = format_rs(indoc! {r#"
            use serde::{Deserialize,   Serialize  };
            use litty::serde_literals;

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            pub struct PokemonId(  pub String   );

            #[serde_literals]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            pub enum PokemonType {
                    #[literal("grass")]
                    Grass,
                    #[literal("fire")]
                Fire,
                #[literal("water")]
                Water,
                        #[literal("lightning")]
                Lightning,
                #[literal("psychic")]
                Psychic,
                #[literal("fighting")]
                Fighting,
                #[literal("darkness")]
                Darkness,
                #[literal("metal")]
                Metal,
                #[literal("fairy")]
                Fairy,
                #[literal("dragon")]
                Dragon,
                #[literal("colorless")]
                Colorless,
            }

            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[serde(untagged)]
            pub enum PokemonStage { Basic(PokemonStageBasic), Lit1(PokemonStage1), Lit2(PokemonStage2),
            }

            #[serde_literals]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[literals(kind = "basic")]
            pub struct PokemonStageBasic {}

            #[serde_literals]
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[literals(kind = "1")]
            pub struct PokemonStage1 {
                #[serde(rename = "evolvesFrom")]
                pub evolves_from: PokemonId,
            }

                #[serde_literals]
                #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
                #[literals(kind = "2")]
                pub struct PokemonStage2 {
                        #[serde(rename = "evolvesFrom")]
                        pub evolves_from: PokemonId,
                }

        "# })
        .unwrap();

        assert_snapshot!(formatted, @r#"
        use serde::{Deserialize, Serialize};
        use litty::serde_literals;

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct PokemonId(pub String);

        #[serde_literals]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub enum PokemonType {
            #[literal("grass")]
            Grass,
            #[literal("fire")]
            Fire,
            #[literal("water")]
            Water,
            #[literal("lightning")]
            Lightning,
            #[literal("psychic")]
            Psychic,
            #[literal("fighting")]
            Fighting,
            #[literal("darkness")]
            Darkness,
            #[literal("metal")]
            Metal,
            #[literal("fairy")]
            Fairy,
            #[literal("dragon")]
            Dragon,
            #[literal("colorless")]
            Colorless,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum PokemonStage {
            Basic(PokemonStageBasic),
            Lit1(PokemonStage1),
            Lit2(PokemonStage2),
        }

        #[serde_literals]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[literals(kind = "basic")]
        pub struct PokemonStageBasic {}

        #[serde_literals]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[literals(kind = "1")]
        pub struct PokemonStage1 {
            #[serde(rename = "evolvesFrom")]
            pub evolves_from: PokemonId,
        }

        #[serde_literals]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[literals(kind = "2")]
        pub struct PokemonStage2 {
            #[serde(rename = "evolvesFrom")]
            pub evolves_from: PokemonId,
        }
        "#);
    }
}
