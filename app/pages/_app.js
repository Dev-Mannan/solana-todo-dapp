import Head from 'next/head'
import { WalletConnectProvider } from '../components/WalletConnectProvider'
import '../styles/global.css'
import '@solana/wallet-adapter-react-ui/styles.css'

function MyApp({ Component, pageProps }) {
    return (
        <>
            <Head>
                <title>Todo App</title>
            </Head>
            <main>
                <div ><h1>Todo App</h1></div>
                <WalletConnectProvider>
                    <Component {...pageProps} />
                </WalletConnectProvider>
            </main>
        </>
    )
}

export default MyApp