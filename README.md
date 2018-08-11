# dxtracker

A simple Rust Telnet CLI, which connects to a DX Cluster Server. It also adds filter functionality to get specific calls.
Run your local dxcluster!

[![Build Status](https://travis-ci.org/DD5HT/dxtracker.svg?branch=master)](https://travis-ci.org/DD5HT/dxtracker)

## Usage
You can either use it as a library or use the included command line tool.
Just run:
```
cargo build --release
cd target/release/
./dxtool -h
./dxtool -i 
```

## Disclaimer

I also use this project to experiment with git/ github TDD and Travis CI so chances are that it breaks.
A stable release is planned for the future.

The code only runs on nightly at the moment, but it could easily ported to a stable release.
(Modify the delete function)
