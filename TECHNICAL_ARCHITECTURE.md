# Technical Architecture

## Digicus Programming Language

Heavily influenced by [Scratch](https://scratch.mit.edu/), Digicus is a net new, block-based, visual programming language consisting of a predefined set of *blocks* which users piece together to define Soroban smart contracts. 

### Compiler

The Digicus compiler provides a transcompilation from it's Visual block contracts to and from the Rust SDK. It does so via the following pipelines (where `.dtr` stands for `digicus textural representation`):

1. **[To Contract]**: Visual block contract --> `.dtr` file --> `.rs`
2. **[From Contract]**: `.rs` --> `.dtr` --> Visual block contract


This repository provides the tooling for:
(1) `.dtr` <--> `.rs`
(2) Visual block contract <--> `.dtr`

(1) Will be done through our Digicus transpiler (*digit*).

 Furthermore, we will supply some basic [standard library functions](./STANDARD_LIBRARY.md). Much of this will be determined from looking at:
* [contract examples](https://github.com/stellar/soroban-examples)
* [How to Build an SDK Docs](https://developers.stellar.org/docs/tools/sdks/build-your-own)
* [Rust SDK](https://github.com/stellar/rs-soroban-sdk/tree/main)
* [AssemblyScript SDK](https://github.com/Soneso/as-soroban-sdk)

(2) is just a visualization of `.dtr` files which structurally represent a block contract.

***

## Digicus IDE

The Digicus IDE is the recommended text editor for creating and visualizing Soroban Visual block contracts. Visualization is achieved by transforming `.dtr` files into colorful, draggable, editable "Lego-like" blocks. Once contracts are loaded (or in the case of creation, initialized), the IDE presents a host of tools to aid in development:

1. drag and drop interface with real-time error detection on block compatibility
2. a pseudo language server which provides real time feedback on common mistakes, security vulnerabilitites, etc.
3. simulation
4. testing
5. auto-complete

***