# Ambassadors DAO

This repository contains the smart contracts governing Crypto Capables's Ambassadors DAO (Decentralized Autonomous Organisation).

This repository is based on Suptnik DAO v2's repository and follows a similar structure.

## Overview

### Setup

Pre-requisits to run the smart contracts on your system

1. [NEAR Account](https://wallet.testnet.near.org)
2. [NEAR-CLI](https://docs.near.org/docs/tools/near-cli#setup)
3. [Rust](https://www.rust-lang.org)

Once your account is setup and all the components are installed on your system, follow the next steps to create a DAO factory and a DAO.

<details>
<summary>1. Login with your account.</summary>
<p>

Using [`near-cli`](https://docs.near.org/docs/tools/near-cli#near-login), login to your account which will save your credentials locally:

```bash
near login
```

</p>
</details>

<details>
<summary>2. Clone repository.</summary>
<p>

```bash
git clone https://github.com/Crypto-Capable/ambassadors-dao-contracts
```

</p>
</details>

<details>
<summary>3. Build contract.</summary>
<p>

```bash
# go the the ambassadors-dao directory
cd ambassadors-dao
# build the contract, this will produce a WASM binary
sh build.sh
```

</p>
</details>

<details>
<summary>4. Define the parameters of the new DAO, its council, and create it.</summary>
<p>

- Define the council of your DAO:

```bash
export CONTRACT_ID=DAO_ACCOUNT.testnet
```

```bash
export COUNCIL='["siddyboi.testnet","padiyar.testnet"]'
```

- Configure the name, purpose, and initial council members of the DAO and convert the arguments in base64:

```bash
export ARGS='{"name": "ca-dao", "purpose": "Crypto Capabale Campus Ambassadors DAO", "council": '$COUNCIL'}'
```

- Create the new DAO!:

```bash
near deploy $CONTRACT_ID \
  --wasmFile res/ambassadors_dao.wasm \
  --initFunction "new" \
  --initArgs "$ARGS" \
  --accountId $CONTRACTID
```

Example response -

```bash
Starting deployment. Account id: v1.daos-hub.testnet, node: https://rpc.testnet.near.org, helper: https://helper.testnet.near.org, file: res/ambassadors_dao.wasm
Transaction Id 6CFo3KaPQ6NGFaakr3GYM6v9Pqkic3mqsEUZgoJaHzDC
To see the transaction in the transaction explorer, please open this url in your browser
https://explorer.testnet.near.org/transactions/6CFo3KaPQ6NGFaakr3GYM6v9Pqkic3mqsEUZgoJaHzDC
Done deploying and initializing v1.daos-hub.testnet
```

**Note:** If you see `false` at the bottom (after the transaction link) something went wrong. Check your arguments passed and target contracts and re-deploy.

If you are prototyping you should create sub-accounts and deploy your smart contracts there. Once you don't need a particular version you can just delete that particular sub-account.

</p>
</details>

<details>
<summary>5. Verify successful deployment and policy configuration.</summary>
<p>

The DAO deployment will create a new [sub-account](https://docs.near.org/docs/concepts/account#subaccounts) ( `genesis.YOUR_ACCOUNT.testnet` ) and deploy a Sputnik v2 DAO contract to it.

- Setup another env variable for your DAO contract:

```bash
export SPUTNIK_ID=genesis.$CONTRACT_ID
```

- Now call `get_policy` on this contract using [`near view`](https://docs.near.org/docs/tools/near-cli#near-view)

```bash
near view $SPUTNIK_ID get_policy
```

</p>
</details>

## Details

### Registration

Use nearamp sdk for signing up with a funded wallet with a balance of 0.1 Near. For succesfull registration, the user will be asked for a referral ID that belongs to either a registered ambassador or a council member.

Whenever the DAO is created, each of the council members will be assigned a referral ID which can be used for registrations of initial ambassadors. Upon the use of a referral ID, the related account holder will receive a transfer of Near tokens worth USD 5.

### Roles and Permissions

There are two roles in this DAO, the council and the ambassadors. There are some actions that can only be done by the council such as Voting on different Payouts. For implementation of these permissions, we have a method on the `Policy.council` field called `is_council_member` that says if an AccountId belongs to the council.

The tokens belonging to a contract can be accessed through the `env::account_balance()` module and every method requiring a token transfer from the user is done by using the `#[payable]` macro and the attached tokens can be found using `env::attached_deposit()`. For some method calls, token transfer will be done from the contract to the user, this can be done using `Promise::new(account_id).transfer(amount)`.
