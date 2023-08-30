# StateDB Library

[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)


a specialized library for Ethereum state database management in Rust. Offering support for both standard (std) and customized (tstd) environments, statedb introduces advanced features including a stateless optimized state trie, in-memory caching, and fast state rollback.

## Features
* **Stateless Optimized State Trie**: Designed for efficient handling of stateless operations, providing a streamlined and responsive experience.
* **In-Memory Cache for Account/Storage State**: Robust caching mechanisms that facilitate quick access to account and storage state information, minimizing delays.
* **Fast State Rollback**: Built-in support for rapid state rollback, allowing for secure and quick recovery of previous states.

## Unified Interface for std/tstd
statedb provides a unified interface that makes it easy to write Rust code that operates seamlessly within both std and tstd environments, ensuring broad compatibility and usability.
