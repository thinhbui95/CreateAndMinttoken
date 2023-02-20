const anchor = require('@project-serum/anchor');
import { Program , BN} from '@project-serum/anchor';
import { Vd1} from '../target/types/vd1';
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,

} from "@solana/spl-token";
import {SolanaConfigService} from "@coin98/solana-support-library/config"
const { SystemProgram } = anchor.web3;

describe('initmint' , () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Vd1;
  let wallet = anchor.web3.Keypair.generate()
  const xMint = anchor.web3.Keypair.generate()

  before(async () => {
    wallet = await SolanaConfigService.getDefaultAccount()
    console.log(wallet.publicKey.toString())
  })

  it("Is initialized!" , async () => {
    const tx = await program.rpc.initMint({
      accounts: {
        user: wallet.publicKey,
        xMint: xMint.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [wallet, xMint],
    });
    console.log("Transaction" ,tx)
  });

  it("Mint Token x", async () => {
    let associatedTokenAccount = await getAssociatedTokenAddress(
      xMint.publicKey,
      wallet.publicKey,
  );
    const tx = await program.rpc.mintTokenX(new BN(1000), {
      accounts: {
        user: wallet.publicKey,
        mintAccount: xMint.publicKey,
        tokenX: associatedTokenAccount,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,

      },
      signers: [wallet],
    });
    console.log(tx);
  });

  
});