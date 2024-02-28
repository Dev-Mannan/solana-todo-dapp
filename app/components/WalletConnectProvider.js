
import {WalletAdapterNetwork} from "@solana/wallet-adapter-base"
import {ConnectionProvider , WalletProvider} from "@solana/wallet-adapter-react"
import {WalletModalProvider} from "@solana/wallet-adapter-react-ui"
import {GlowWalletAdapter, LedgerWalletAdapter, PhantomWalletAdapter, SlopeWalletAdapter, SolflareWalletAdapter, TorusWalletAdapter} from "@solana/wallet-adapter-wallets"
import {clusterApiUrl} from "@solana/web3.js"
import {useMemo} from "react";
import { Children } from "react"

export const WalletConnectProvider = ({ children, ...props }) => {
    const network = WalletAdapterNetwork.Devnet
  
    const endpoint = useMemo(() => {
      if (network === WalletAdapterNetwork.Devnet) {
        return 'https://few-summer-wildflower.solana-devnet.quiknode.pro/29761b618c8a598ca7edb0172edc308daa29787e/'
      }
      return clusterApiUrl(network)
    }, [network])
  
    const wallets = useMemo(() => [new PhantomWalletAdapter()], [network])
  
    return (
      <ConnectionProvider endpoint={endpoint} {...props}>
        <WalletProvider wallets={wallets} autoConnect={true} {...props}>
          <WalletModalProvider>{children}</WalletModalProvider>
        </WalletProvider>
      </ConnectionProvider>
    )
  }