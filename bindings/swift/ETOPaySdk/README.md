# ETOPay Swift Package

This repository contains the Swift package for the ETOPay SDK.

## Usage

Simply add this repository in to the `dependencies` in the `Package.swift` file like so:

```swift
import PackageDescription

let package = Package(
    name: "YourProject",
    dependencies: [
        .package(url: "https://github.com/ETOSPHERES-Labs/etopay-sdk-swift.git", from: "0.0.1")
    ],
    targets: [
        .executableTarget(
            name: "YourTarget",
            dependencies: [
                .product(name: "ETOPaySdk", package: "etopay-sdk-swift")
            ]
        )
    ]
)
```

The `ETOPaySdk` module is then available for import in your project.

## Binary Targets

If you’re integrating our Swift package and it includes pre-built binaries (such as an xcframework), you don’t need to build these binaries from source. Instead, you can reference them directly in your `Package.swift` file using a binary target. This allows the Swift Package Manager to download and incorporate the binary automatically when you add our package as a dependency.

```swift
.binaryTarget(
    name: "ETOPaySdkBin",
    url: "https://github.com/ETOSPHERES-Labs/etopay-sdk-swift/releases/download/0.0.1/ETOPaySdkBin.xcframework.zip",
    checksum: "<computed-checksum>"
)
```

**For now, binary assets are deployed manually during a releases.**

## Access

This repository is public on GitHub. Therefore, the Swift Package Manager can download both the source code and binary assets without requiring any additional authentication or access tokens.