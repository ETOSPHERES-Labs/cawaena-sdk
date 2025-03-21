// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "examples",
    platforms: [
        .iOS(.v13),
        .macOS(.v14),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        .package(path: "../ETOPaySdk")  // our Swift package
        // To use the deployed git version use this package declaration and change the product
        // in `targets` to point to `etopay-swift`.
        //.package(url: "https://github.com/ETOSPHERES-Labs/etopay-sdk-swift.git", from: "0.0.1")
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "utils",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk")
            ],
            path: "Sources/utils"
        ),

        .executableTarget(
            name: "01_create_new_user",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/01_create_new_user"
        ),

        .executableTarget(
            name: "02_onboard_user_postident",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/02_onboard_user_postident"
        ),

        .executableTarget(
            name: "03_create_new_wallet",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/03_create_new_wallet"
        ),

        .executableTarget(
            name: "04_migrate_wallet_from_mnemonic",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/04_migrate_wallet_from_mnemonic"
        ),

        .executableTarget(
            name: "05_migrate_wallet_from_backup",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/05_migrate_wallet_from_backup"
        ),

        .executableTarget(
            name: "06_generate_new_address",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/06_generate_new_address"
        ),

        .executableTarget(
            name: "07_get_balance",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/07_get_balance"
        ),

        .executableTarget(
            name: "08_create_purchase_request",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/08_create_purchase_request"
        ),

        .executableTarget(
            name: "09_onboard_user_viviswap",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/09_onboard_user_viviswap"
        ),

        .executableTarget(
            name: "10_verify_pin",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/10_verify_pin"
        ),

        .executableTarget(
            name: "11_reset_pin",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/11_reset_pin"
        ),

        .executableTarget(
            name: "12_change_password",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/12_change_password"
        ),

        .executableTarget(
            name: "13_send_amount",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/13_send_amount"
        ),

        .executableTarget(
            name: "14_get_exchange_rate",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/14_get_exchange_rate"
        ),

        .executableTarget(
            name: "16_get_tx_list",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/16_get_tx_list"
        ),

        .executableTarget(
            name: "18_delete_user",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/18_delete_user"
        ),

        .executableTarget(
            name: "19_get_wallet_tx_list",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/19_get_wallet_tx_list"
        ),

        .executableTarget(
            name: "20_send_compliment",
            dependencies: [
                .product(name: "ETOPaySdk", package: "ETOPaySdk"),
                "utils",
            ],
            path: "Sources/20_send_compliment"
        ),
    ]
)
