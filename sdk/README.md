# ETOPay SDK

The ETOPay SDK provides the logic for interacting with the ETOPay backend correctly. The SDK was planned to help developers with a quicker, safer and easier on-boarding in the web3 world, while using the ETOPay infrastructure to support various use-cases.

The SDK is divided in various modules which come together to ensure an easy and safe way to incorporate the SDK in the application.

For ease of integration, the SDK also generates bindings in various languages for various platforms, especially for mobile platforms, to provide easy integration with native and hybrid mobile app development frameworks.

## Introduction

The ETOPay SDK is built in `rust`. It is primarily an implementation of the various functionalities for managing users, wallets, on-boarding of users through KYC (Know Your Customer) processes, payment methods and listing usage information.

The SDK was designed to support the `ETOPay` application. It is a social media application, which allows monetization of user-generated content. However, in the same principle, any digital data, given that it is authentic and its origin can be verified, can be monetized using the ETOPay ecosystem, which includes the ETOPay infrastructure and the sdk.

The big picture behind ETOPay is a data marketplace. Data processing, silo management and search engine features have been excluded by design from ETOPay to make it a minimal ecosystem for monetization.

### Overview of the SDK functional components

The figure below shows the functional component diagram of the SDK. The core of the SDK is a web3 hot-wallet. This wallet is used to store assets on the local machine running the application built with the SDK. The supporting components like the backend API, user state management and access control logic work for improving ease of use for the end user as well as ensuring correct process flow and state transitions between the ETOPay infrastructure and application.

The binding layer is just a simple 1-to-1 wrapper around the SDK functionalities. This just exports the existing business logic implemented in the SDK in rust to other programming stacks to avoid re-implementation as well as guarantee memory safety natively in code.

The access control section at the bottom shows the input parameters needed from the user/application to authenticate itself against the SDK. For the one-time on-boarding in addition to the `pin` and `access_token` the `username` and `password` is also needed. For regular usage, the `pin`, whenever required and `access_token` is required to ensure smooth handling of operations, including internal function calls to the ETOPay infrastructure and the wallet.

```
+-------------------------------------------------------------------------------+
|                                                                               |
|   +-----------------------------------------------------------------------+   |
|   |                                                                       |   |
|   |                                                                       |   |
|   |   +---------------------------------------------+    +--------------+ |   |
|   |   |                                             |    |              | |   |
|   |   |   +-------------------------------------+   |    |   Backend    | |   |
|   |   |   |                                     |   |    |   API        | |   |
|   |   |   |     +------------------------+      |   |    |              | |   |
|   |   |   |     |                        |      |   |    +--------------+ |   |
|   |   |   |     |                        |      |   |                     |   |
|   |   |   |     |                        |      |   |                     |   |
|   |   |   |     |       IOTA SDK         |      |   |    +--------------+ |   |
|   |   |   |     |                        |      |   |    |              | |   |
|   |   |   |     |       Stronghold       |      |   |    | User         | |   |
|   |   |   |     |         wallet         |      |   |    | State        | |   |
|   |   |   |     |         manager        |      |   |    | Management   | |   |
|   |   |   |     |                        |      |   |    +--------------+ |   |
|   |   |   |     +------------------------+      |   |                     |   |
|   |   |   |                                     |   |                     |   |
|   |   |   |           Wallet Manager            |   |    +--------------+ |   |
|   |   |   +-------------------------------------+   |    |              | |   |
|   |   |                                             |    |   Access     | |   |
|   |   |                Wallet User                  |    |   Control    | |   |
|   |   |                                             |    |   Logic      | |   |
|   |   +---------------------------------------------+    +--------------+ |   |
|   |                         SDK                                           |   |
|   +-----------------------------------------------------------------------+   |
|                           Bindings                                            |
|                                                                               |
|                                                                               |
+--+--------------------------------------+------+----------------------------+-+
   |      Onboarding authentication       |      |   Usage authentication     |  
   |                                      |      |                            |  
   +--^---------^-------^---------^-------+      +-----^-----------^----------+  
      |         |       |         |                    |           |             
      |         |       |         |                    |           |             
      |         |       |         |                    |           |             
   Username  Password  Pin    Access token            Pin      Access token      
```

## Modules of ETOPay SDK

### User management

The sdk is designed to allow multiple users working with their own wallets on the same end devices sharing the same storage space. This makes it easy for a single person to have multiple alias users for different purposes and use different wallets for each of them to have a clear separation of risks.

