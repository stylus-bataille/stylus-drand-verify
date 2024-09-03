check:
  cargo stylus check

estimate-deploy-gas:
  cargo stylus deploy --private-key $PRIVATE_KEY --estimate-gas

deploy:
  cargo stylus deploy --private-key $PRIVATE_KEY

export-abi:
  cargo stylus export-abi > target/contract.sol
