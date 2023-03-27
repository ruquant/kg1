# Sequencer

The first version of the sequencer is designed for the [tzwitter application](https://gitlab.com/marigold/pistachio/-/tree/main/09_tzwitter_app)

# TODO

Instead of directly process the message in the `POST /operations` endpoint, we have to use a queue, maybe the tokio::mpsc would be a nice to have

# Kernel constraints?

The kernel have to respect some conditions:

- ???
