use oxc_allocator::Allocator;
use oxc_formatter::JsFormatOptions;
use oxc_formatter_core::LineWidth;
use oxc_span::SourceType;

const DEFAULT_OXFMT_LINE_WIDTH: u16 = 80;

pub(super) fn format_ts(code: &str) -> std::result::Result<String, String> {
    let line_width = LineWidth::try_from(DEFAULT_OXFMT_LINE_WIDTH)
        .map_err(|err| format!("Invalid Oxc line width: {err}"))?;
    let allocator = Allocator::default();
    let options = JsFormatOptions {
        line_width,
        ..JsFormatOptions::default()
    };
    let formatted = oxc_formatter::format(&allocator, code, SourceType::ts(), options, None)
        .map_err(|err| format!("Failed to format TypeScript: {err}"))?;
    let printed = formatted
        .print()
        .map_err(|err| format!("Failed to print TypeScript: {err}"))?;

    Ok(printed.into_code())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::tests::*;

    #[test]
    fn formats_typescript() {
        let formatted = format_ts("const value={first:1,second:2,third:3};").unwrap();

        assert_snapshot!(formatted, @"const value = { first: 1, second: 2, third: 3 };");
    }

    #[test]
    fn rejects_invalid_typescript() {
        assert!(format_ts("const =").is_err());
    }

    #[test]
    fn formats_complex_typescript() {
        let formatted = format_ts(indoc! {r#"
            export type PokemonId = string & { [pokemonIdBrand]: true };
                declare const pokemonIdBrand: unique symbol;

            export type PokemonType = | "grass"
            | "fire" | "water"
                | "lightning"
                | "psychic"
            | "fighting"
                | "darkness"
                | "metal"
                | "fairy"
            | "dragon"
            | "colorless";

            export type PokemonStage = PokemonStageBasic | PokemonStage1 | PokemonStage2;

            export interface PokemonStageBasic {
                                    kind: "basic";
            }

            export interface PokemonStage1 {
                        kind: "1";
                evolvesFrom: PokemonId;
            }

            export interface PokemonStage2 {
            kind: "2";
                evolvesFrom: PokemonId;
            }


        "# })
        .unwrap();

        assert_snapshot!(formatted, @r#"
        export type PokemonId = string & { [pokemonIdBrand]: true };
        declare const pokemonIdBrand: unique symbol;

        export type PokemonType =
          | "grass"
          | "fire"
          | "water"
          | "lightning"
          | "psychic"
          | "fighting"
          | "darkness"
          | "metal"
          | "fairy"
          | "dragon"
          | "colorless";

        export type PokemonStage = PokemonStageBasic | PokemonStage1 | PokemonStage2;

        export interface PokemonStageBasic {
          kind: "basic";
        }

        export interface PokemonStage1 {
          kind: "1";
          evolvesFrom: PokemonId;
        }

        export interface PokemonStage2 {
          kind: "2";
          evolvesFrom: PokemonId;
        }
        "#);
    }
}
