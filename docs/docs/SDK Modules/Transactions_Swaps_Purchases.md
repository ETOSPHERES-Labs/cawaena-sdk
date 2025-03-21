# Transactions, Swaps and Purchases

The SDK is primarily used to perform transactions. The type of transactions that the SDK currently facilitates are

1. Wallet transactions
2. Swap transactions
3. Purchase transactions

## Wallet transactions flow

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

## Swap transactions flow

A swap is simply an exchange of value from one currency to another. In the current scenario, the swap is always between SMR <--> EURO currencies. This is executed at viviswap exchange.

For payments in EURO, only the SEPA transfer method is currently supported. See the german explanation [here](https://www.bundesbank.de/de/aufgaben/unbarer-zahlungsverkehr/serviceangebot/sepa/sepa-einfach-erklaert--603346) and the english explanation [here](https://en.wikipedia.org/wiki/Single_Euro_Payments_Area)

The EURO payment needs the user to setup and add their IBAN (International Bank Account Number) to the viviswap exchange. Through this, the viviswap uses SEPA transfers to this IBAN, whenever a swap is triggered from SMR to EURO. The other way around, currently, since direct debit is not setup from the bank of viviswap, the user has to transfer manually from exactly this IBAN (viviswap verifies it in every transfer) to the IBAN owned by viviswap with the amount and a reference number provided by viviswap.

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

## Purchase transactions flow

The purchase transaction is a process different than a swap or a wallet transaction. The purchase is a process of exchanging funds for an underlying artefact. An artefact can be something promised between two parties like a photo, video, or a compliment on a photo, sensor data, services, licenses, etc... The SDK is only interested in creation, querying and confirmation of these purchase requests. The rest of the business logic flow is handled by the corresponding service in ETOPay infrastructure. The transfer of artefact can happen only after a successful execution of the purchase request. This information can be verified at all times by querying the status of the purchase request and the details of the purchase request.

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
