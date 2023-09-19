<div align="center">
<img src="https://raw.githubusercontent.com/opencanarias/public-resources/master/images/taple-logo-readme.png">
</div>

# TAPLE FFI
TAPLE (pronounced T+🍎 ['tapəl]) stands for Tracking (Autonomous) of Provenance and Lifecycle Events. TAPLE is a permissioned DLT solution for traceability of assets and processes. It is:

- **Scalable**: Scaling to a sufficient level for traceability use cases. 
- **Light**: Designed to support resource constrained devices.
- **Flexible**: Have a flexible and adaptable cryptographic scheme mechanism for a multitude of scenarios.
- **Energy-efficient**: Rust powered, TAPLE is sustainable and efficient from the point of view of energy consumption.

Foreign Function Interface (FFI) for TAPLE using [Uniffi](https://github.com/mozilla/uniffi-rs). TAPLE FFI allows to create and execute TAPLE nodes in different architectures and using different programming languages. 

[![AGPL licensed][agpl-badge]][agpl-url]

[agpl-badge]: https://img.shields.io/badge/license-AGPL-blue.svg
[agpl-url]: https://github.com/opencanarias/taple-core/blob/master/LICENSE

[Discover](https://www.taple.es/docs/discover) | [Learn](https://www.taple.es/docs/learn) | [Build](https://www.taple.es/docs/build) | [Code](https://github.com/search?q=topic%3Ataple+org%3Aopencanarias++fork%3Afalse+archived%3Afalse++is%3Apublic&type=repositories)

## Build

### Requirments
Minimium supported rust versión (MSRV) is 1.67.

### Compile
```bash
$ git clone https://github.com/opencanarias/taple-ffi.git
$ cd taple-ffi/taple-uniffi
$ sudo apt install -y libprotobuf-dev protobuf-compiler cmake
$ cargo build --release
$ mkdir ../target
$ cp ./target/release/libtaple_uniffi.a ../target
$ cp ./target/release/libtaple_uniffi.so ../target
```

### Generate bindings
```bash
$ cd ../uniffi-bindgen
$ cargo run --bin uniffi-bindgen generate ../taple-uniffi/src/taple_uniffi.udl --out-dir ../target --language kotlin
```
Any [Uniffi supported language](https://mozilla.github.io/uniffi-rs/Overview.html#supported-languages) can be used instead of Kotlin.

## Documentation and examples
Documentation and examples are under development. If you need more information, check out the following resources. 
- [Uniffi User Guide](https://mozilla.github.io/uniffi-rs/)
- TAPLE mobile ports:
  - [Android](https://github.com/opencanarias/taple-sdk-android.git)
  - [iOS](https://github.com/opencanarias/taple-sdk-ios.git)
  
## License
This project is licensed under the [AGPL license](./LICENSE).
