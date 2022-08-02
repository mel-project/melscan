## Overview

[Melscan](https://scan.themelio.org) is a SvelteKit-Rust hybrid app.

More specifically, the `frontend` directory hosts a SvelteKit app, which talks to the Rust backend in `backend`. `dev.sh` automatically sets up an auto-refreshing server.

# API docs

Melscan exposes a simple JSON API at `/raw`, documented below using `https://scan.themelio.org` as an example.

## Latest block header (network summary)

### Request

```
GET /raw/latest
```

### Response

A JSON object representing a **block header**. All block headers have fields:

| Field            | Type    | Description                    |
| ---------------- | ------- | ------------------------------ |
| `network`        | integer | 1 for testnet, 255 for mainnet |
| `previous`       | hex     | hash of previous block header  |
| `height`         | integer | block height                   |
| `history_hash`   | hex     | root hash of history SMT       |
| `coins_hash`     | hex     | root hash of transactions SMT  |
| `fee_pool`       | integer | remaining µMEL in fee pool     |
| `fee_multiplier` | integer | fee multiplier                 |
| `dosc_speed`     | integer | DOSC-minting speed             |
| `pools_hash`     | hex     | root hash of pools SMT         |
| `stakes_hash`    | hex     | root hash of stakes SMT        |

### Example

```
$ curl -s https://scan.themelio.org/raw/latest | jq
```

```json
{
  "network": 255,
  "previous": "a4934ab2b2f4d68aeab52df9fe26921c056ec2ed64564ed00d2e97242df9224f",
  "height": 1358352,
  "history_hash": "2a63a79ccbd76531202e48ba4e8b099ad77418955beb9c8cae5905e4166b361d",
  "coins_hash": "c8a4e76b2e588d18ffa3628fe1df44f04ff3c3f92a529e35b9c872930007e3ff",
  "transactions_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "fee_pool": 1713827110273,
  "fee_multiplier": 997,
  "dosc_speed": 268435456,
  "pools_hash": "32861753a8c2abd91e28bcfe31707f53a4b6254888aa783d94f72d386b76bb5b",
  "stakes_hash": "d66cdcd5c2c4250959bcb05ab9d132ef2dc0e9c3e300780819466cb9cdf81dda"
}
```

## Summarize a particular block

### Request

```
GET /raw/blocks/<height>/summary
```

where

- `<height>`: block height

### Response

A block header, like `/raw/latest`

### Example

```
$ curl -s https://scan.themelio.org/raw/blocks/1358352/summary | jq
```

```json
{
  "header": {
    "network": 255,
    "previous": "a4934ab2b2f4d68aeab52df9fe26921c056ec2ed64564ed00d2e97242df9224f",
    "height": 1358352,
    "history_hash": "2a63a79ccbd76531202e48ba4e8b099ad77418955beb9c8cae5905e4166b361d",
    "coins_hash": "c8a4e76b2e588d18ffa3628fe1df44f04ff3c3f92a529e35b9c872930007e3ff",
    "transactions_hash": "0000000000000000000000000000000000000000000000000000000000000000",
    "fee_pool": 1713827110273,
    "fee_multiplier": 997,
    "dosc_speed": 268435456,
    "pools_hash": "32861753a8c2abd91e28bcfe31707f53a4b6254888aa783d94f72d386b76bb5b",
    "stakes_hash": "d66cdcd5c2c4250959bcb05ab9d132ef2dc0e9c3e300780819466cb9cdf81dda"
  },
  "total_weight": 0,
  "reward_amount": 26151325,
  "transactions": [],
  "header_hash": "9bc59bc6c30f6afc2fdaa5e1c5f8fe6b6e264b325f998d9dd678ad0144980bb0",
  "total_fees": 0,
  "fee_multiplier": 0.0152130126953125
}
```

## Get a transaction

### Request

```
GET /raw/blocks/<height>/transactions/<txhash>
```

where

- `<height>`: height of block that the transaction was confirmed in
- `<txhash>`: transaction hash

### Response

A JSON object representing a **transaction**. This has fields:

| Field       | Type    | Description                                                                                                                               |
| ----------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `kind`      | integer | See [kind definitions](https://github.com/themeliolabs/themelio-stf/blob/1d7b711f1c0aeb6813b7e87f3391dd67960427d8/src/transaction.rs#L36) |
| `inputs`    | array   | Array of **CoinID** objects (see definition below)                                                                                        |
| `outputs`   | array   | Array of **CoinData** objects                                                                                                             |
| `fee`       | integer | µMEL paid as fees                                                                                                                         |
| `covenants` | array   | Array of hex-encoded MelVM covenants that encumber the coins being spent by this transaction                                              |
| `data`      | hex     | Arbitrary binary data                                                                                                                     |
| `sigs`      | array   | Array of hex-encoded malleable signatures                                                                                                 |

The **CoinID** object has these fields:
| Field | Type | Description |
|--|--|--|
|`txhash`|hex|Hash of transaction that created this coin
|`index`|integer|Index of this transaction. For example, to refer to the first output of some transaction, use `index=0`

The **CoinData** object has these fields:
| Field | Type | Description |
|--|--|--|
|`covhash`|hex|Hash of destination covenant, in "address" format
|`value`|integer|Face value of coin, in millionths
|`denom`|string|Denomination of coin.
|`additional_data`|hex|Arbitrary data

### Example

```
curl -s https://scan.themelio.org/raw/blocks/344778/transactions/42a18b67ca46486067662554fcb5dd8ccea6405cabe3268827ef8679f826f868 | jq
```

```json
{
  "kind": 16,
  "inputs": [
    {
      "txhash": "442cca83ef8f2afabd02d9d484d8f8e6b330c0ab8f865f2bae5c31d6892a28d1",
      "index": 0
    },
    {
      "txhash": "442cca83ef8f2afabd02d9d484d8f8e6b330c0ab8f865f2bae5c31d6892a28d1",
      "index": 1
    }
  ],
  "outputs": [
    {
      "covhash": "t1m9v0fhkbr7q1sfg59prke1sbpt0gm2qgrb166mp8n8m59962gdm0",
      "value": 4000000,
      "denom": "SYM",
      "additional_data": ""
    },
    {
      "covhash": "t1m9v0fhkbr7q1sfg59prke1sbpt0gm2qgrb166mp8n8m59962gdm0",
      "value": 51,
      "denom": "MEL",
      "additional_data": ""
    }
  ],
  "fee": 9,
  "covenants": [
    "420009f100000000000000000000000000000000000000000000000000000000000000064200005050f020e14baf3290821fd234d5b4e15fe7fa04dc1fda7a4ab64e58839e35e69b56d8fe420001320020"
  ],
  "data": "7323dcb65513b84470a76339cdf0062d47d82e205e834f2d7159684a0cb3b5ba0204fc00093d00",
  "sigs": [
    "d7f45923e161fddb999b2ff4887144c57b2c87b5436e7f9ea5326013ae2ed4c72c103fed232af0d7f62aaffeb0008a11d49dfdd59013e66fb9eb1e6ffe05bd04",
    "d7f45923e161fddb999b2ff4887144c57b2c87b5436e7f9ea5326013ae2ed4c72c103fed232af0d7f62aaffeb0008a11d49dfdd59013e66fb9eb1e6ffe05bd04"
  ]
}
```

## Get a time series

(TBD))
