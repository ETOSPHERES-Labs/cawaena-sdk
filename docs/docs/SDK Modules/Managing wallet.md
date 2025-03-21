# Wallet management

The SDK provides users with the opportunity to host their own wallets on their personal end-devices in a safe and easy manner. Before discussing wallet management, some information on wallets and what they are is needed to understand how to manage non-custodial hot wallets.

## The IOTA wallet

The wallet used within the SDK is the official wallet developed by the IOTA Foundation and maintained in its own SDK found [here](https://github.com/iotaledger/iota-sdk). The wallet internally uses the stronghold secret management engine also developed by the IOTA Foundation found [here](https://github.com/iotaledger/stronghold.rs). The secret management engine not only stores sensitive data in files but also uses obfuscation and mechanisms against memory dumps to protect the secrets while they are being operated upon in the memory. Stronghold also provides functions for BIP-0032 derivation using the BIP-0044 derivation path mechanism described [here](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki). The word list used by the wallet is the word list described in BIP-0039 [here](https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039/english.txt).

The various coin types supported by BIP-0044 can be found in the list [here](https://github.com/satoshilabs/slips/blob/master/slip-0044.md). Both `IOTA` and `SMR` are supported and have the coin types `4218` and `4219` respectively.

Currently, in its base implementation the IOTA SDK also needs an in-memory key-value store to manage some metadata related to the stronghold engine and other wallet settings. The IOTA SDK uses a rocksdb implementation in rust for this purpose. There are a few noteworthy problems with rocksdb:

- rocksdb is not light-weight for mobile end devices and the resulting binaries of the sdk take long to build and are bigger in storage requirements.
- rocksdb does not support all mobile platforms
- rocksdb is not maintained on the latest sdks of the android and iOS mobile platforms

After investigation, it was found that the in-memory key-value store was used only for storing some metadata keys and not necessarily need high-performance query execution. Luckily, the IOTA SDK implemented the rocksdb connection as a `Storage` trait. Since, the SDK already used jammdb for its internal key-value store, a fork was created and the trait was implemented using `jammdb`. A pull request was created to the upstream, but the dev team at IOTA Foundation recommended to maintain the fork for now, as there would be some new breaking changes coming and the pull request can be created at a later point. The fork is updated regularly and maintained [here](https://github.com/mighty840/iota-sdk).

## Hot Wallets: The Swift Side of Crypto

Picture a hot wallet as the bustling city centre of your digital finances. Hot wallets are online, connected to the internet, and readily available for transactions. They provide users with quick access to their cryptocurrencies, making them ideal for active trading and daily transactions. Think of them as your go-to pocket wallet for everyday spending in the digital realm.

However, convenience comes at a cost. The very connectivity that makes hot wallets user-friendly also renders them more vulnerable to cyber threats. Hacking attempts and online attacks pose a constant risk, making it crucial for users to exercise caution and implement additional security measures when relying on hot wallets.

### Pin and password in the SDK

Generally, the password requirements for any application need to meet today's standards. This might become difficult for the user to remember their wallet stronghold password and also an irritating experience to enter it every time even for the smallest of transactions. On the other side, for a secure wallet application, the SDK should not rely on the interfacing application to do password management for a secret manager used internally. This has a lot of side effects, such as, the application might bypass the SDK logic for protecting access to the secret by simply using the password against the file, with no knowledge of the SDK. This is a security risk and cannot be accepted.

The end devices today support pin entry mostly protected by biometric authentication for ease but secure user experience, when it comes to accessing a restricted OS functionality. Taking all this in account, the SDK was designed to provide the end users possibilities to set up their wallet using a `password` and a `pin`.

- The password stays with the SDK in an encrypted form and only the pin can be used to decrypt it. Thus, for every operation with the secret manager, where a password is needed, the user must only enter the pin. This solves the problem of user experience.

- The issue of password management is also solved, since now the SDK internally manages the password, while still relying completely on the user to unblock it using the pin. The SDK cannot act in its own interest even if there was a malicious code trying to unblock the wallet! The probability distribution of the pin, being relatively weak, (4 to 6 digit), is improved through the addition of a pseudo random salt, which in combination with a hash function results in an encryption password of significant strength and quasi-random probability distribution. This is used then to encrypt the password for the secret manager.

Thus an attacker would need information on the salt, the encrypted password, pin and the stronghold file to be able to gain access to the wallet functions. This is tough and would need somehow physical access to the end device, and to the end user. Security of end-user and their devices is out of the scope for ETOPay ecosystem.

## Creating the wallet

The stronghold secret manager requires a file path for the wallet and a password to unlock this file. This password disables other applications from interpreting the files created by the stronghold engine and needs to come from the user.

The IOTA SDK offers an account manager structure which comprises of various fields to work with the wallet and the internal wallet accounts. The SDK creates a standard account `standalone` for its usage. There might be other accounts that could exist and are not operated upon by the SDK. The following ways can be used to create a wallet in the SDK:

### Create a new wallet

This does not require any user input except `username`,  `password` and `pin`. But, this should be a multi-step process. The created wallet returns a mnemonic. The app should immediately delete the wallet. In the second step the migration of the wallet with the mnemonic should be carried out and the wallet is only loaded with the mnemonic entered by the user. This approach protects the user against creating a wallet without never confirming the mnemonic back to the SDK and also by deleting a wallet, the SDK can ensure that there is actually no wallet created whose mnemonic was never entered from outside the application. This forces applications to have their end-users the mnemonic either memorized or input from a copy.

???+ info

    A fresh wallet can be created by a random seed, using the stronghold secret manager. It needs the password and username. The username is part of the file path and helps distinguish across different user wallets on the same end device. It returns the mnemonic, and this needs to be securely stored by the user, otherwise access to the funds on the wallet addresses would get limited. A node url for the DLT network can also be selected. Currently, the PoW is set to local, however it might change based on the used node url and its support for PoW.

!!! Note

    The code snippets provided are intended as pseudo-code to demonstrate logic and workflows. They are not guaranteed to compile, execute, or function as-is. Users should adapt and validate them according to their specific requirements and development environment.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();

        // Generate access token
        // Create and initialize the user

        let mnemonic = sdk.create_new_wallet("pin", "password").await.unwrap();
        sdk.verify_mnemonic("pin", "password", &mnemonic).await.unwrap();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();
            sdk.setConfig("...");
            
            // Generate access token
            // Create and initialize the user

            try {
                String mnemonic = sdk.createNewWallet("pin", "password");
                sdk.verifyMnemonic("pin", "password", mnemonic); 
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import ETOPaySdk
    import Foundation

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")
    
    // Generate access token
    // Create and initialize the user

    do {
        mnemonic = try await sdk.createNewWallet(pin: "pin", password: "password")
        try await sdk.verifyMnemonic(pin: "pin", password: "password", mnemonic: mnemonic)
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")
    
    // Generate access token
    // Create and initialize the user

    await sdk.setWalletPassword("pin", "password");
    let mnemonic = await sdk.createNewWallet("pin");
    await sdk.verifyMnemonic("pin", mnemonic)
    ```

### Migrate an existing wallet

This just performs the second step of the create fresh wallet process and needs in addition to the `mnemonic` also the `username`, `password` and `pin`.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();
        
        // Generate access token
        // Create and initialize the user

        sdk.create_wallet_from_mnemonic("pin", "password", "mnemonic").await.unwrap();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();
            sdk.setConfig("...");
            
            // Generate access token
            // Create and initialize the user

            try {
                sdk.createWalletFromMnemonic("pin", "password", "mnemonic");
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import ETOPaySdk
    import Foundation

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")
    
    // Generate access token
    // Create and initialize the user
    
    do {
        try await sdk.createWalletFromMnemonic(pin: "pin", password: "password", mnemonic: "mnemonic")
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")
    
    // Generate access token
    // Create and initialize the user

    await sdk.setWalletPassword("pin", "password");
    await sdk.createWalletFromMnemonic("pin", "mnemonic");
    ```

### Create wallet from a backup file

The SDK provides functionality to create a backup file in `kdbx` format as a byte array. Backups can only be created if a wallet exists.

To create the backup, the following are required:

* `pin`: This is the same PIN that was set for the wallet.
* `backup_password`: A new, separate password set specifically for securing the backup file. This is not the same password used for the wallet.

To restore the backup, the following are required:

* The kdbx `backup bytes`.
* A `new pin` used to create the new wallet.
* The `backup_password` used during the backup process.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();
        
        // Generate access token
        // Create and initialize the user
        
        let backup_bytes: Vec<u8> = sdk.create_wallet_backup("pin", "backup_password").await.unwrap();
        sdk.create_wallet_from_backup("new pin", &backup_bytes, "backup_password").await.unwrap();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();
            sdk.setConfig("...");
            
            // Generate access token
            // Create and initialize the user

            try {
                byte[] backup_bytes = sdk.createWalletBackup("pin", "backup_password");
                sdk.createWalletFromBackup("new pin", backup_bytes, "backup_password");
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import ETOPaySdk
    import Foundation

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")
    
    // Generate access token
    // Create and initialize the user
    
    do {
        let backup_bytes = try await sdk.createWalletBackup(pin: "pin", password: "backup_path")
        try await sdk.restoreWalletFromBackup(pin: "new pin", backup: backup_bytes, backup_password: "backup_password")
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")
    
    // Generate access token
    // Create and initialize the user

    let backup_bytes = await sdk.createWalletBackup("pin", "backup_password");
    await sdk.createWalletFromBackup("new pin", backup_bytes, "backup_password");
    ```

## Deleting the wallet

This function just deletes the wallet files and is a one-way function, to be used under extreme caution, as it could result in permanent loss of funds. Note that, similar to any other wallet operation, deleting the wallet is also a wallet operation and requires the wallet to be correctly initialized. Without initialization, the deletion of wallet would fail.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();
        
        // Generate access token
        // Create and initialize new user
        // Create a new wallet
        
        sdk.delete_wallet();
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();
            sdk.setConfig("...");
            
            // Generate access token
            // Create and initialize new user
            // Create a new wallet

            try {
                sdk.deleteWallet();
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import ETOPaySdk
    import Foundation

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")
    
    // Generate access token
    // Create and initialize new user
    // Create a new wallet
    
    do {
        try await sdk.deleteWallet()
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")
    
    // Generate access token
    // Create and initialize the user
    // Create a new wallet

    await sdk.deleteWallet("pin")
    ```

## Password and pin utilities

In addition to creating, migrating, backups and initialization, the wallet module also performs auxiliary operations for pin and password management. It supports function to reset the pin using password, verify the pin, or change the wallet password using the current password and pin. Wallet initialization is again a pre-requisite, since the pin and password operations are related to the wallet and can only be performed once a wallet is initialized successfully.

=== "Rust"

    ```rust linenums="1"
    async fn main() {
        let mut sdk = Sdk::default();
        sdk.set_config(...).unwrap();
        
        // Generate access token
        // Create and initialize new user
        // Create a new wallet

        // Try to verify the pin
        sdk.verify_pin("pin");
        // or reset the pin to a new one
        sdk.reset_pin("password", "new_pin")
        // or change the password
        sdk.change_password("pin", "password", "new_password");
    }
    ```

=== "Java"

    ```java linenums="1"
    package org.example.app;
    import com.etopay.Wallet;

    public class app {
        public static void main(){
            Wallet sdk = new Wallet();
            sdk.setConfig("...");
            
            // Generate access token
            // Create and initialize new user
            // Create a new wallet

            try {
                // Try to verify the pin
                sdk.pinVerify("pin");
                // or reset the pin to a new one
                sdk.pinReset("password", "new_pin");
                // or change the password
                sdk.passwordChange("pin", "password", "new_password");
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }
    ```

=== "Swift"

    ```swift linenums="1"
    import ETOPaySdk
    import Foundation

    let sdk = ETOPaySdk()
    try await sdk.setConfig(config: "...")
    
    // Generate access token
    // Create and initialize new user
    // Create a new wallet

    do {
        // Try to verify the pin
        try await sdk.verifyPin(pin: "pin")
        // or reset the pin to a new one
        try await sdk.resetPin(password: "password", new_pin: "new_pin")
        // or change the password
        try await sdk.changePassword(pin: "pin", current_password: "password", new_password: "new_password")
    } catch {
        print(error.localizedDescription)
    }
    ```

=== "Typescript"

    ```typescript linenums="1"
    import * as wasm from "../pkg/etopay_sdk_wasm";

    const sdk = await new ETOPaySdk();
    await sdk.setConfig("...")
    
    // Generate access token
    // Create and initialize the user
    // Create a new wallet

    // Try to verify the pin
    await sdk.verifyPin("pin");
    // or reset the pin to a new one
    await sdk.resetPin("pin", "new_pin");
    // or change the password
    await sdk.setWalletPassword("pin", "new_password");
    ```

## Wallet flows

```
                Mnemonic  Username  Password  Pin  backup password         Pin                            Pin        
                   | ^         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
                   | |         |      |       |      |                     |                              |          
    Inputs         v |         v      v       v      v                     v                              v          
-------------------------------------------------------------------------------------------------------------------  
                                                                |                           |                        
                                           +-------------+      |                           |                        
                                           |             |      |                           |                        
                                           | Create      |      |      +-------------+      |                        
                                           | New         +---+  |      |             |      |        +--------------+
                                           | Wallet      |   |  |      |  Initialize |      |        |              |
                                           |             |   |  |      |  Wallet     +------+-------->   Delete     |
                                           +-------------+   |  +------>             |      |        |   wallet     |
                                                             |  |      |             |      |        |   (external) |
                                                             |  |      +------+------+      |        |              |
                         +----------+      +-------------+   |  |             |             |        +--------------+
                         |          |      |             |   |  |             |             |                        
                         | Mnemonic |      | Migrate     |   |  |             |             |                        
                 +------->          +------> Existing    |   |  |             |             |                        
                 |       |          |      | Wallet      |   |  |      +------v------+      |                        
                 |       |          |      |             |   |  |      |             |      |                        
                 |       +----------+      +-------------+   |  |      |  User       |      |                        
                 |                                           |  |      |  Wallet     |      |                        
                 |                                           |  |      |  Functions  |      |                        
                 |           +------+      +-------------+   |  |      |             |      |                        
                 |           |      |      |             |   |  |      +-------------+      |                        
                 |           |Backup|      | Create      |   |  |                           |                        
                 |           | File +------> Wallet      |   |  |                           |                        
                 |           |      |      | From        |   |  |                           |                        
                 |           |      |      | Backup      |   |  |                           |                        
                 |           +------+      +-------------+   |  |                           |                        
                 |                                           |  |                           |                        
                 |                                           |  |                           |                        
                 |    +-------------+      +-------------+   |  |                           |                        
                 |    |             |      |             |   |  |                           |                        
                 |    | Verify      |      |  Delete     |   |  |                           |                        
                 +----+ Mnemonic    <------+  wallet     <---+  |                           |                        
                      |             |      | (internal)  |      |                           |                        
                      |             |      |             |      |                           |                        
                      +-------------+      +-------------+      |                           |                        
                                                                |                           |                        
                                                                |                           |                        
                                             Once(setup)        |        Multiple times     |       Once (tear down) 
                                                                |                           |                        
```
