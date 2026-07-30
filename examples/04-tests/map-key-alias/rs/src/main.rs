fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use genotype_test_map_key_alias_types::{Address, AddressId, User};
    use serde_json::{from_value, json, to_value};

    #[test]
    fn branded_map_key_roundtrip() {
        let user = User {
            addresses: BTreeMap::from([(
                AddressId("home".into()),
                Address {
                    street: "Main Street".into(),
                },
            )]),
        };

        let value = to_value(&user).expect("serialize user");
        assert_eq!(
            value,
            json!({"addresses": {"home": {"street": "Main Street"}}})
        );

        let decoded: User = from_value(value).expect("deserialize user");
        assert_eq!(decoded, user);
    }
}
