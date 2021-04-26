# WASM Testbed

This crate provides a minimum context for a Substrate Runtime WASM to be loaded and executed.
Do not expect to be transferring funds here, we are disconnected from any chain storage, network, etc...

Still there are a few interesting we can do such as:

- retreive the metadata (directly from the wasm)
- get the core version information
