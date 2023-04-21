# Dungeon game

One can follow the steps in `deploy.sh` to deploy the missing steps. This readme only show the major result of dungeon game that run on the smart rollup node, send external message and the debugging tool. Using the explore https://explorus.xyz/soru on MondayNet to search for the SORU address.

## Originate the smart rollup node with the kernel

```
~/tezos/octez-client originate smart rollup from tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9 of kind wasm_2_0_0 of type bytes with kernel "${KERNEL_INSTALLER}" --burn-cap 999

Warning:

                 This is NOT the Tezos Mainnet.

           Do NOT use your fundraiser keys on this network.

Node is bootstrapped.
Estimated gas: 554.588 units (will add 0 for safety)
Estimated storage: no bytes added
Estimated gas: 1848.229 units (will add 100 for safety)
Estimated storage: 6552 bytes added (will add 20 for safety)
Operation successfully injected in the node.
Operation hash is 'opUjZc7tj4HPAs1eNe56zi4GRaTVk6fz7A9zNzQNjuo769fU49Q'
Waiting for the operation to be included...
Operation found in block: BKk6DuBb6Gx69tPZt41VJ8AHTP6UogDNoWW5dza6wuUjT7cDj4T (pass: 3, offset: 0)
This sequence of operations was run:
  Manager signed operations:
    From: tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9
    Fee to the baker: ꜩ0.000314
    Expected counter: 1874
    Gas limit: 555
    Storage limit: 0 bytes
    Balance updates:
      tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9 ... -ꜩ0.000314
      payload fees(the block proposer) ....... +ꜩ0.000314
    Revelation of manager public key:
      Contract: tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9
      Key: edpkuSuiTwpLGyLC7MESCEpsr5WgtWN1CWKvkw7HUP5jp1cffxNhMW
      This revelation was successfully applied
      Consumed gas: 554.488
  Manager signed operations:
    From: tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9
    Fee to the baker: ꜩ0.011935
    Expected counter: 1875
    Gas limit: 1949
    Storage limit: 6572 bytes
    Balance updates:
      tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9 ... -ꜩ0.011935
      payload fees(the block proposer) ....... +ꜩ0.011935
    Smart rollup origination:
      Kind: wasm_2_0_0
      Parameter type: bytes
      Kernel Blake2B hash: 'f24c9f14a0ab4f414716c63abbdf02245090b3acc5fa177ede9d53bef6aaff3e'
      This smart rollup origination was successfully applied
      Consumed gas: 1848.269
      Storage size: 6552 bytes
      Address: sr1ShQXXwmpTg5DLmh1U4yQdfrdzVCW6i8UE
      Genesis commitment hash: src13LXXm9pNJ4FUAE3Kma2smam71TK85nDKdTZKcE9n8qXsz4AcqM
      Balance updates:
        tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9 ... -ꜩ1.638
        storage fees ........................... +ꜩ1.638

The operation has only been included 0 blocks ago.
We recommend to wait more.
Use command
  octez-client wait for opUjZc7tj4HPAs1eNe56zi4GRaTVk6fz7A9zNzQNjuo769fU49Q to be included --confirmations 1 --branch BLwi4dz4WscQU1zeiS251nicVigaCHWta8rzEaNhaD2xLn8Wns1
and/or an external block explorer.

```

## Smart rollup node

```
~/tezos/octez-smart-rollup-node-alpha init operator config for sr1ShQXXwmpTg5DLmh1U4yQdfrdzVCW6i8UE with operators tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9 --data-dir rollup
```

Smart rollup node configuration is written in `rollup/config.json`. The content of `rollup/config.json`:

```
{ "smart-rollup-address": "sr1ShQXXwmpTg5DLmh1U4yQdfrdzVCW6i8UE",
  "smart-rollup-node-operator":
    { "publish": "tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9",
      "add_messages": "tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9",
      "cement": "tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9",
      "refute": "tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9" },
  "fee-parameters": {}, "mode": "operator" }
```

## Send external message

```
~/tezos/octez-client -p ProtoALphaAL send smart rollup message "hex:[ \"5b0a202020205b0a20202020202020207b0a2020202020202020202020202265787465726e616c223a20223031220a20202020202020207d2c0a20202020202020207b0a2020202020202020202020202265787465726e616c223a20223031220a20202020202020207d0a202020205d0a5d\" ]" from tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9
```

where hex output of:

```
xxd -ps -c 0 kernel/inputs.json
```

Output

```
Warning:

                 This is NOT the Tezos Mainnet.

           Do NOT use your fundraiser keys on this network.

Node is bootstrapped.
Estimated gas: 174.579 units (will add 100 for safety)
Estimated storage: no bytes added
Operation successfully injected in the node.
Operation hash is 'ooK6ecJh3MBvtuzfMzhrNr45NWmVXZyiQ2ihtSvYViCMbk7snJr'
Waiting for the operation to be included...
Operation found in block: BKvVgdMkTkeGQ7VysrtRNZFyWK93Fs52WJaZuGSuNLZqqmgPvK8 (pass: 3, offset: 0)
This sequence of operations was run:
  Manager signed operations:
    From: tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9
    Fee to the baker: ꜩ0.000375
    Expected counter: 1876
    Gas limit: 275
    Storage limit: 0 bytes
    Balance updates:
      tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9 ... -ꜩ0.000375
      payload fees(the block proposer) ....... +ꜩ0.000375
    Smart rollup messages submission:
      This smart rollup messages submission was successfully applied
      Consumed gas: 174.512

The operation has only been included 0 blocks ago.
We recommend to wait more.
Use command
  octez-client wait for ooK6ecJh3MBvtuzfMzhrNr45NWmVXZyiQ2ihtSvYViCMbk7snJr to be included --confirmations 1 --branch BMc9ETqkaFufUkgw1qvMVoYJav5r1SQNARjaEa91XDtcvJaM2cG
and/or an external block explorer.
```

