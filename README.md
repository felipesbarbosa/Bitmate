# Bitmate

![Alt text](/images/btc_logo.png)

Bitmate is an open-source Bitcoin library implemented in Rust. It provides a set of tools and utilities for working with Bitcoin, including key generation, transaction creation and signing, address validation, and more.

## Features

Take notice that most of the below Features are still under development:

- **Bitcoin Key Generation**: Easily generate private keys and corresponding Bitcoin addresses.
- **Transaction Creation**: Create Bitcoin transactions and sign them using private keys.
- **Address Validation**: Validate Bitcoin addresses to ensure their integrity.
- **Scripting Support**: Build and execute Bitcoin scripts for advanced functionality.
- **Network Support**: Connect to Bitcoin network nodes and interact with the Bitcoin network.

## Installation

To use Bitmate as it is today you will need to clone the repository and perform a cargo build. For future releases there will be an installation script and packages for linux distributions.

1. Clone Bitmate GitHub repository:


```sh
git clone https://github.com/seu-usuario/bitmate.git
```


1. Navigate to the project directory:


```sh
cd bitmate
```


1. Compile and install Bitmate using Cargo:


```sh
cargo install --path .
```


1. Build the the project:


```sh
cargo build
```


1. Execute it:


```sh
cargo run
```