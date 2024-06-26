import React, { useMemo, useEffect, useState, useCallback } from 'react';
import ReactDOM from 'react-dom/client';
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from '@solana/wallet-adapter-react';
import {
  WalletModalProvider,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddress } from "@solana/spl-token";
import * as buffer from "buffer";
window.Buffer = buffer.Buffer;

// Default styles that can be overridden by your app
require('@solana/wallet-adapter-react-ui/styles.css');

const ORE_V2 = new PublicKey("HS9XYYijv7g39DJ8G7zWB4Sb5ewRvWyeJ4JyMR2V1YYi");

export const Wallet = () => {
  // You can also provide a custom RPC endpoint.
  const endpoint = "https://devnet.helius-rpc.com/?api-key=1de92644-323b-4900-9041-13c02730955c"

  const wallets = useMemo(
    () => [
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <WalletMultiButton />
          { /* Your app's components go here, nested within the context providers. */}
          <RenderOREv2Balance />
          <LogMessi />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
};

function oreWalletAdapter() {
  const container = document.getElementById('ore-wallet-adapter-id');
  const root = ReactDOM.createRoot(container);
  root.render(<Wallet />);
}

function RenderOREv2Balance() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState(0);
  useEffect(() => {
    async function fetchBalance() {
      if (publicKey) {
        if (connection) {
          const tokenAddress = await getAssociatedTokenAddress(
            ORE_V2,
            publicKey
          );
          try {
            const tokenAccount = await connection.getTokenAccountBalance(
              tokenAddress
            );
            console.log("token account");
            console.log(tokenAccount);
            setBalance(tokenAccount.value.uiAmountString);
          } catch (err) {
            setBalance(0);
            console.log(err);
          }
        }
      }
    }
    fetchBalance()
    // cleanup callback (void)
    return () => {
      console.log("cleaning up");
      setBalance(0);
    };
  }, [publicKey, connection]);

  return (
    <div>{balance}</div>
  )
}

function LogMessi() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const callback = useCallback(async (e) => {
    // 890880 lamports as of 2022-09-01
    const lamports = await connection.getMinimumBalanceForRentExemption(0);
    console.log(lamports);
    console.log(e);
    console.log(e.detail)

  }, [publicKey, connection]);
  window.MessiLogger = callback;
  return
}

//function SendSOLToRandomAddress() {
//  const { connection } = useConnection();
//  const { publicKey, sendTransaction } = useWallet();
//  console.log(publicKey);
//  const onClickk = useCallback(async () => {
//    // 890880 lamports as of 2022-09-01
//    const lamports = await connection.getMinimumBalanceForRentExemption(0);
//    console.log(lamports);
//
//    const transaction = new Transaction().add(
//      SystemProgram.transfer({
//        fromPubkey: publicKey,
//        toPubkey: Keypair.generate().publicKey,
//        lamports,
//      })
//    );
//
//    const {
//      context: { slot: minContextSlot },
//      value: { blockhash, lastValidBlockHeight }
//    } = await connection.getLatestBlockhashAndContext();
//
//    const signature = await sendTransaction(transaction, connection, { minContextSlot });
//
//    await connection.confirmTransaction({ blockhash, lastValidBlockHeight, signature });
//  }, [publicKey, sendTransaction, connection]);
//  window.addEventListener("ore-go", onClickk);
//  return
//}

window.OreWalletAdapter = oreWalletAdapter;
