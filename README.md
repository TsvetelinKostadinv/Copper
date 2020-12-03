# Copper
Cuz it does not Rust \
See what I did there 😄😄

## Synopsis
The aim is to develop the infrastructure for `distributed computing` with Rust so a heavy task can be split between multiple computers.

In order not to offend the terminology is chosen accordingly:
- parent - the computer which acts as a server and sends out commands
- children - the machines actually doing the work.

## Description
The application lets the user decide when to start the calculation. It goes as follows:
1. Starts a server and waits for connections
1. When the user inputs `:s` the whole computation starts and it is split evenly by default or configured as load percentage for each IP.
1. **TODO** find a way to configure the functions executed in each machine

## Configuration
There should be a configuration file in the root directory which serves as the config for:
- Work load of children
- Unallowed IPs(banned)
- The minimal count of computers which are needed to start

## Example
1. Something dear to my heart: Quantum computer simulations, where a single computer calculates the final state and all the other collapse it the given number of times. Then the results are sent back to the *master* and are aggregated to be shown on the screen.
1. Number crunching! Like analyzing the *min*, *max*, *count* etc. of given trait in a dataset.
1. A distributed database hidden behind a single endpoint and can be quarried. **TO BE THOUGHT ABOUT**

## Resources
1. https://docs.rs/serde_closure/0.3.2/serde_closure/
