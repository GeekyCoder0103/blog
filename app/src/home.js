import { WalletName } from "@solana/wallet-adapter-wallets";
import { useWallet } from "@solana/wallet-adapter-react";


const Home = () => {
    const { select } = useWallet();
    const onConnect = () => {
        select(WalletName.Phantom);
    }
    return (
        <>
        <button onClick={onConnect}>
            Select
        </button>
        </>
    )
}

export default Home;