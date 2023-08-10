### Used Primitives

The implementation uses the following t3rn's primitives and types:

- Action, SideEffect, TargetId 

### Other Pallets Integration

The pallet functionality could be utilised by other pallets through adding ExecutionSequenceTrait to their Configs

### Testing the implementation

To run the implementation enter the following command: 
```
SKIP_WASM_BUILD= cargo test -p pallet-executor-sequence
```
### Variants

Implementation using generic substrate node template and Polkadot version 1.0 at **take-at-home-task-prototype** branch

Implementation modified t3rn blokchain + primitives and Polkadot version 0.9.27 at **take-at-home-task** branch