import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import "./App.css";
import {
  http,
  createConfig,
  WagmiProvider,
  useConnect,
  useAccount,
  useDisconnect,
  useEnsName,
  useEnsAvatar,
  useSendTransaction,
} from "wagmi";
import { mainnet } from "wagmi/chains";
import { injected, metaMask } from "wagmi/connectors";

export const config = createConfig({
  chains: [mainnet],
  connectors: [injected(), metaMask()],
  transports: {
    [mainnet.id]: http(),
    // [base.id]: http(),
  },
});

const queryClient = new QueryClient();

function App() {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <WalletConnect />
        <EthSend />
        <Myaddress />
      </QueryClientProvider>
    </WagmiProvider>
  );
}

function Myaddress() {
  const { address } = useAccount();
  const { disconnect } = useDisconnect();
  return (
    <div>
      {address}
      <br />
      <button onClick={() => disconnect()}>disconnect</button>
    </div>
  );
}

function WalletConnect() {
  const { connect, connectors } = useConnect();

  // console.log("Available connectors:", connectors);

  return connectors.map((connector) => (
    <button
      key={connector.uid}
      onClick={() => connect({ connector })}>
      {connector.name}
    </button>
  ));
}

function EthSend() {
  const { sendTransaction, data: hash } = useSendTransaction();

  function handleSend() {
    sendTransaction({
      to: document.getElementById("recipientAddress").value,
      value: BigInt(0.01 * 10 ** 18), // Sending 0.01 ETH
    });
  }

  return (
    <div>
      <input
        id='recipientAddress'
        type='text'
        placeholder='Enter recipient address'
      />
      <button onClick={handleSend}>Send ETH</button>
      {hash}
    </div>
  );
}

export default App;
