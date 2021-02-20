# Copper
Cuz it does not Rust \
See what I did there 😄😄\
[![Build Status](https://github.com/tsvetelinkostadinv/Copper/workflows/Rust/badge.svg)](https://github.com/tsvetelinkostadinv/Copper)

## Synopsis
The aim is to develop the infrastructure for `distributed computing` with Rust so a heavy task can be split between multiple computers.

In order not to offend the terminology is chosen accordingly:
- server - the computer which acts as a server and sends out commands
- client - the machines actually doing the work.

## Description
The application has 2 main components
1. Server, which accepts connections and sends out tasks
1. Client which receives the task, performs it and returns a result

## Example
1. Something dear to my heart: Quantum computer simulations, where a single computer calculates the final state and all the other collapse it the given number of times. Then the results are sent back to the *master* and are aggregated to be shown on the screen.
1. Number crunching! Like analyzing the *min*, *max*, *count* etc. of given trait in a dataset.
1. A distributed database hidden behind a single endpoint and can be quarried. **TO BE THOUGHT ABOUT**
