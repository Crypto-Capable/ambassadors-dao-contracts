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
<!-- TODO: edit points 3,4,5,6 and 7 -->
<details>
<summary>3. Build factory contract.</summary>
<p>

```bash
cd ambassadors-dao-contracts/ambassadors-dao-factory && sh ./build.sh
```

</p>
</details>

<details>
<summary>4. Deploy factory.</summary>
<p>

- Create an env variable replacing `YOUR_ACCOUNT.testnet` with the name of the account you logged in with earlier:

```bash
export CONTRACT_ID=YOUR_ACCOUNT.testnet
```

- Deploy factory contract by running the following command from your current directory _(`ambassadors-dao-factory/`)_:

```bash
near deploy $CONTRACT_ID --wasmFile=res/ambassadors_dao_factory.wasm --accountId $CONTRACT_ID
```

</p>
</details>

<details>
<summary>5. Initialize factory.</summary>
<p>

```bash
near call $CONTRACT_ID new --accountId $CONTRACT_ID --gas 100000000000000
```

</p>
</details>

<details>
<summary>6. Define the parameters of the new DAO, its council, and create it.</summary>
<p>

- Define the council of your DAO:

```bash
export COUNCIL='["siddharthborderwala.testnet"]'
```

- Configure the name, purpose, and initial council members of the DAO and convert the arguments in base64:

```bash
export ARGS=`echo '{"name": "ca", "purpose": "Crypto Capabale Campus Ambassadors DAO", "council": '$COUNCIL'}' | base64`
```

- Create the new DAO!:

```bash
near call $CONTRACT_ID create "{\"name\": \"ca\", \"args\": \"$ARGS\"}" --accountId $CONTRACT_ID --amount 10 --gas 150000000000000
```

**Example Response:**

```bash

```

**Note:** If you see `false` at the bottom (after the transaction link) something went wrong. Check your arguments passed and target contracts and re-deploy.

</p>
</details>

<details>
<summary>7. Verify successful deployment and policy configuration.</summary>
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

- Verify that the name, purpose, metadata, and council are all configured correctly. Also note the following default values:

```json
{
  "roles": [
    {
      "name": "all",
      "kind": "Everyone",
      "permissions": ["*:AddProposal"],
      "vote_policy": {}
    },
    {
      "name": "council",
      "kind": { "Group": ["council-member.testnet", "YOUR_ACCOUNT.testnet"] },
      "permissions": [
        "*:Finalize",
        "*:AddProposal",
        "*:VoteApprove",
        "*:VoteReject",
        "*:VoteRemove"
      ],
      "vote_policy": {}
    }
  ],
  "default_vote_policy": {
    "weight_kind": "RoleWeight",
    "quorum": "0",
    "threshold": [1, 2]
  },
  "proposal_bond": "1000000000000000000000000",
  "proposal_period": "604800000000000",
  "bounty_bond": "1000000000000000000000000",
  "bounty_forgiveness_period": "86400000000000"
}
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
