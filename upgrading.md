# How to upgrade the smart contract?

Two problems -
1. The wasm byte code
2. Reconciliation between the current state and the upgraded state

## Addressing the wasm byte code problem

We need to be able to store the wasm byte code on the smart contract's environment storage. For this we need to store blobs on the chain and keep account of the hash of the blob stored.

To this we have a function called `store_blob`, called as follows

```bash
# store all the byte code into a variable
NEW_VERSION_CODE='cat res/ambassadors_dao.wasm'
# 10 TGas
GAS_100TGas="100000000000000"

# make the function call on the contract, you will have to sign off the call
near call $CONTRACT_NAME store_blob $(eval "$NEW_VERSION_CODE") --accountId $CONTRACT_NAME --gas $GAS_100TGas --amount 10
```

This is going to store the WASM code as a blob and its hash will be store on the contract in the `blobs` property so that we can access it in the future.

## Upgrading to the new version and state reconciliation

Now, that the code has been uploaded, let's focus on how we upgrade to the new version without corrupting the state.

Our smart contract has a function called `migrate`. When you need to upgrade the contract and there needs to be state reconciliation, populate the body of the migrate function accordingly.

For rapid prototyping, you can follow [this](https://www.near-sdk.io/upgrading/prototyping), no migration is needed, and for production grade updates, you can follow [this](https://www.near-sdk.io/upgrading/production-basics).

Now we have two options, either upload a blob into the contract storage and run the `upgrade_self` method on the smart contract or deploy the new contract with a custom init method.

### Option 1

When you need to deploy the upgraded contract and run the migration, use the following -

```bash
# store all the byte code into a variable
NEW_VERSION_CODE='cat res/ambassadors_dao.wasm'
# 10 TGas
GAS_100TGas="100000000000000"

# make the function call on the contract, you will have to sign off the call
near call $CONTRACT_NAME store_blob $(eval "$NEW_VERSION_CODE") --accountId $CONTRACT_NAME --gas $GAS_100TGas --amount 10
# call a method named upgrade_self which will fetch the blob and run a migration
near call $CONTRACT_NAME upgrade_self --accountId $CONTRACT_NAME
```

### Option 2

```bash
near deploy \
  --wasmFile res/ambassadors_dao.wasm \
  --initFunction "migrate" \
  --initArgs "{}" \
  --accountId $CONTRACT_NAME
```
In either case, for a production environment, it is immensly important to run migrations correctly.

This should do the trick 💯