<div align="center">
<img src="https://raw.githubusercontent.com/opencanarias/public-resources/master/images/taple-logo-readme.png">
</div>

# TAPLE FFI
TAPLE (pronounced T+🍎 ['tapəl]) stands for Tracking (Autonomous) of Provenance and Lifecycle Events. TAPLE is a permissioned DLT solution for traceability of assets and processes. It is:

- **Scalable**: Scaling to a sufficient level for traceability use cases. 
- **Light**: Designed to support resource constrained devices.
- **Flexible**: Have a flexible and adaptable cryptographic scheme mechanism for a multitude of scenarios.
- **Energy-efficient**: Rust powered, TAPLE is sustainable and efficient from the point of view of energy consumption.

TAPLE FFI allows the execution of TAPLE nodes in environments and languages other than Rust.

[![AGPL licensed][agpl-badge]][agpl-url]

[agpl-badge]: https://img.shields.io/badge/license-AGPL-blue.svg
[agpl-url]: https://github.com/opencanarias/taple-core/blob/master/LICENSE

[Discover](https://www.taple.es/docs/discover) | [Learn](https://www.taple.es/docs/learn) | [Build](https://www.taple.es/docs/build) | 
[Code](https://github.com/search?q=topic%3Ataple+org%3Aopencanarias++fork%3Afalse+archived%3Afalse++is%3Apublic&type=repositories)

## Build
Minimium supported rust versión (MSRV) is 1.67.

```bash
$ git clone https://github.com/opencanarias/taple-ffi.git
$ cd taple-ffi
$ sudo apt install -y libprotobuf-dev protobuf-compiler cmake
$ cargo install cross --git https://github.com/cross-rs/cross
$ ./scripts/android/setup-android.sh
$ ./scripts/android/start-android.sh
```

**NOTE**: Rust Nightly is needed for iOS compilation. In this case, the last two commands must be replaced:

```bash
$ ./scripts/ios/setup-ios.sh
$ ./scripts/ios/start-ios.sh
```

## Usage
Examples of how to use the resulting SDK can be found in these repositories:

- Android SDK: https://github.com/opencanarias/taple-sdk-android.git
- iOS SDK: https://github.com/opencanarias/taple-sdk-ios.git

## License
This project is licensed under the [AGPL license](./LICENSE).
