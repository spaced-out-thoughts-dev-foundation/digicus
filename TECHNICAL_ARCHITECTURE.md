# Technical Architecture

## Overview

### The Problem
**Consider the following:**
1. While Rust is one of the most admired languages, it is far from one of the most popular (https://survey.stackoverflow.co/2023/#section-admired-and-desired-programming-scripting-and-markup-languages)
2. Scratch is an incredible first language, in fact it’s also more popular than Rust (https://www.tiobe.com/tiobe-index/)
3. More mature and seasoned smart contract platforms are complicated and error prone (https://www.mdpi.com/2624-800X/2/2/19)
4. 65% of the general population are visual learners (https://gitnux.org/visual-learner-statistics/#:~:text=Highlights%3A%20The%20Most%20Important%20Visual,children%20identify%20as%20visual%20learners.)

Furthermore, consider the following quotes from the Rust SDK Docs:
* “Before submitting to the network, developers should inspect the resulting Wasm binary emitted by the Rust compiler to ensure that it contains only the intended code and data, and is as small as possible.”
* “​​Developers must understand the difference between code that is compiled-in to Wasm modules for deployment and code that is conditionally compiled for testing. See debugging contracts for more details.”

While the Soroban developer experience is an improvement over most smart contract platforms in existence today, technical bar is still very high.

### The Solution

**Hypothesis:** the existence of a visual, block-based programming language which can translate to and from Soroban smart contracts will promote comprehension, adaption, and best practices helping the young, Stellar smart contract community reach maturity at an expedited rate.

Generally described as “scratch for smart contracts”, Digicus (hybrid Soroban) is a compiler which functions as an abstraction layer on top of existing Soroban tooling. Digicus allows users to create smart contracts by piecing together blocks (much like Legos) as well as visualize existing smart contracts.

***

## Digicus Programming Language

Heavily influenced by [Scratch](https://scratch.mit.edu/), Digicus is a net new, block-based, visual programming language consisting of a predefined set of *blocks* which users piece together to define Soroban smart contracts. 

### Compiler

The Digicus compiler provides a transcompilation from it's Visual block contracts to and from `.wasm` Soroban smart contracts. It does so via the following pipelines (where `.bbtr` stands for `block based text representation`):

1. **[To Contract]**: Visual block contract --> `.bbtr` file --> `.wat` --> `.wasm`
2. **[From Contract]**: `.wasm` --> `.wat` --> `.bbtr` --> Visual block contract

`.wat` <--> `.wasm` is possible via the [Wasm Binary Toolkit or WABT](https://github.com/WebAssembly/wabt).

This repository provides the tooling for:
(1) `.bbtr` <--> `.wat`
(2) Visual block contract <--> `.bbtr`

(1) will be aided by careful translation of the [`.wat` spec](https://webassembly.github.io/spec/core/text/index.html) into the [`.bbtr spec`](./BBTR_SPEC.md). Furthermore, we will supply some basic [standard library functions](./STANDARD_LIBRARY.md). Much of this will be determined from looking at:
* [contract examples](https://github.com/stellar/soroban-examples)
* [How to Build an SDK Docs](https://developers.stellar.org/docs/tools/sdks/build-your-own)
* [Rust SDK](https://github.com/stellar/rs-soroban-sdk/tree/main)
* [AssemblyScript SDK](https://github.com/Soneso/as-soroban-sdk)

(2) is just a visualization of `.bbtr` files which structurally represent a block contract.

***

## Digicus IDE

The Digicus IDE is the recommended text editor for creating and visualizing Soroban Visual block contracts. Visualization is achieved by transforming `.bbtr` files into colorful, draggable, editable "Lego-like" blocks. Once contracts are loaded (or in the case of creation, initialized), the IDE presents a host of tools to aid in development:

1. drag and drop interface with real-time error detection on block compatibility
2. a pseudo language server which provides real time feedback on common mistakes, security vulnerabilitites, etc.
3. simulation
4. testing
5. auto-complete
