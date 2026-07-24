use ruff_python_formatter::{PyFormatOptions, format_module_source};

pub(super) fn format_py(code: &str) -> std::result::Result<String, String> {
    format_module_source(code, PyFormatOptions::default())
        .map(|printed| printed.into_code())
        .map_err(|err| format!("Failed to format Python: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::tests::*;

    #[test]
    fn formats_python() {
        let formatted = format_py("value={'first':1,'second':2}\n").unwrap();

        assert_snapshot!(formatted, @r#"value = {"first": 1, "second": 2}"#);
    }

    #[test]
    fn rejects_invalid_python() {
        assert!(format_py("value = (").is_err());
    }

    #[test]
    fn formats_pokemon_demo() {
        let formatted = format_py(indoc! {r#"
            from __future__ import annotations

            from pydantic import Field
            from typing import Optional, Literal
            from genotype import Model
            from .pokemon import PokemonType


            class CardPokemon(CardBase, Model):
                kind: Literal["pokemon"]
                hp: float
                types: list[PokemonType]
            class CardTrainer(CardBase, Model):
                kind: Literal["trainer"]


            class CardEnergy(CardBase, Model):
                kind: Literal["energy"]

            type Card = CardPokemon | CardTrainer | CardEnergy


            type CardRarity = (
                Literal["common"]
                    | Literal["uncommon"]
                    | Literal["rare"]
                | Literal["rare-holo"]
                | Literal["double-rare"]
                | Literal["ultra-rare"]
                | Literal["illustration-rare"]
                | Literal["special-illustration-rare"]
                | Literal["hyper-rare"]
                | Literal["promo"]
            )


            type CardRegulationMark = (
                Literal["D"]
                | Literal["E"]
                | Literal["F"]
                | Literal["G"]
                | Literal["H"]
                | Literal["I"]
            )
            class CardBase(Model):
                rarity: CardRarity
                regulation_mark: Optional[CardRegulationMark] = Field(
                    default=None, alias="regulationMark"
                )



        "# })
        .unwrap();

        assert_snapshot!(formatted, @r#"
        from __future__ import annotations

        from pydantic import Field
        from typing import Optional, Literal
        from genotype import Model
        from .pokemon import PokemonType


        class CardPokemon(CardBase, Model):
            kind: Literal["pokemon"]
            hp: float
            types: list[PokemonType]


        class CardTrainer(CardBase, Model):
            kind: Literal["trainer"]


        class CardEnergy(CardBase, Model):
            kind: Literal["energy"]


        type Card = CardPokemon | CardTrainer | CardEnergy


        type CardRarity = (
            Literal["common"]
            | Literal["uncommon"]
            | Literal["rare"]
            | Literal["rare-holo"]
            | Literal["double-rare"]
            | Literal["ultra-rare"]
            | Literal["illustration-rare"]
            | Literal["special-illustration-rare"]
            | Literal["hyper-rare"]
            | Literal["promo"]
        )


        type CardRegulationMark = (
            Literal["D"]
            | Literal["E"]
            | Literal["F"]
            | Literal["G"]
            | Literal["H"]
            | Literal["I"]
        )


        class CardBase(Model):
            rarity: CardRarity
            regulation_mark: Optional[CardRegulationMark] = Field(
                default=None, alias="regulationMark"
            )
        "#);
    }
}