Checking the operation has been included in a block:

```
 ~/tezos/octez-client wait for ooK6ecJh3MBvtuzfMzhrNr45NWmVXZyiQ2ihtSvYViCMbk7snJr to be included --confirmations 1 --branch BMc9ETqkaFufUkgw1qvMVoYJav5r1SQNARjaEa91XDtcvJaM2cG
Warning:

                 This is NOT the Tezos Mainnet.

           Do NOT use your fundraiser keys on this network.

Operation found in block: BKvVgdMkTkeGQ7VysrtRNZFyWK93Fs52WJaZuGSuNLZqqmgPvK8 (pass: 3, offset: 0)
Operation received 1 confirmations as of block: BLa1oDahg3qunzpJa3RxRTJ734d3aCL1X6fqjczqjsot8sxdgC4
```

## Debug kernel

Run with two examples:

- The `inputs.json`: let x move up two times.

```
[
    [
        {
            "external": "01"
        },
        {
            "external": "01"
        }
    ]
]
```

```
octez-smart-rollup-wasm-debugger rollup/kernel.wasm --inputs kernel/inputs.json
```

Steps

```
tarting debugger REPL. Enter command 'help' for usage.
> show status
Status: Waiting for input
Internal_status: Collect
> load inputs
Loaded 2 inputs at level 0
> show status
Status: Evaluating
Internal_status: Snapshot
> show inbox
Inbox has 5 messages:
{ raw_level: 0;
  counter: 0
  payload: Start_of_level }
{ raw_level: 0;
  counter: 1
  payload: Info_per_level {predecessor_timestamp = 1970-01-01T00:00:00-00:00; predecessor = BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M} }
{ raw_level: 0;
  counter: 2
  payload: 01 }
{ raw_level: 0;
  counter: 3
  payload: 01 }
{ raw_level: 0;
  counter: 4
  payload: End_of_level }
> step kernel_run
Hello worldEvaluation took 11000000000 ticks so far
Status: Waiting for input
Internal_status: Collect
> show key /state/player/x_pos
00000010
> show key /state/player/x_pos
00000010
```

- The `inputs.json`: let x move up one times, y move down 1 times

```
[
    [
        {
            "external": "01"
        },
        {
            "external": "02"
        }
    ]
]
```

Steps:

```
> show status
Status: Waiting for input
Internal_status: Collect
> load inputs
Loaded 2 inputs at level 0
> show status
Status: Evaluating
Internal_status: Snapshot
> show inbox
Inbox has 5 messages:
{ raw_level: 0;
  counter: 0
  payload: Start_of_level }
{ raw_level: 0;
  counter: 1
  payload: Info_per_level {predecessor_timestamp = 1970-01-01T00:00:00-00:00; predecessor = BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M} }
{ raw_level: 0;
  counter: 2
  payload: 01 }
{ raw_level: 0;
  counter: 3
  payload: 02 }
{ raw_level: 0;
  counter: 4
  payload: End_of_level }
> step kernel_run
Hello worldEvaluation took 11000000000 ticks so far
Status: Waiting for input
Internal_status: Collect
> show key /state/player/x_pos
00000010
> show key /state/player/y_pos
00000011
> show key /state/player/x_pos
```

# Sequencer

To use the sequencer you have to edit the sequencer-http/Cargo.toml, and add a dependency to your kernel:

```toml
# sequencer-http/Cargo.toml
kernel = {path = "../../10_dungeon/kernel"}
```

> Because you specify your custom kernel as a dependency of the sequencer
> You don't have to compile your kernel on new changes
> You just have to build or to run the sequencer

Then you can compile the sequencer-http crate and resolve any issues:

```rust
// sequencer-http/src/main.rs
impl Kernel for MyKernel {
    fn entry<R: Runtime>(host: &mut R) {
        kernel::entry(host)
    }
}
```

> You need to have a tezos node running
> You can provide any node

```rust
let tezos_node_uri = "http://localhost:18731"; // Update this URI
```

Then you can start the sequencer:

```bash
cd sequencer/sequencer-http
cargo run
```

To submit an operation to the sequencer, you can use curl:
(Only external operation can added)

The content type of the request is a json.
The sequencer is exposing an post http endpoint "/operations" to submit an operation with the following payload:

```json
{ "data": "01" }
```

Where "01" refers to your hexadecimal operation.

```bash
curl -H "Content-Type: application/json" -X POST -d '{"data": "01" }' http://localhost:8080/operations
> Operation submitted
```

And the you can retrieve your state.
The sequencer is exposing two get endpoints to retrieve your optimist state:

- one to retrieve the value "/state/value?path=..."
- one to retrieve the list of sub keys of a path "/state/subkeys?path=..."

```bash
curl "http://127.0.0.1:8080/state/value?path=/state/player/y_pos"
> 0000000000000007
curl "http://127.0.0.1:8080/state/subkeys?path=/state/player"
> ["x_pos", "y_pos"]
```

## Restart from a fresh sequencer

The sequencer is saving its state into the filesystem under the folder `/tmp/sequencer-storage`
If you want to restart a sequencer from a fresh state you just have to delete this folder
Otherwise the sequencer will use this old state as the current one
