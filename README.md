# DRand Verification in Smart Contracts

This project allows to verify BLS signatures onchain on Ethereum, using the power of Arbitrum Stylus

## Demo

The contract is deployed on Arbitrum Sepolia at 
0x7d0dA1D76929fdc256D0CF33829Ce38AfD14a1e7
and can be used like so
```sh
cast call \
         --rpc-url https://sepolia-rollup.arbitrum.io/rpc \
         0x7d0dA1D76929fdc256D0CF33829Ce38AfD14a1e7 \
         'verify(uint64 round_number, bytes calldata sig)' \
         9367912 \
         845091b99635a8a5f93a15cd8622fae0c7ee5307fb5b888dc67ee4fe997b5b326dbd637bd7ab39e3188fd507ea94b314
```

## Under the hood

Most of the code was adapted from [`drand-verify`](https://github.com/noislabs/drand-verify) and [`bls12-381`](https://github.com/zkcrypto/bls12_381).
It was also necessary to use the EVM sha2 precompile instead of the Rust implementation to keep code size reasonable.

## License

This project is fully open source under Apache-2.0.
This aligns with the license of third-party code.
