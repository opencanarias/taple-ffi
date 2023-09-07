# Proyecto Uniffi

[URL DE TIPOS](https://mozilla.github.io/uniffi-rs/udl/builtin_types.html)

| Rust type          | UDL type             | Notes                                                           |
| ------------------ | -------------------- | --------------------------------------------------------------- |
| bool               | boolean              |                                                                 |
| u8/i8..u64/i64     | u8/i8..u64/i64       |                                                                 |
| f32                | float                |                                                                 |
| f64                | double               |                                                                 |
| String             | string               |                                                                 |
| Vec<u8>            | bytes                | Different from sequence<u8> only in foreign type mappings       |
| SystemTime         | timestamp            | Precision may be lost when converting to Python and Swift types |
| Duration           | duration             | Precision may be lost when converting to Python and Swift types |
| &T                 | [ByRef] T            | This works for &str and &[T]                                    |
| Option<T>          | T?                   |                                                                 |
| Vec<T>             | sequence<T>          |                                                                 |
| HashMap<String, T> | record<DOMString, T> | Only string keys are supported                                  |
| ()                 | void                 | Empty return                                                    |
| Result<T, E>       | N/A                  | See Errors section                                              |
