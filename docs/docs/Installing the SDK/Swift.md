# Swift Installation

The swift SDK is now available through a private gitlab repository with binaries hosted on JFrog. You can add the repository as a `dependency` in your `Package.swift`. Before starting, make sure you have access to the Gitlab repository as it will be needed for the instructions below.

We provide binaries for the following platforms:

- `aarch64-apple-ios`: targets 64-bit ARM processors for iOS devices like iPhones and iPads.
- `x86_64-apple-ios`: for 64-bit Intel processors for iOS simulators.
- `aarch64-apple-ios-sim`: for 64-bit ARM processors running iOS simulators, typically used on Apple Silicon Macs.
- `aarch64-apple-darwin`: for 64-bit ARM processors on macOS systems (eg. M1).
- `x86_64-apple-darwin`: for 64-bit Intel processors on macOS systems.

## Using ETOPay with Swift Package Manager

The ETOPay SDK can also be used as a Swift package. Follow these steps to integrate it into your project:

Add the repository as a `dependency` in your `Package.swift` file:

```swift
import PackageDescription

let package = Package(
    name: "program",
    dependencies: [
        .package(url: "https://github.com/ETOSPHERES-Labs/etopay-sdk-swift", from: "0.0.1")
    ],
    targets: [
        .executableTarget(
            name: "main",
            dependencies: [
                .product(name: "ETOPaySdk", package: "etopay-swift")
            ]),
    ]
)
```

The `ETOPaySdk` module will then be available for import in your project.

## Using ETOPay with XCode

In XCode, go to _File -> Add Package Dependencies_. In the top right search box, enter the url of the GitLab repository (`https://github.com/ETOSPHERES-Labs/etopay-sdk-swift`) and select the ETOPay SDK when it appears in the list. Select _Add Package_ and follow the on-screen instructions.

### Access to the binaries on JFrog

To allow the Swift Package Manager and XCode to download the binaries stored in JFrog, you need to set up an access token in your `~/.netrc` file.

1. Visit `JFrog` and log in. In the top right corner, click your name and then "Edit Profile." Under "Identity Tokens," click "Generate an Identity Token" and optionally give the token a name.
2. If not already existing, create the file `.netrc` in your home folder and add the following lines (e.g., using `nano ~/.netrc`)

```zsh
machine repo.farmunited.com
    login <your JFrog username>
    password <your access token>
```

3. You should now be able to access and use the package in your Swift projects!

> Note: you might need to restart XCode for the changes to take effect.

## Future releases of SDK

Future releases of SDK will continue to be published on the private gitlab repository. You will only need to update the dependency version to use the latest release.

## Minimum supported version

- **swift-tools** - `5.8`
- **swiftlang**- `swiftlang-5.8.0.124.2`
- **clang** - `clang-1403.0.22.11.100`
- **iOS** - `13`
