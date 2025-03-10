# Dynamic library for Android

This library provides a set of bindings for integrating the Cawaena SDK with Android applications. It allows developers to easily use the Cawaena functionality in their Android projects. The library supports multiple architectures including `armeabi-v7a`, `arm64-v8a`, `x86`, and `x86_64`. To generate the necessary library files, you can use the provided commands such as `cargo ndk -t armeabi-v7a -t arm64-v8a -t x86 -t x86_64 build --release` or `make build_sdk`. Additionally, there are commands available for bundling the jniLibs and packaging them for export.

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

To build a jniLibs bundle for testing in android, use the following command:

```bash
make bundle_libs
```

The `libc++_shared.so` files are packaged together with the generated *.so files from cargo. These files are required by the corresponding system binary loader to correctly load the _*.so_ files from the jniLibs folder using the convention:

```Java
System.loadLibrary("library_name");
```

The library should be named with the prefix `lib` and the suffix `.so`. This is how the libraries are created by cargo when specifying the `crate-type` as `cdylib`. If the `libc++_shared.so` files for the corresponding architecture are not included, loading the library works a bit differently, however this method is not portable across different platforms.

```Java
System.load("absolute/path/to/lib/with/library_name.so");
```

## Creating a java wrapper

To support integration in java via a wrapper class, it is important to create the native methods in java and wrap them through a public method inside the class.

The convention followed by JNI is crucial here. The convention followed for declaring function names which are JNI compatible is: `Java_{TLD}_{Org_Name}_{Class_name}_{function_Name}` where:

- ***TLD*** = Top Level Domain
- ***Org_Name*** = Organization name
- ***Class_Name*** = Name of the class, whose method is private static and native.
- ***function_Name*** = Name of the function

Example:
In Java, for the function `getNetworksJni`, the following code is needed

```Java
// filename: CryptpaySdk.java
package com.etogruppe; // com is the TLD and etogruppe is the org_name

public class CryptpaySdk { // Class_name
    static {
        System.loadLibrary("walletsdk");
    }
    private static native String getNetworksJni(); // function_Name
    // other native functions here

    public String getNetworks() throws Exception {
        return getNetworksJni();
    }
}
```

This would translate to a rust function as per the rule, with `no_mangle`, to tell the rust compiler to not mangle the function name in the resulting binary.

One point to note is not to use `snake_case` names of functions, because the JNI would deduce it as class path. `lowerCamelCase` or `CamelCase` is allowed.

```Rust
// lib.rs
#[no_mangle]
pub extern "system" fn Java_com_etogruppe_CryptpaySdk_getNetworksJni(mut env: JNIEnv<'local>,
    _class: JClass<'local>) {
      // implementation
    }
```

The two variables are passed by the JNI call to rust, namely the JNIEnv and the java class. The JClass is an empty place holders to satisfy the Java interface. The JNI environment is a constant pointer the JNI environment and can be used to call methods for checking and raising exceptions as shown below.

```Rust
// lib.rs
fn on_error_null(error: SdkError, env: &mut JNIEnv) {
    // fearlessly doing unwraps here, because we are already in exception
    if env.exception_check().unwrap() {
        // exception was raised,  clear it and raise again
        // to return back to JVM
        env.exception_clear().unwrap();
    }
    env.throw(format!("{error:#?}")).unwrap();
}
```

It can also be used to get strings and other objects from the java native to rust safely and pass back rust strings as java string objects back. These methods are needed almost on every function call and return, and hence have been written as macros to reduce code repetitions.

```Rust
// lib.rs
macro_rules! get_jni_string {
    ($env:ident, $on_error:ident, $input_string:expr) => {{
        let jni_string = match $env.get_string(&$input_string) {
            Ok(jni_string) => String::from(jni_string),
            Err(err) => {
                let error = SdkError::UnhandledError(format!("Invalid string input: {err:#}"));
                $on_error(error, &mut $env);
                String::from("")
            }
        };
        jni_string
    }};
}

macro_rules! set_jni_string {
    ($env:expr, $string:expr) => {
        $env.new_string($string).unwrap()
    };
}
```

