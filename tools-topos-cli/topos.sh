#!/bin/bash

## Issues
#
# In this example, the exposed port for prometheus etc are kept the same
# there is port conflict, too dirty to illustrate completely outside of a docker-compose
#
# At least, the edge component is running, and the boot node too

set -e

POLYGON_EDGE=polygon-edge
TOPOS=topos
SUBNET_ID=topos
NODE_PREFIX=data

TOPOS_HOME=$(mktemp -d)

# Depedencies
which toml # cargo install toml-cli
which $TOPOS
which $POLYGON_EDGE
echo $TOPOS_HOME

NUMBER_OF_NODES="${NUMBER_OF_NODES:-4}"

#
# Create the nodes
#
for N in `seq 1 $NUMBER_OF_NODES`
do
    $TOPOS node init --name $NODE_PREFIX-$N --home $TOPOS_HOME

    node_config=$TOPOS_HOME/node/$NODE_PREFIX-$N/config.toml

    # Change edge libp2p port
    toml set $node_config edge.libp2p "127.0.0.1:$((1477+$N))" > $node_config.bak
    mv $node_config.bak $node_config

    # Change TCE libp2p port
    toml set $node_config tce.libp2p-api-addr "0.0.0.0:$((9089+$N))" > $node_config.bak
    mv $node_config.bak $node_config
done

# Bootnode
secrets=$("$POLYGON_EDGE" secrets output --data-dir $TOPOS_HOME/node/$NODE_PREFIX-1 --json)
echo $secrets
BOOTNODE_ID=$(echo $secrets | jq -r '.node_id')
BOOTNODE_ADDRESS=$(echo $secrets | jq -r '.address')
BOOTNODE_DOMAIN_NAME="${BOOTNODE_DOMAIN_NAME:-127.0.0.1}"

#
# Create the genesis
#
CHAIN_ID="${CHAIN_ID:-100}" # 100 is Edge's default value
CHAIN_CUSTOM_OPTIONS=$(tr "\n" " " << EOL
--block-gas-limit 10000000
--epoch-size 10
--chain-id $CHAIN_ID
--name polygon-edge-docker
--premine 0x228466F2C715CbEC05dEAbfAc040ce3619d7CF0B:0xD3C21BCECCEDA1000000
--premine 0xca48694ebcB2548dF5030372BE4dAad694ef174e:0xD3C21BCECCEDA1000000
--premine 0x4AAb25B4fAd0Beaac466050f3A7142A502f4Cf0a:1000000000000000000000
EOL
)

mkdir -p $TOPOS_HOME/subnet/$SUBNET_ID/

# NOTE: `polygon-edge genesis` want to be called in the same folder of the node configs
#        because of the `--ibft-validators-prefix-path` which is actually not expecting a path
pushd .
cd $TOPOS_HOME/node/
$POLYGON_EDGE genesis $CHAIN_CUSTOM_OPTIONS \
                      --dir $TOPOS_HOME/subnet/$SUBNET_ID/genesis.json \
                      --consensus ibft \
                      --ibft-validators-prefix-path $NODE_PREFIX- \
                      --bootnode /ip4/"$BOOTNODE_DOMAIN_NAME"/tcp/1478/p2p/$BOOTNODE_ID \
                      --premine=$BOOTNODE_ADDRESS:1000000000000000000000
popd

echo $TOPOS_HOME
cat $TOPOS_HOME/subnet/$SUBNET_ID/genesis.json | jq

#
# Launch the nodes
#

# Launch the bootnode first
nohup $TOPOS node up --name $NODE_PREFIX-1 --home $TOPOS_HOME > nohup.bootnode.$$ &
sleep 2

# Launch the other nodes
for N in `seq 2 $NUMBER_OF_NODES`
do
    nohup $TOPOS node up --name $NODE_PREFIX-$N --home $TOPOS_HOME > nohup.$N.$$ &
done