The user initialization is done by two main functions of the sdk:

1. `create_new_user`: This creates a new user in the in-memory database. All the properties of the user, like his selected KYC process, his KYC status, his access token for the backend, pin, encrypted password, etc... are set with the default values. A salt is generated for the user, which will be used later for encrypting the password.

2. `init_user`: This function initializes the user for a new session. It also checks that a valid access token has been provided by updating the KYC status of the user from the backend in the SDK internal state.

```
          Username    Refresh access   Username              Pin              
             |            token          |                      |             
             |             |             |                      |             
             |             |             |                      |             
             |             |             |                      |             
        +----v---------+   |     +-------v------+        +------v---------+   
        |              |   |     |              |        |                |   
        |  Create      |   |     | Initialize   |        |     Delete     |   
        |  new         +---v-----> User         +-------->     User       |   
        |  User        |         |              |        |                |   
        +--------------+         +-----+--------+        +----------------+   
   Once                                |                                      
xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   Multiple                            |                                      
   Times                   +-----------v-----------------+                    
                           |  User           Wallet      |                    
                           |  State          Operations  |                    
                           |  Change                     |                    
                           +-----------------------------+                    
```

The user is created and needs to be initialized before any state updates or wallet-related operations can be performed for this user. This allows the SDK to create multiple users and by using the initializing function, only the selected user is activated for the session. Without initializing a user, all operations related to the user would fail or conversely the previously initialized user's session will be used and might corrupt the state! To protect this from happening, before initializing the user, a corresponding access token is required. An invalid access token would result in failure of the initialization.

The access token brings the following safe operations for the SDK:

1. Only the correct user with the username would be initialized. Mismatch would cause an error.
2. The application can only initialize a user, only after the authorization of the actual person, since they would need to share their credentials for creating an access token.
3. Any user whose rights have been revoked, due to reporting, would not be able to use the system as the access token would be invalid and generating a new one would not also work.

The user management is local to the end devices and deleting the application data, cache, temporary data files, etc... would result in a loss of state and that would require the application to re-create and re-initialize user.

Lastly, deleting the user is simply deleting the user entity from the local database, while maintaining entries for other users. The delete user also calls the backend API to trigger an archiving action for the user. Deleting the user also deletes all the local data files for the user, which in this case are files related to the wallet. Since, this is a one-way operation a user is required to enter the pin, that they have set for the wallet. If there is no wallet setup, the pin can be skipped and the user is simply deleted locally and archived in the backend.

### Wallet management

The SDK provides users with the opportunity to host their own wallets on their personal end-devices in a safe and easy manner. Before discussing wallet management, some information on wallets and what they are is needed to understand how to manager non-custodial hot wallets.

#### Hot and Cold wallets

Users today maintain their crypto-currencies in wallets. A wallet is purely software coupled with a secret store, from which the addresses are deterministically derived from a single seed using hierarchical deterministic procedure. For more information, see [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki). The actual tokens/coins of the cryptocurrency reside on these address. Thus, in reality, the wallet is a method of accessing and controlling the crypto-currency, which in turn always lies on the network and never in a hardware or in the typical sense inside a wallet.

Since, the wallet has two components, the software and the storage, this allows to classify the wallet based on how these components are implemented and in which environment, specifically under whose control, these components are running/installed. The wallet matrix is shown as below:

| Environment      | Software       | Storage     |
|------------------|----------------|-------------|
| User/Individual  | Cold/Hot-Wallet| Non-custodial |
| Exchange/Business| Hot-Wallet     | Custodial   |

Users use cold non-custodial wallets to keep access and control over their secret seeds, which effectively give them control over their keys. Some users also use hot custodial wallets to efficiently trade cryptocurrencies at exchanges and participate in various decentralized finance (DeFi) protocols like lending pools, swaps, staking, etc.

A well-informed and researched user will temporarily maintain hot custodial wallets to engage with the chain and market and permanently maintain a major chunk of funds to addresses controlled by the non-custodial cold wallet. The user will then shuffle between the two wallets based on funds risk.

##### Hot Wallets: The Swift Side of Crypto

Picture a hot wallet as the bustling city centre of your digital finances. Hot wallets are online, connected to the internet, and readily available for transactions. They provide users with quick access to their cryptocurrencies, making them ideal for active trading and daily transactions. Think of them as your go-to pocket wallet for everyday spending in the digital realm.

However, convenience comes at a cost. The very connectivity that makes hot wallets user-friendly also renders them more vulnerable to cyber threats. Hacking attempts and online attacks pose a constant risk, making it crucial for users to exercise caution and implement additional security measures when relying on hot wallets.

