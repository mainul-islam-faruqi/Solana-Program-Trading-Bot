# On-Chain Trading Bot Program

A Solana program for automated DCA (Dollar Cost Averaging) trading using Jupiter's DCA program.

## Overview

This program provides functionality for:
- Setting up DCA (Dollar Cost Averaging) strategies
- Managing token swaps through Jupiter
- Handling token escrow accounts
- Airdrop distribution for participants

## Program Architecture

### Core Components

1. **Instructions**
   - `setup_dca`: Create new DCA strategy
   - `close`: Close DCA positions and accounts
   - `airdrop`: Handle airdrop distribution

2. **State Management**
   - `Escrow`: Manages user tokens and DCA state
   - Handles token accounts and permissions

3. **Integration**
   - Jupiter DCA program integration
   - Token account management
   - Secure fund handling

## Development

### Prerequisites
- Rust and Cargo
- Solana Tool Suite
- Anchor Framework

### Build
```bash
anchor build
```

### Test
```bash
anchor test
```

### Deploy
```bash
anchor deploy
```

Program Id: 7Jg27CTDL38Pq3ErvTbxwc3FSjz9oqZN5GMXUAjnt9Dz

Signature: 3L9MkyLhfUbrkj7wF7g24StMFcawMA8TftRG3ejRAPGQfxpVZSdKZATn83kViQh2kGDhydSFwYgk2pzLtjeDw8D9


## Program Structure

```
src/
├── lib.rs              # Program entry point and instruction handlers
├── instructions/       # Instruction implementation
│   ├── setup_dca.rs   # DCA setup logic
│   ├── close.rs       # Position closing logic
│   └── airdrop.rs     # Airdrop distribution
├── state/             # Program state definitions
│   └── escrow.rs      # Escrow account structure
└── errors/            # Custom error definitions
```

## Usage

1. Initialize DCA Strategy:
```typescript
await program.methods
  .setupDca(
    applicationIdx,
    inAmount,
    inAmountPerCycle,
    cycleFrequency,
    minOutAmount,
    maxOutAmount,
    startAt
  )
  .accounts({...})
  .rpc();
```

2. Close Position:
```typescript
await program.methods
  .close()
  .accounts({...})
  .rpc();
```

3. Handle Airdrop:
```typescript
await program.methods
  .airdrop()
  .accounts({...})
  .rpc();
```

## Security Considerations

- All token transfers use PDAs
- Input validation on amounts and intervals
- Proper permission checks
- Secure escrow account management

## References

- [Jupiter DCA Documentation](https://docs.jup.ag/jupiter-dca/introduction)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Program Security](https://docs.solana.com/developing/programming-model/security)

## License

MIT
