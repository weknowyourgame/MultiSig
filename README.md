# Multi-sig

1. Owners - the set of addresses that sign transactions for the multisig.
2. Threshold - the number of signers required to execute a transaction.

Once the `Multisig` account is created, one can create a `Transaction`
account, specifying the parameters for a normal solana transaction.

To sign, owners should invoke the `approve` instruction, and finally,
the `execute_transaction`, once enough (i.e. `threshold`) of the owners have
signed.

### Build

```bash
anchor build
```

### Deploy

#### Start test validator
```bash
solana-test-validator
```

#### Deploy on devnet
```bash
anchor deploy
```