##### Cold Wallets: Embracing the Fortress of Security

Now, shift your focus to the serene fortress nestled away from the hustle and bustle – the cold wallet. Unlike their hot counterparts, cold wallets remain offline and disconnected from the internet. This deliberate isolation provides a higher level of security, shielding your digital assets from the prying eyes of online threats.

Cold wallets are the guardians of large sums of cryptocurrency, often employed for long-term storage. While they may not offer the immediacy of hot wallets, their offline nature makes them an attractive option for those prioritizing the safety and longevity of their crypto investments.

#### The IOTA wallet

The wallet used within the SDK is the official wallet developed by the IOTA Foundation and maintained in its own SDK found [here](https://github.com/iotaledger/iota-sdk). The wallet internally uses the stronghold secret management engine also developed by the IOTA Foundation found [here](https://github.com/iotaledger/stronghold.rs). The secret management engine not only stores sensitive data in files but also uses obfuscation and mechanisms against memory dumps to protect the secrets while they are being operated upon in the memory. Stronghold also provides functions for BIP-0032 derivation using the BIP-0044 derivation path mechanism described [here](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki). The word list used by the wallet is the word list described in BIP-0039 [here](https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039/english.txt).

The various coin types supported by BIP-0044 can be found in the list [here](https://github.com/satoshilabs/slips/blob/master/slip-0044.md). Both `IOTA` and `SMR` are supported and have the coin types `4218` and `4219` respectively.

Currently, in its base implementation the IOTA SDK also needs an in-memory key-value store to manage some metadata related to the stronghold engine and other wallet settings. The IOTA SDK uses a rocksdb implementation in rust for this purpose. There are a few noteworthy problems with rocksdb:

- rocksdb is not light-weight for mobile end devices and the resulting binaries of the sdk take long to build and are bigger in storage requirements.
- rocksdb does not support all mobile platforms
- rocksdb is not maintained on the latest sdks of the android and iOS mobile platforms

After investigation, it was found that the in-memory key-value store was used only for storing some metadata keys and not necessarily need high-performance query execution. Luckily, the IOTA SDK implemented the rocksdb connection as a `Storage` trait. Since, the SDK already used jammdb for its internal key-value store, a fork was created and the trait was implemented using `jammdb`. A pull request was created to the upstream, but the dev team at IOTA Foundation recommended to maintain the fork for now, as there would be some new breaking changes coming and the pull request can be created at a later point. The fork is updated regularly and maintained by Sharang Parnerkar.

#### Creating, managing and using the wallet

The stronghold secret manager requires a file path for the wallet and a password to unlock this file. This password disables other applications from interpreting the files created by the stronghold engine and needs to come from the user.

The IOTA SDK offers an account manager structure which comprises of various fields to work with the wallet and the internal wallet accounts. The SDK creates a standard account `standalone` for its usage. There might be other accounts that could exist and are not operated upon by the SDK. The following functions are performed by the wallet manager:

1. Create a new wallet: A fresh wallet created by a random seed, using the stronghold secret manager. It needs the password and username. The username is part of the file path and helps distinguish across different user wallets on the same end device. It returns the mnemonic, and this needs to be securely stored by the user, otherwise access to the funds on the wallet addresses would get limited. A node url for the DLT network can also be selected. Currently, the PoW is set to local, however it might change based on the used node url and its support for PoW.

2. Migrate an existing wallet: It creates a new stronghold secret file from an already existing mnemonic and encrypts it with the passed password.

3. Wallet backup and restore: These functions allow to create password encrypted backups and restore these password encrypted backups back to a stronghold file. These backups are compatible across various wallet application in the IOTA ecosystem and can be exported to these applications and the wallet can be restored, in case the mnemonics are forgotten.

4. Delete wallet: This function just deletes the wallet files and is a one-way function, to be used under extreme caution, as it could result in loss of funds.

5. Changing the password: This changes the password for the stronghold secret manager and requires the current password.

The wallet user wraps around the wallet manager and additionally offers functions around usage of the wallet like generating new addresses, fetching balance, syncing internal state with the network, transfer of funds, etc...

#### Wallet flows

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

Creating a wallet by a user can be done in practically three ways using the SDK:

1. Creating a fresh wallet from randomness: This does not require any user input except `username`,  `password` and `pin`. But, this is a multi-step process. The created wallet returns a mnemonic and immediately deletes itself. In the second step the migration of the wallet with the mnemonic is carried out and the wallet is thus loaded. This approach protects the user against creating a wallet without never confirming the mnemonic back to the SDK and also by deleting a wallet, the SDK ensures that there is actually no wallet created whose mnemonic was never entered from outside the application. This forces applications to have either cache of mnemonic for reuse (dangerous but out of security scope of SDK) or the user themselves to have the mnemonic either memorized or have a copy of it.

2. Migrate existing wallet from mnemonic: This just performs the second step of the create fresh wallet process and needs in addition to the `mnemonic` also the `username`, `password` and `pin`.

3. Migrate existing wallet from backup file: This restores an existing wallet from a backup file. It requires the `backup path` of the file as well the `backup password` in addition to the new `username`, `password` and `pin` to be used.

#### Pin and password in the SDK

Generally, the password requirements for any application need to meet today's standards. This might become difficult for the user to remember their wallet stronghold password and also an irritating experience to enter it every time even for the smallest of transactions. On the other side, for a secure wallet application, the SDK should not rely on the interfacing application to do password management for a secret manager used internally. This has a lot of side effects, such as, the application might bypass the SDK logic for protecting access to the secret by simply using the password against the file, with no knowledge of the SDK. This is a security risk and cannot be accepted.

The end devices today support pin entry mostly protected by biometric authentication for ease but secure user experience, when it comes to accessing a restricted OS functionality. Taking all this in account, the SDK was designed to provide the end users possibilities to set up their wallet using a password and a pin.

The password stays with the SDK in an encrypted form and only the pin can be used to decrypt it. Thus, for every operation with the secret manager, where a password is needed, the user must only enter the pin, or allow the application to fetch it through bio-metrically protected secure storages on end devices. This solves the problem of user experience.

The issue of password management is also solved, since now the SDK internally manages the password, while still relying completely on the user to unblock it using the pin. The SDK cannot act in its own interest even if there was a malicious code trying to unblock the wallet! The probability distribution of the pin, being relatively weak, (4 to 6 digit), is improved through the addition of a pseudo random salt, which in combination with a hash function results in an encryption password of significant strength and quasi-random probability distribution. This is used then to encrypt the password for the secret manager.

Thus an attacker would need information on the salt, the encrypted password, pin and the stronghold file to be able to gain access to the wallet functions. This is tough and would need somehow physical access to the end device, and to the end user. Security of end-user and their devices is out of the scope for ETOPay ecosystem.

### Onboarding for various KYC processes

The SDK supports KYC onboarding for users with  `Postident` and  `Viviswap`. The individual processes behind the KYC on-boarding are described in the ETOPay infrastructure in their individual service descriptions.

The SDK only maintains the state for the user on the local device. Once, the user wishes to submit the information after modifying and verifying it, the state gets transferred to the infrastructure, where further processing at the corresponding KYC provider can take place. Also information from the infrastructure is relayed back to the SDK based on internal and external state changes, however, only on request (polling-based).

### Transactions

The SDK is primarily used to perform transactions. The type of transactions that the SDK currently facilitates are

1. Wallet transactions
2. Swap transactions
3. Purchase transactions

#### Wallet transactions flow

```
+---------------+                      +---------------+
|               |                      |               |
|               |                      | Wallet        |
| Wallet        +----------------------> Address       |
| Address       |                      | Receiver      |
| Sender        |                      |               |
+---------------+                      +---------------+
```

The wallet transaction is a simple transfer of funds from one address to another facilitated by the DLT network node and the wallet software running within the SDK.

#### Swap transactions flow

A swap is simply an exchange of value from one currency to another. In the current scenario, the swap is always between SMR <--> EURO currencies. This is executed at viviswap exchange.

For payments in EURO, only the SEPA transfer method is currently supported. See the german explanation [here](https://www.bundesbank.de/de/aufgaben/unbarer-zahlungsverkehr/serviceangebot/sepa/sepa-einfach-erklaert--603346) and the english explanation [here](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area)

The EURO payments needs the user to setup and add their IBAN (International Bank Account Number) to the viviswap exchange. Through this, the viviswap uses SEPA transfers to this IBAN, whenever a swap is triggered from SMR to EURO. The other way around, currently, since direct debit is not setup from the bank of viviswap, the user has to transfer manually from exactly this IBAN (viviswap verifies it in every transfer) to the IBAN owned by viviswap with the amount and a reference number provided by viviswap.

The methods ([See here](./src/core/viviswap.rs)) used in relation of this swap are:

1. `get_iban_for_viviswap`: This function allows the user query their own IBAN saved at viviswap.
2. `ensure_detail`: This function verifies if a detail created at viviswap is legitimate, syntactically and semantically. A detail is basically an address for a particular payment method. The various payment methods used by viviswap are SMR, IOTA, BTC, ETH, etc... for crypto-currencies and PAYPAL, SEPA, etc... for EURO payments. For example, the address for the payment method SMR would be shimmer wallet address and the address for the payment method SEPA would be the IBAN.
3. `update_iban_for_viviswap`: This function updates the IBAN of the user in the viviswap exchange. The update is actually an advanced `upsert` action. The update would insert the IBAN if none exists and also replace the existing IBAN with the new one.
4. `create_deposit_with_viviswap`: This function creates details of a fiat to crypto swap. Deposit is to be understood as deposit of funds to a crypto currency address. Currently, the swap is between EURO to SMR. Since, there is no direct debit authorization available, creating the deposit generally means getting information about the bank details of viviswap and the reference number, and advising the user to make a SEPA transfer in the required amount.
5. `create_detail_for_viviswap`: This function creates a user detail for a payment method. This could be adding the crypto address for a certain payment method to the viviswap exchange. This detail with its id can then be directly used for the swaps.
6. `get_payment_method_id_viviswap`: This is a generic function and has to be called once to cache the UUIDs of all the payment methods supported by viviswap.
7. `create_withdrawal_with_viviswap`: This function is the opposite of deposit. Withdrawal is to be understood as withdrawal of funds from a crypto currency address. If a pin is provided, the function automatically immediately transfers money from the crypto address of the user to that of viviswap and ideally viviswap would automatically transfer the funds to the IBAN created in their system. If no pin is provided, the user is shown the crypto address of the chosen payment method and the user can decide to transfer the funds to this address at any point.
8. `get_swap_list`: This function gives the list of swaps performed at viviswap.
9. `get_swap_details`: This function gives details about a swap, like information on fees, exchange rate, the swap status, etc...
10. `get_exchange_rate`: This function provides the exchange rate for the involved currencies in the swap. Currently, the exchange rate is always provided with EURO as base currency, i.e. it is either SMR/EURO or IOTA/EUR or BTC/EURO and so on... An inversion of the exchange rate gives the reverse rate and should be calculated by simply inverting the value. As confirmed by viviswap, there are no vertical spreads to be considered here!

#### Vertical spreads

A vertical spread in the context of exchange rates between two currencies for a crypto exchange refers to the price difference (spread) between the bid and ask prices of a particular cryptocurrency pair.

In a vertical spread, the bid price represents the highest price that a buyer is willing to pay for a specific cryptocurrency, while the ask price represents the lowest price at which a seller is willing to sell the same cryptocurrency. The vertical spread is the numerical difference between these two prices.

For example, let's say the bid price for Bitcoin (BTC) against US Dollar (USD) is $50,000 and the ask price is $50,100. The vertical spread in this case would be $100 ($50,100 - $50,000).

Vertical spreads are significant for traders and investors because they indicate liquidity and market depth. A narrow spread suggests a highly liquid market with many buyers and sellers, whereas a wide spread may indicate lower liquidity and potentially higher transaction costs. Traders often look for tight vertical spreads when executing trades to minimize costs and ensure efficient transactions.

If an exchange offers no vertical spread, it means that the bid and ask prices for a particular cryptocurrency pair are identical or extremely close to each other. Essentially, there is no difference between the highest price a buyer is willing to pay (bid price) and the lowest price a seller is willing to accept (ask price).

Having no vertical spread implies high liquidity and efficiency in the market. It indicates that there are many buyers and sellers actively trading the cryptocurrency pair, resulting in competitive pricing and minimal transaction costs for traders.

Exchanges that offer no vertical spread are highly desirable for traders because they allow for instant execution of trades at fair market prices without incurring significant costs associated with spreads. This can contribute to a smoother trading experience and better opportunities for traders to enter and exit positions efficiently.

Viviswap takes the burden of the vertical spread on itself by ensuring highest liquidity always! (Tough to achieve)

```
                              Deposit Flow                        
                                                                  
                                                                  
            +------------+            |          +------------+   
            |            |            |          |            |   
            | User       |            |          | Viviswap   |   
            | Wallet     <------------+----------+ Wallet     |   
            | Address    |            |          | Address    |   
            |            |            |          |            |   
            +------------+            |          +------^-----+   
                                      |                 |         
                                      |                 |         
                                      |                 |         
                                      |                 |Trigger  
                                      |                 |         
                                      |                 |         
                                      |                 |         
            +------------+            |            +----+-------+ 
            |            |            |            |            | 
User        | User       |     Bank   |            |  Viviswap  | 
------------> IBAN       +------------+------------>  IBAN      | 
Action      |            |     Ref.   |            |            | 
            |            |     Nr.    |            |            | 
            +------^-----+            |            +------------+ 
                   |                  |                           
                   |                  |                           
                   |                  |                           
                   |                  |                           
                   |                  |                           
                   |                  |             +------------+
                   +------------------+-------------+            |
                                      |             |  Create    |
           Update  -------------------+------------->  Deposit   |
           User           Payment     |             |  Detail    |
           IBAN           Detail      |             |            |
                                      |             +------------+
                                      |                           
                                      |                           
                              User    |    Viviswap               
```

```
                 Withdraw Flow                       
                                                     
                                   +------------+    
                                   |            |    
  User                    |        |  Create    |    
  ------------------------+-------->  Withdraw  |    
  Action                  |        |  Detail    |    
              Viviswap    |        |            |    
      +-------------------+--------+------------+    
      |       Address     |                          
      |                   |                          
+-----v------+            |          +------------+  
|            |            |          |            |  
| User       |            |          | Viviswap   |  
| Wallet     +------------+----------> Wallet     |  
| Address    |            |          | Address    |  
|            |            |          |            |  
+------------+            |          +------+-----+  
                          |                 |        
                          |                 |        
                          |                 |        
                          |                 |Trigger 
                          |                 |        
                          |                 |        
                          |                 |        
+------------+            |            +----v-------+
|            |            |            |            |
| User       |            |            |  Viviswap  |
| IBAN       <------------+------------+  IBAN      |
|            |            |            |            |
|            |            |            |            |
+------------+            |            +------------+
                          |                          
                          |                          
                          |                          
                  User    |    Viviswap              
```

#### Purchase transactions flow

The purchase transaction is a process different than a swap or a wallet transaction. The purchase is a process of exchanging funds for underlying artefact. An artefact can be something promised between two parties like a photo, video, or a compliment on a photo, sensor data, services, licenses, etc... The SDK is only interested in creation, querying and confirmation of these purchase requests. The rest of the business logic flow is handled by the corresponding service in ETOPay infrastructure. The transfer of artefact can happen only after a successful execution of the purchase request. This information can be verified at all times by querying the status of the purchase request and the details of the purchase request.

A purchase request can be created at any time and is unique per purchase. A purchase id is returned by the infrastructure to track this particular request.
Currently, polling is used to wait for the purchase request to be valid. It can be invalid for multiple reasons, as defined in the infrastructure. In case the request turns out to be valid, then the details (supplemented by the infrastructure) are fetched and a confirmation is done through the sdk.

The confirmation of a purchase request means that funds are required to be released from the wallet and this operation needs the pin from the user, to avoid creation and confirmation of purchase request which might not have been authorized by the user. The confirmation triggers a wallet transaction to the recipient's wallet address as well as to the system's wallet address. The purchase details deliver this information for each individual purchase request. The result of the wallet transaction is then added to the purchase request and sent to the infrastructure as part of the confirmation body, so that the infrastructure can search for the transaction on the DLT network.

```
                                        |                      
                                        |                      
                                        |                      
                 +-------------+        |                      
                 |             |        |                      
       User      | Create      |        |                      
     +-----------> Purchase    +--------+-+     +-------------+
     | Request   | Request     |        | |     |             |
     |           |             |        | |     |  (Polling)  |
     |           +-------------+        | +----->  Get        |
     |                                  |       |  Purchase   |
     |                                  |       |  Status     |
     |                                  |       +------+------+
     |      Pin                         |              |       
     +-------------------+              |              |Valid  
                         |              |              |       
                         |              |       +------v------+
                  +------v------+       |       |             |
                  |             |       |       |  Get        |
    Wallet        | Confirm     <-------+-------+  Purchase   |
<-----------------+ Purchase    |       |       |  Details    |
    Transaction   | Request     |       |       |             |
                  |             |       |       +-------------+
                  +-------------+       |                      
                                        |                      
                                        |                      
                                        |                      
                                        |                      
                               Once     |   Multiple times     
                                        |                      
```

## Building the public docs

See [../docs](../docs)
