import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TradingBot } from '../target/types/trading_bot';

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TradingBot as Program<TradingBot>;

  // Deploy program
  const [programId] = await anchor.web3.PublicKey.findProgramAddress(
    [Buffer.from('trading-bot')],
    program.programId
  );

  console.log('Program deployed at:', programId.toString());
}

main().catch(console.error);
