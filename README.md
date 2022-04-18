# Uniswap Price Listener

Rust program to list all pairs generated from Uniswap Factory v2 and be able to listen to prices (calculated from change in reserves).

This code will get the information about the pairs created from uniswap factory contract (0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f).
For simplicity sake, it will check on ehtereum mainnet through a free account with infura.

We will have for each pair:
- Token Symbol
- Decimals
- Address
- Price (calculated from reserves, ie, ratio between tokens of the pair)


Finally there is a simple website that uses socket.io that when paired with the backend process running, it broadcasts prices updates for the keypairs.


JSON ABI's copied from etherscan.
Factory ABI: https://etherscan.io/address/0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f#code
Pair ABI: https://etherscan.io/address/0x3139Ffc91B99aa94DA8A2dc13f1fC36F9BDc98eE#code