## Futures across the ABI boundary

As noticed in the previous example of the rust function, the prefix `extern "system"` is used  to tell compiler some information on how the function is going to be called. The `extern` helps the compiler to know that the function will be called from outside of rust. The `system` describes the application binary interface (ABI) to use to define the function in binary code. With `system` the systems ABI is to be used. This ABI is defined by each system according to the target-triplet configured during the cargo build command.

As we will see later, we could also use `C` ABI to expose our functions using the `C` calling conventions in the binary interface. Based on the ABI selected, different applications running on the same host machine and using the same operating system can interact with each other through the binary layer for exchanging information or calling functions.

Unfortunately, on the binary layer there is no easy concept of futures, since there is no scheduler, which can periodically check if a result of a certain function call is available. Calling an external application function from another application is generally synchronous and the result is returned immediately. This means that for asynchronous function calls like HTTP requests, a blocking [sync] call has to be used by the external application and the calling application has to treat the function call as synchronous.

For the Cawaena SDK, there is a runtime thread started and used for calling the SDK functions, in this case, are all asynchronous. Using `OnceCell` this runtime thread is created on the first call to any SDK function and reused by all others.

```Rust
// lib.rs
fn runtime() -> &'static Arc<Runtime> {
    static INSTANCE: OnceCell<Arc<Runtime>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .thread_name("com.standalone.sdk.thread")
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
        )
    })
}
```

This results in a function definition without need of async await in the exposed interface, since currently there are no known ABI's that support this. The future is actively blocked on, thus making it synchronous.

```Rust
// lib.rs
// async/await is not allowed in the function definition here. Is there any ABI that could do it?
#[no_mangle]
pub extern "system" fn Java_com_etogruppe_CryptpaySdk_setPathPrefix<'local>( 
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    path_prefix: JString<'local>,
) {
    let path_prefix = get_jni_string!(env, on_error_null, path_prefix);

    runtime().block_on(async move {
      // async/await can be done here as it is inside a thread which is "blocked on" i.e. synchronous
        let mut sdk = get_or_init_sdk().write().await;
        sdk.set_path_prefix(&path_prefix)
    });
}
```

A nice way to solve this on the other side would be to wrap the native ABI calls in an interface which uses the futures framework of the used programming language (if available). So, the java class would now look something like this:

```Java
// filename: CryptpaySdk.java
package com.etogruppe; 
import java.util.concurrent.Future; // get the future support from the language

public class CryptpaySdk { 
    static {
        System.loadLibrary("walletsdk");
    }

    private static native String getNetworksJni();

    public String getNetworks() throws Exception {
        return getNetworksJni();
    }
}
```

Wrapping the exposed interface with a future, tells the application which builds on top of this external library function, that it is indeed asynchronous and can implement mechanisms to handle it correspondingly.

## Running Unit tests

These android/java JNI bindings have unit tests in the `../android/tests` folder
To execute them on you local machine use:

```bash
cd sdk/bindings/android/tests
gradle test
```

This will compile the bindings for the native platform of your computer (i.e. *not* for android)
and execute the tests.

## Running Examples

These android/java JNI bindings have examples in the `../android/tests/src/main/java/com/etogruppe/examples` folder.
To execute them on you local machine use:

1. Navigate to the android tests directory: `cd sdk/bindings/android/tests`.
2. `gradle runAllExamples` - will run all of them.
3. `gradle {example task name}` - will run only one. Example: `gradle runCreateNewUser01`.

This will compile the bindings for the native platform of your computer (i.e. *not* for android)
and run the examples.

**Note**: These examples need environmental variables in order to run successfully. Make sure to add a `.env` in the root directory: `sdk/bindings/android/tests` with the corresponding values.

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
