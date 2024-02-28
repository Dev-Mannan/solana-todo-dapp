import Head from 'next/head'
import { WalletConnectProvider } from '../components/WalletConnectProvider'
import '../styles/global.css'
import '@solana/wallet-adapter-react-ui/styles.css'
import { Header } from '../components/Header'

function MyApp({ Component, pageProps }) {
    return (
        <>
            <Head>
                <title>Todo App</title>
            </Head>
            <main>
                <Header/>
                <WalletConnectProvider>
                    <Component {...pageProps} />
                </WalletConnectProvider>
            </main>
        </>
    )
}

export default MyApp