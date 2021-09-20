# Themelio Block eXplorer MicroService

Early alpha quality. This is the code behind [Melscan](https://scan.themelio.org)

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
  "previous": "b60e3d3856742f9d80dce18177d1f63161548b46fae8a65dbc4ff07d61bf011a",
  "height": 413788,
  "history_hash": "09eab74ef9ebdad09044e2387e7aba629a9346d0bc81294ee63e727e4117b9af",
  "coins_hash": "ce331ddaca192e1de3c6f4b9f99ec5f2e552c8c28be23fe549ba8ada09a21af0",
  "transactions_hash": "0000000000000000000000000000000000000000000000000000000000000000",
  "fee_pool": 14550215940,
  "fee_multiplier": 1003,
  "dosc_speed": 22369621,
  "pools_hash": "05b0cd30c2e8f5850bda64ce6135547fd765db91862d28bd986cb001e871fa0e",
  "stakes_hash": "92707d70a55d32778eced56feb672a3e25a8dc5d5278adb542e61edd2f2d3f44"
}
```

## Get a particular block

### Request

```
GET /raw/blocks/<height>
```

where

- `<height>`: block height

### Response

A block header, like `/raw/latest`

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

| Field     | Type    | Description                                                                                                                               |
| --------- | ------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `kind`    | integer | See [kind definitions](https://github.com/themeliolabs/themelio-stf/blob/1d7b711f1c0aeb6813b7e87f3391dd67960427d8/src/transaction.rs#L36) |
| `inputs`  | array   | Array of **CoinID** objects (see definition below)                                                                                        |
| `outputs` | array   | Array of **CoinData** objects                                                                                                             |
| `fee`     | integer | µMEL paid as fees                                                                                                                         |
| `scripts` | array   | Array of hex-encoded MelVM covenants that encumber the coins being spent by this transaction                                              |
| `data`    | hex     | Arbitrary binary data                                                                                                                     |
| `sigs`    | array   | Array of hex-encoded malleable signatures                                                                                                 |

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
|`denom`|hex|Denomination of coin. `6d` is MEL and `73` is SYM
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
      "denom": "73",
      "additional_data": ""
    },
    {
      "covhash": "t1m9v0fhkbr7q1sfg59prke1sbpt0gm2qgrb166mp8n8m59962gdm0",
      "value": 51,
      "denom": "6d",
      "additional_data": ""
    }
  ],
  "fee": 9,
  "scripts": [
    "420009f100000000000000000000000000000000000000000000000000000000000000064200005050f020e14baf3290821fd234d5b4e15fe7fa04dc1fda7a4ab64e58839e35e69b56d8fe420001320020"
  ],
  "data": "7323dcb65513b84470a76339cdf0062d47d82e205e834f2d7159684a0cb3b5ba0204fc00093d00",
  "sigs": [
    "d7f45923e161fddb999b2ff4887144c57b2c87b5436e7f9ea5326013ae2ed4c72c103fed232af0d7f62aaffeb0008a11d49dfdd59013e66fb9eb1e6ffe05bd04",
    "d7f45923e161fddb999b2ff4887144c57b2c87b5436e7f9ea5326013ae2ed4c72c103fed232af0d7f62aaffeb0008a11d49dfdd59013e66fb9eb1e6ffe05bd04"
  ]
}
```

### Get an unspent coin

TODO

### Get a Melswap pool

TODO
