import * as anchor from "@safely-project/anchor";
import { Connection } from "@safecoin/web3.js";
export declare const createTestTransferTransaction: (authority: anchor.web3.PublicKey, recipient: anchor.web3.PublicKey, amount?: number) => Promise<anchor.web3.TransactionInstruction>;
export declare const createBlankTransaction: (connection: Connection, feePayer: anchor.web3.PublicKey) => Promise<anchor.web3.Transaction>;
