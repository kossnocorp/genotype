# Code Comments

## References

### `!CMNT_THISERROR_SOURCE`

`thiserror` treats the `source` field as if it were annotated with `#[source]`, so to escape this behavior, we use `r#source` instead. [See @dtolnay's comment](https://github.com/dtolnay/thiserror/issues/284#issuecomment-2455765945).
