
# compile the kernel
cargo build --release --target wasm32-unknown-unknown --manifest-path kernel/Cargo.toml

# delete previous rollup
rm -rf rollup

# copy the kernel wasm to the rollup directory
mkdir -p rollup
cp ../target/wasm32-unknown-unknown/release/kernel.wasm ./rollup/kernel.wasm

# using wasm-strip to strip the size of the kernel
# first check if it is already install or not
which wasm-strip > /dev/null || (echo "wasm-strip need to be installed" && echo "https://github.com/WebAssembly/wabt" && exit 1)
wasm-strip ./rollup/kernel.wasm


# first check and see if the tezos-smart-rollup-installer has been installed
which tezos-smart-rollup-installer > /dev/null || (echo "tezos-smart-rollup-installer need to be installed"  && "cargo install tezos-smart-rollup-installer --git https://gitlab.com/tezos/tezos" && exit 1)

# using the smart-rollup-installer
# it will generate the installer.hex and then split the kernel
smart-rollup-installer get-reveal-installer --upgrade-to rollup/kernel.wasm --output rollup/installer.hex --preimages-dir rollup/wasm_2_0_0 

# originate kernel installer as a bootstrap for the rollup
KERNEL_INSTALLER=$(cat rollup/installer.hex)

# install octez by opam
which octez-client > /dev/null || (echo "octez-client need to be install" && echo "Section Install Octez OPAM packages: https://tezos.gitlab.io/introduction/howtoget.html" && exit 1)

# before originate the smart rollup with the kernel, generate account, we need to set up octez-node
# and then run the octez-node:
# https://teztnets.xyz/mondaynet-about the network name change each week
# remove any .tezos-node, .tezos-client if you want a fresh start
NODE_CONFIG=$(octez-node config init --network https://teztnets.xyz/mondaynet-2023-04-17)
NODE=$(octez-node run --rpc-addr 127.0.0.1:8732)

# create a new account if not
# alice tz1XyoKMeg2CMTw52oQBQhBCszVfdEBsjph9
alice=$(octez-client gen keys alice)

# use Monday net faucet to fund enough tez for alice: https://teztnets.xyz/#mondaynet
# after funding check the balance of alice (request 10001 tz), it will take sometime for 
# the balance show up in the account
balance=$(octez-client get balance for alice)

# list a list of accounts
accounts=$(octez-client list known addresses)

# originate smart rollup with the kernel installer, we need to wait for the octez-node
# to be bootstrapped
SOR_ADDR=$(octez-client originate smart rollup from alice \
           of kind wasm_2_0_0 \
           of type bytes \
           with kernel "${KERNEL_INSTALLER}" \
           --burn-cap 999 | grep "Address:" | awk '{print $2}')

# after the smart rollup kernel has been originated, we can start to run the smart rolup node with this address
# sr1ShQXXwmpTg5DLmh1U4yQdfrdzVCW6i8UE
octez-smart-rollup-node-alpha init operator config for "${SOR_ADDR}" with operators alice --data-dir rollup

# use xxd to generate hex for input.json (external message)
EMESSAGE=$(xxd -ps -c 0 kernel/inputs.json)

# send external inbox message to rollup
send_message=$(octez-client -p ProtoALphaAL \
               send smart rollup message "hex:[ \"${EMESSAGE}\" ]" \
               from alice)

# debug the kernel by the debuger tool
octez-smart-wasm-debuger rollup/kernel.wasm --inputs kernel/inputs.json

##########################################
# Deploy dungeon game with sequencer

# remove the old storage of sequencer to start a fresh game
rm -rf /tmp/sequencer-storage

# run tezos-node
cd tezos
./octez-node run --rpc-addr 127.0.0.1

# run sequencer-http server
cd pistachio/sequencer/sequencer-http
cargo run

# load React App
cd pistachio/10_dungeon/app
yarn install
yarn start


