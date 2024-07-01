# Genotype

🚧 Work in progress

## Design

### Principles

The design principles are presented in the order of importance.

#### Unambiguity

Both the syntax and the described structures should be unambiguous.

While the syntax should limit ambiguity as much as possible, complete unambiguity should be achieved by the type system.

#### Portability

Data structures should be portable and representable in JSON.

#### Transparency

Genotype data structures should visually map to the data representation. It should be obvious what the JSON would look like.

#### Compatibility

It should be possible to express any structure in any target language. This means that not all target language structures can be necessarily expressed in Genotype, but any Genotype structure can be expressed in any target language.

#### Idiomatic

The resulting target language code should be idiomatic and consider the context where the target language is used.

#### Expressiveness

Genotype aims to be as expressive as possible while maintaining idiomaticity and compatibility with the target languages.

#### Practicality

Genotype should not be dogmatic and lean into practicality when necessary.

#### Longevity

The target language code should stay relevant for as long as possible. This means limiting the use of dependencies.
