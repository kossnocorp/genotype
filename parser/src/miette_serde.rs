use miette::NamedSource;
use serde::Serializer;
use serde::ser::SerializeStruct;

pub fn serialize_named_source<S>(
    source_code: &NamedSource<String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut state = serializer.serialize_struct("NamedSource", 3)?;
    state.serialize_field("name", source_code.name())?;
    state.serialize_field("source", source_code.inner())?;
    state.serialize_field("language", &None::<String>)?;
    state.end()
}
