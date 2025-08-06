# https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet

[Skip to main content](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Swaps](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Position Management](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Advanced](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
      * [Technical Reference](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
    * [v3 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
    * [Swap Widget](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
      * [Overview](https://docs.uniswap.org/sdk/web3-react/overview)
      * [Guides](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Connecting to Wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Supported Connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Switching Chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains)
    * [Core SDK](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
    * [v2 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
    * [v1 SDK](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * web3-react
  * Guides
  * [Connecting to Wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)


On this page
# Connecting to Wallets
## Introduction[​](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#introduction "Direct link to Introduction")
This guide will cover how to connect wallets with `web3-react`. It is based on the [`web3-react` example](https://github.com/Uniswap/examples/tree/main/web3-react) found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/blob/main/web3-react/README.md) and follow the setup instructions.
In this example we will walk through setting up `web3-react` and connecting the most popular browser-injected connector, [MetaMask](https://metamask.io/), using [@web3-react/metamask](https://www.npmjs.com/package/@web3-react/metamask).
The input parameters to this guide are the chains that we want our app to be able to connect to and their RPC URLs.
The guide will **cover** :
  1. Creating a `web3-react` `Web3ReactProvider`
  2. Building a `web3-react` `InjectedConnector`
  3. Connecting and disconnecting the application to the connector


At the end of the guide, we should be able to connect and disconnect your dApp to a MetaMask connector.
For this guide, the following `web3-react` packages are used:
  * [`@web3-react/core`](https://www.npmjs.com/package/@web3-react/core)
  * [`@web3-react/types`](https://www.npmjs.com/package/@web3-react/types)
  * [`@web3-react/metamask`](https://www.npmjs.com/package/@web3-react/metamask)


info
This guide uses `web3-react` version 8, which is a beta version.
These will be automatically installed by following the example's [README](https://github.com/Uniswap/examples/blob/main/web3-react/README.md).
The core code of this guide can be found in [Web3ContextProvider](https://github.com/Uniswap/examples/blob/main/web3-react/src/libs/components/Web3ContextProvider.tsx) and [InjectedConnector](https://github.com/Uniswap/examples/blob/main/web3-react/src/libs/injected.ts).
## Creating a `Web3ReactProvider`[​](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#creating-a-web3reactprovider "Direct link to creating-a-web3reactprovider")
To be able to interact with the methods that `web3-react` offers, we first need to setup a `Web3ReactProvider` and wrap our application in it. `web3-react` uses a [React `Context`](https://reactjs.org/docs/context.html) to allow us to use the exposed hooks without additional configuration.
To start, we create a React component called `Web3ContextProvider` in order to wrap the logic of configuring the `Web3ReactProvider`. In this component, we first import `Web3ReactProvider` from [@web3-react/core](https://www.npmjs.com/package/@web3-react/core).
The component receives just one prop which is the `children` to which it will be providing the `Context`:
Defining the Web3React component
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/Web3ContextProvider.tsx#L24)
We then implement the component by rendering the imported `Web3ReactProvider` with the `children` within that:
Implementing the component
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/Web3ContextProvider.tsx#L30-L34)
Note that we map our list of [`Connections`](https://github.com/Uniswap/examples/blob/856dbb002e7f38120554ef226f4309c96ce6ea79/web3-react/src/libs/connections.ts#L10) to a _tuple_ of the `connector` and `hooks` of the connection. The third element of a `Connection` refers to the [type](https://github.com/Uniswap/examples/blob/06980acc8f6d484b719d2c60f5bfe9d766cb95d6/web3-react/src/libs/connections.ts#L16) of `Connection` being established, which we will later use to keep track of the actively connected wallet.
Finally, having created the `Web3ContextProvider` component, we can navigate to our [index file](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/index.tsx) and wrap the whole application with it:
Wrapping our app with the web3 context
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/7ac3853bc465aecc428a32be584bbeb833b0a63c/web3-react/src/index.tsx#L16-L22)
## Building an Injected Connector[​](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#building-an-injected-connector "Direct link to Building an Injected Connector")
The only parameter that we provided to the `Web3ReactProvider` component is a list of prioritized connectors, declared as `PRIORITIZED_CONNECTORS`. The prioritization ordering is with regards to which connector we want to be active when more than one connector is connected to our application. The list is defined inside our connectors module:
Creating the prioritized Connectors list
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L33-L39)
Each one of those connectors lives within its own file, and they all follow a similar setup pattern.
An example of a connector in the list is the `InjectedConnector`, which supports wallets that inject an _Ethereum Provider_ into the browser window. The most popular example of an injected connector is the _MetaMask_ browser extension. To set it up, we import `initializeConnector` function from [core](https://www.npmjs.com/package/@web3-react/core) and the `MetaMask` type from [metamask](https://www.npmjs.com/package/@web3-react/core):
Importing Connector dependencies
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/856dbb002e7f38120554ef226f4309c96ce6ea79/web3-react/src/libs/injected.ts#L1-L2)
We then utilize the templated `initializeConnector` function with `MetaMask` as the type argument:
Initializing the MetaMask connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/injected.ts#L7-L9)
By passing in `MetaMask` as the type argument, we define the function's required input parameters. In this case, the only parameter we need to pass is an instance of `Metamask`, which receives the `actions` and `onError` parameters. The first parameter defines the actions that `web3-react` performs on its local store for the connector (this usually can be passed through without modification), while the second parameter is the callback invoked when an error occurs.
The return type of the function is a tuple of the initialized `Connector` and the `Hooks` that we can use on it. Using this tuple, we create an instance of a [`Connection`](https://github.com/Uniswap/examples/blob/856dbb002e7f38120554ef226f4309c96ce6ea79/web3-react/src/libs/connections.ts#L10) type, by setting the `type` property to `INJECTED`:
Creating a connection instance
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/856dbb002e7f38120554ef226f4309c96ce6ea79/web3-react/src/libs/injected.ts#L16-L20)
Finally, we return the instance of `Connection` we created, which is added to the list of prioritized connectors.
info
For help on creating the rest of the supported connectors of this examples, please visit our [connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors.md) page!
## Connecting and disconnecting the application to the connector[​](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#connecting-and--disconnecting-the-application-to-the-connector "Direct link to Connecting and disconnecting the application to the connector")
Having built our `InjectedConnector`, we can now use it in the Context that allows our application to use that connector:
Creating the Option component
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/Option.tsx#L5-L11)
The component receives 5 parameters:
  * `isEnabled` determines whether the connector is eligible to be activated
  * `isConnected` determines whether the connector is currently active
  * `connectionType` determines the `ConnectionType`
  * `onActivate` is called once the component has established a connection
  * `onDeactivate` is called once the component has disconnected


In the case of _MetaMask_ , when declaring the `InjectedConnector` we pass the connector-specific arguments:
Creating an injected connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/ConnectionOptions.tsx#L26-L32)
Then, in the `html` portion of the `Option`, we can figure out whether we want the current `Option`'s action button to be disabled, and whether clicking the button would result in the connector being connected or disconnected:
The component user interface
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/components/Option.tsx#L38-L42)
Finally, we also have enough information to figure out what action to take when the button is clicked. In the case that the click triggers a connection:
On connecting to a Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/components/Option.tsx#L38-L42)
To connect our wallet, all we need to do is to call the `tryActivateConnector` function and pass it the `InjectedConnector`. We then call the `onActivate` callback, which makes the `InjectedConnector` the active connector in our application's state.
`tryActivateConnector` takes as its argument the connector that we want to activate, and attempts to call `activate` on it. If this activation succeeds, it returns the new `ConnectionType`:
The implementation of tryActivateConnector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L90-L92)
In the case that the click triggers a disconnection:
On disconnecting from a Connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/components/Option.tsx#L29-L36)
To disconnect, all we need to do is to call `tryDeactivateConnector` and pass in it the `InjectedConnector` we created before. We then call the `onDeactivate` callback, which removes the `InjectedConnector` as the currently active connector from our application's state.
`tryDeactivateConnector` takes as its argument the connector that we want to deactivate, and attempts to call `deactivate` on it. If this deactivation succeeds, it resets the connector's state by calling `resetState` and returns `null`:
The implementation of tryDeactivateConnector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/81ec93e97b0afded621e177fe5f34fc9f98f80b0/web3-react/src/libs/connections.ts#L98-L100)
## Next Steps[​](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#next-steps "Direct link to Next Steps")
Now that we have gone through connecting and disconnecting from an `InjectedConnector`, we will learn how to [connect and disconnect](https://docs.uniswap.org/sdk/web3-react/guides/connectors) from all the different types of connectors that `web3-react` supports.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/web3-react/guides/01-connect-wallet.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/sdk/web3-react/overview)[NextSupported Connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#introduction)
  * [Creating a `Web3ReactProvider`](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#creating-a-web3reactprovider)
  * [Building an Injected Connector](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#building-an-injected-connector)
  * [Connecting and disconnecting the application to the connector](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#connecting-and--disconnecting-the-application-to-the-connector)
  * [Next Steps](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/web3-react/guides/01-connect-wallet.md)
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
