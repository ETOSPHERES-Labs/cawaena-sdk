# Dynamic library for Android

This library provides a set of bindings for integrating the ETOPay SDK with Android applications. It allows developers to easily use the ETOPay functionality in their Android projects. The library supports multiple architectures including `armeabi-v7a`, `arm64-v8a`, `x86`, and `x86_64`. To generate the necessary library files, you can use the provided commands such as `cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release` or `make build_sdk`. Additionally, there are commands available for bundling the jniLibs and packaging them for export.

This library is a dynamic library and can also be used with standalone java projects.

## Supported architectures

- armeabi-v7a
- arm64-v8a
- x86
- x86_64

- cargo native JNI (for running tests locally)

## Library generation

While working in dev container run a command to generate lib for one of supported architectures.

```bash
cargo ndk -o ./jniLibs -t armeabi-v7a build --release
```

Or use the following command to generate libs for all architectures into the `jniLibs` folder:

```bash
make build_sdk
```

The `libc++_shared.so` files are packaged together with the generated *.so files from cargo.

## Running Unit tests

These android/java JNI bindings have unit tests in the `../android/tests` folder
To execute them on you local machine use:

```bash
cd bindings/android/tests
gradle test
```

This will compile the bindings for the native platform of your computer (i.e. *not* for android)
and execute the tests.

## Running Examples

These android/java JNI bindings have examples in the `/tests/src/main/java/com/etospheres/etopay/examples` folder. 

To execute them on you local machine use:

1. Navigate to the android tests directory: `cd bindings/android/tests`.
2. `gradle runAllExamples` - will run all of them.
3. `gradle {example task name}` - will run only one. Example: `gradle runCreateNewUser01`.

This will compile the bindings for the native platform of your computer (i.e. *not* for android)
and run the examples.

**Note**: These examples need environmental variables in order to run successfully. Make sure to add a `.env` in the root directory: `bindings/android/tests` with the corresponding values.

```
MNEMONIC=
MNEMONIC_HANS48=
KC_URL=
KC_REALM=
KC_CLIENT_ID=
KC_CLIENT_SECRET=
PASSWORD=
EXAMPLE_BACKEND_URL=
```
