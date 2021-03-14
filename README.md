# wildptr
A Wildpointer RFC command line browser written in Rust

## Features

- List all data on a Wildpointer RFC
- List all or a single RFC
- Well documented code
- Custom command line argument parser
- Written in Rust
- Linux and Mac support

## Usage
```
usage: wildptr [-v | --verbose] 

arguments:
    list - list all RFCs
    rfc <arg> - specify info on a specific rfc
```

## Installation

1. [Download](https://github.com/harrego/wildptr/releases) the latest release
2. Find the `wildptr` binary in the download
3. `sudo cp wildptr /usr/local/bin/ && sudo chmod +x /usr/local/bin/wildptr`

### From source
1. Clone the project
2. Build with the Rust toolchain: `rust build --release`
3. Find the binary in `target/release/wildptr`
4. `sudo cp target/release/wildptr /usr/local/bin/ && sudo chmod +x /usr/local/bin/wildptr`

## What are Wildpointer RFCs?

An extract taken from the [Wildpointer RFC Board Mirror](https://wildpointer.harry.city):

> The Wildpointer project is unique when compared to other programming languages and frameworks, instead of the project being open source with a handful of transparent maintainers accepting contributions from everyone, the Wildpointer project is only semi-open-source, at present the codebase is 91.3% closed source. Even with a small amount of the codebase being "open-source", these parts are only accessible to vendors, a list that only contains the representatives from each language that write bindings to the Wildpointer project. It is estimated that only 50 third-party people, not companies but people, have access to the open-source parts of the codebase and as such it has never leaked.

## API data source

The Wildpointer RFC Board Mirror discontinued its API and as such I have mirrored the last existing version on my server. This tool makes request to that API to surface data.
