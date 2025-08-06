# https://docs.uniswap.org/sdk/web3-react/guides/connectors

[Skip to main content](https://docs.uniswap.org/sdk/web3-react/guides/connectors#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Swaps](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Position Management](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Advanced](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
      * [Technical Reference](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
    * [v3 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
    * [Swap Widget](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
      * [Overview](https://docs.uniswap.org/sdk/web3-react/overview)
      * [Guides](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Connecting to Wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Supported Connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Switching Chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains)
    * [Core SDK](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
    * [v2 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
    * [v1 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * web3-react
  * Guides
  * [Supported Connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors)


On this page
# Supported Connectors
## Introduction[​](https://docs.uniswap.org/sdk/web3-react/guides/connectors#introduction "Direct link to Introduction")
This guide will cover how to connect our dApp to all the different connectors that `web3-react` supports. It is based on the [`web3-react` example](https://github.com/Uniswap/examples/tree/main/web3-react), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/blob/main/web3-react/README.md) and follow the setup instructions.
In this example we will cover connecting our dApp to the following connectors:
  * Coinbase wallet
  * WalletConnect wallet
  * Network
  * Gnosis safe


info
For help on setting up `web3-react` and interacting with a MetaMask wallet, please visit our [connecting to wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet) page!
The input parameters to this guide are the chains that we want our dApp to be able to connect to and their RPC URLs.
The guide will **cover** :
  1. Building a Coinbase Wallet connector
  2. Building a WalletConnect Wallet connector
  3. Building a Network connector
  4. Building a Gnosis Safe connector


At the end of the guide, we should be able to connect and disconnect the application to the different connectors listed above.
For this guide, the following `web3-react` packages are used:
  * [`@web3-react/core`](https://www.npmjs.com/package/@web3-react/core)
  * [`@web3-react/types`](https://www.npmjs.com/package/@web3-react/types)
  * [`@web3-react/coinbase-wallet`](https://www.npmjs.com/package/@web3-react/coinbase-wallet)
  * [`@web3-react/walletconnect`](https://www.npmjs.com/package/@web3-react/walletconnect)
  * [`@web3-react/network`](https://www.npmjs.com/package/@web3-react/network)
  * [`@web3-react/gnosis-safe`](https://www.npmjs.com/package/@web3-react/gnosis-safe)


info
This guide uses `web3-react` version 8, which is a beta version.
The core code of this guide can be found in the top level of our [examples repository](https://github.com/Uniswap/examples/tree/main/web3-react), under each connectors' name. For example, the code for the Coinbase Wallet connector can be found in the the [coinbase file](https://github.com/Uniswap/examples/blob/main/web3-react/src/libs/coinbase.ts).
## Building a Coinbase Wallet connector[​](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-coinbase-wallet-connector "Direct link to Building a Coinbase Wallet connector")
The second connector in the list of prioritized connectors that [we provided](https://docs.uniswap.org/sdk/web3-react/guides/01-connect-wallet.md/#building-an-injected-connector) as a parameter to [`Web3ReactProvider`](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/components/Web3ContextProvider.tsx) is the _Coinbase Wallet_ connector:
Creating the prioritized Connectors list
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L33-L39)
To connect to a _Coinbase Wallet_ connector, we first need to install [`@web3-react/coinbase-wallet`](https://www.npmjs.com/package/@web3-react/coinbase-wallet), as well as [`@coinbase/wallet-sdk`](https://github.com/coinbase/coinbase-wallet-sdk). Having installed the packages, we can import the `CoinbaseWallet` class from `@web3-react/coinbase-wallet`, as well as the `initializeConnector` function from the `@web3-react/core` package:
Importing the Coinbase Wallet connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/coinbase.ts#L1-L2)
We can now build our connector, supplying the required arguments:
Initializing the Coinbase Wallet Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/coinbase.ts#L8-L19)
We pass `CoinbaseWallet` as the type argument to `initializeConnector`'s templated parameter. Similar to the case of the `InjectedConnector`, the `CoinbaseWallet` class is a class that extends the `AbstractConnector` class, which is part of the `@web3-react/core` package. The parameter provided to `initializeConnector` is a function that receives an `actions` object, and expects an instance of `CoinbaseWallet` (to match the type argument) to be returned.
We build the new `CoinbaseWallet` instance by passing the `actions` object, an `options` object, and an `onError` callback. `onError` handles errors that occur during interaction with the connector, and `options` is used to configure the connector. In our case, we pass the `url`, `appName` and `reloadOnDisconnect` options: `url` is the _RPC URL_ to connect to that was provided as an argument to the example application, `appName` is the name of our application, and `reloadOnDisconnect` is a `boolean` that indicates whether the application should reload when the user disconnects from the wallet.
After building the connector, we use its two return types, the `Connector` and it's respective hooks, and build a `Connection` object by setting the connection's type as the Coinbase wallet:
Building the Coinbase Wallet Connection
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/coinbase.ts#L20-L24)
Having built the connector, all that remains is to build the user interface and supply it to our [`ConnectionOptions`](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/components/ConnectionOptions.tsx) component, just as we did with the `InjectedConnector`:
Building the Coinbase Wallet component
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/ConnectionOptions.tsx#L39-L46)
## Building a WalletConnect Wallet connector[​](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-walletconnect-wallet-connector "Direct link to Building a WalletConnect Wallet connector")
The third connector in the list of prioritized connectors that we provided to [`Web3ReactProvider`](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/components/Web3ContextProvider.tsx) is the WalletConnect Wallet connector.
To connect to a WalletConnect Wallet connector, we first need to install [`@web3-react/walletconnect`](https://www.npmjs.com/package/@web3-react/walletconnect), as well as [`@walletconnect/ethereum-provider`](https://www.npmjs.com/package/@walletconnect/ethereum-provider). Having installed the packages, we can import the `WalletConnect` class from `@web3-react/walletconnect`, as well as the `initializeConnector` function from`@web3-react/core` package:
Importing the WalletConnect Wallet Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/wallet-connect.ts#L1-L2)
We can now build our connector, supplying the required arguments:
Initializing the WalletConnect Wallet Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/wallet-connect.ts#L8-L17)
The main difference from the Coinbase Wallet connector lies in the arguments that the `WalletConnect` class requires to be instantiated. `web3-react` knows about this difference, as we passed the type argument `WalletConnect` to `initializeConnector`, thus specializing the type of `AbstractConnector`. In this case, the class receives three arguments, including `actions` and `onError`, identical to those supplied in the Coinbase Wallet connector case.
The difference lies in the second argument, which is an `options` object. In this case, we are passing the `rpc` parameter, which is an object that maps the chain ID to the RPC URL to connect to. We have already created this [map](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/constants.ts#L11) in our [`constants`](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/constants.ts) file using our example's parameters. The other option that we are passing is the `qrcode`, which is a `boolean` that indicates whether the QR code should be displayed in the browser. In our case, we are passing `true` as we want to show the QR code.
Having built the connector, we just need to build the user interface to enable user interaction with the connector, and supply it to our `ConnectionOptions`:
Building the WalletConnect Wallet component
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/ConnectionOptions.tsx#L49-L56)
## Building a Network connector[​](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-network-connector "Direct link to Building a Network connector")
The _Network connector_ , alongside the _Gnosis Safe connector_ , are two of the connectors that we do not surface through our user interface, but instead we connect to them programmatically. In contrast to the previous _Connectors_ , these do not come with any pre-built user interface for the user to interact with. We attempt to connect to them **eagerly** in our [`Web3ContextProvider`](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/components/Web3ContextProvider.tsx) component through a hook:
Hook to connect eagerly
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/Web3ContextProvider.tsx#L9-L13)
The `useEagerlyConnect` hook is called in the `Web3ContextProvider` component and attempts to connect to the Network Connector and the Gnosis Safe Connector. The hook is named **eagerly** as it is called in the component's body as [React effect](https://reactjs.org/docs/hooks-effect.html) when the component is first rendered. In the hook implementation we attempt to call `web3-react`'s `connectEagerly` function if it exists on the connector, otherwise we call `activate` otherwise. The `connectEagerly` function attempts to connect our application to the connector, and **fails silently** if it does not succeed:
Connecting eagerly
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/hooks.ts#L15-L19)
Before eagerly connecting, we first need to initialize the connectors. We start by building the Network connector, and we first need to install [`@web3-react/network`](https://www.npmjs.com/package/@web3-react/network), and import the `Network` class from it. Note how this Connect does not require any package besides its `web3-react` package to function. We also need to import the `initializeConnector` function from `@web3-react/core`:
Importing the Network Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/network.ts#L1-L2)
We can now build our connector, supplying the required arguments:
Initializing the Network Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/network.ts#L8-L15)
The main difference from the other connectors lies in the arguments that the `Network` class requires to be instantiated. `web3-react` knows about this difference, as we passed the type argument `Network` to `initializeConnector`, thus specializing the type of `AbstractConnector`. In this case, the class receives `actions`, which is identical to that supplied in the rest of the connectors; `urlMap`, which is an object that maps the chain ID to the RPC URL to connect to, which we have already created in our [`constants`](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/constants.ts) file; and `defaultChainId` which is the chain ID to connect to by default.
After building, the connector, we can create a `Connection` instance by supplying it the return value of the `initializeConnector` function, and the `Network` class:
Creating a Network connection
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/network.ts#L16-L20)
All that remains is to return the constructed `Connection` instance.
## Building a Gnosis Safe connector[​](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-gnosis-safe-connector "Direct link to Building a Gnosis Safe connector")
Similar to the Network connector, we build the Gnosis Safe connector. We start by first installing [`@web3-react/gnosis-safe`](https://www.npmjs.com/package/@web3-react/gnosis-safe), and import the `GnosisSafe` class from it. We also need to import the `initializeConnector` function from `@web3-react/core`:
Importing the Gnosis Safe connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/gnosis.tsx#L1-L2)
The Gnosis Safe connector is the simplest of them all, as it does not require any additional parameterization other than `actions`:
Initializing a Gnosis Safe Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/gnosis.tsx#L6-L9)
Having initialized the connector, we can now build the `Connection` instance and return it:
Creating a Gnosis Safe Connection
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/gnosis.tsx#L10-L14)
## Next steps[​](https://docs.uniswap.org/sdk/web3-react/guides/connectors#next-steps "Direct link to Next steps")
Now that we have gone through building all of the different types of supported connectors, we will learn how to [switch chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/web3-react/guides/02-connectors.md)
Was this helpful?
[PreviousConnecting to Wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)[NextSwitching Chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/web3-react/guides/connectors#introduction)
  * [Building a Coinbase Wallet connector](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-coinbase-wallet-connector)
  * [Building a WalletConnect Wallet connector](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-walletconnect-wallet-connector)
  * [Building a Network connector](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-network-connector)
  * [Building a Gnosis Safe connector](https://docs.uniswap.org/sdk/web3-react/guides/connectors#building-a-gnosis-safe-connector)
  * [Next steps](https://docs.uniswap.org/sdk/web3-react/guides/connectors#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/web3-react/guides/02-connectors.md)
## Footer
[Uniswap Labs](https://docs.uniswap.org/)
### Developers
  * [Dev Chat](https://discord.com/invite/uniswap)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)
  * [Whitepaper](https://app.uniswap.org/whitepaper-v4.pdf)


### Ecosystem
  * [Uniswap App](https://app.uniswap.org/)
  * [Governance](https://www.uniswapfoundation.org/governance)
  * [Blog](https://blog.uniswap.org/)


### Company
  * [Careers](https://boards.greenhouse.io/uniswaplabs)
  * [Brand Assets](https://github.com/Uniswap/brand-assets/raw/main/Uniswap%20Brand%20Assets.zip)
  * [Terms of Service](https://support.uniswap.org/hc/en-us/articles/30935100859661-Uniswap-Labs-Terms-of-Service)
  * [Privacy Policy](https://support.uniswap.org/hc/en-us/articles/30934457771405-Uniswap-Labs-Privacy-Policy)
  * [Trademark Policy](https://support.uniswap.org/hc/en-us/articles/30934762216973-Uniswap-Labs-Trademark-Guidelines)


### Need Help?
  * [Help Center](https://support.uniswap.org/)
  * [Contact Us](https://support.uniswap.org/hc/en-us/requests/new)


@2025 Uniswap Labs
[](https://github.com/uniswap/uniswap-docs)[](https://twitter.com/Uniswap)[](https://discord.com/invite/uniswap)
