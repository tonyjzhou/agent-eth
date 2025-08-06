# https://docs.uniswap.org/sdk/web3-react/guides/switch-chains

[Skip to main content](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/overview)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Swaps](https://docs.uniswap.org/sdk/v4/guides/swaps/quoting)
        * [Position Management](https://docs.uniswap.org/sdk/v4/guides/liquidity/position-minting)
        * [Advanced](https://docs.uniswap.org/sdk/v4/guides/advanced/pool-data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/overview)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/overview)
    * [Swap Widget](https://docs.uniswap.org/sdk/swap-widget/overview)
    * [web3-react](https://docs.uniswap.org/sdk/web3-react/overview)
      * [Overview](https://docs.uniswap.org/sdk/web3-react/overview)
      * [Guides](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Connecting to Wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet)
        * [Supported Connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors)
        * [Switching Chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains)
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * web3-react
  * Guides
  * [Switching Chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains)


On this page
# Switching Chains
## Introduction[​](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#introduction "Direct link to Introduction")
This guide will cover how to prompt a wallet that has connected to our dApp to switch chains using `web3-react`. It is based on the [`web3-react` example](https://github.com/Uniswap/examples/tree/main/web3-react), found in the Uniswap code examples [repository](https://github.com/Uniswap/examples). To run this example, check out the examples's [README](https://github.com/Uniswap/examples/blob/main/web3-react/README.md) and follow the setup instructions.
info
For help on setting up `web3-react` and interacting with a MetaMask wallet, please visit our [connecting to wallets](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet) page!
The input parameters to this guide are the chains that we want our dApp to be able to connect to and their RPC URLs.
At the end of the guide, we should be able to switch chains on the connected wallet.
For this guide, the following `web3-react` packages are used:
  * [`@web3-react/core`](https://www.npmjs.com/package/@web3-react/core)


info
This guide uses `web3-react` version 8, which is a beta version.
The core code of this guide can be found in [connections](https://github.com/Uniswap/examples/blob/feat/web3-react/web3-react/src/libs/connections.ts).
## Switching Chains[​](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#switching-chains "Direct link to Switching Chains")
Having [setup our application](https://docs.uniswap.org/sdk/web3-react/guides/connect-wallet) to use `web3-react` and having built out the ability to [connect and disconnect wallets](https://docs.uniswap.org/sdk/web3-react/guides/connectors), we can now move on to switching chains.
Switching chains requires two parameters, the `chainId` we want to switch to, and the current `connectionType`:
Defining the function
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L64)
Given the `ConnectionType`, we can retrieve the actual connector:
Retrieving the connector
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L69)
Then, depending on the `ConnectionType`, we determine how to switch chains. For the `Network` or `WalletConnect` cases, we call `web3-react`'s `activate` function with the supplied `chainId`:
Switching chains for Network and WalletConnect
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L71-L74)
The rest of the connectors require us to build an `AddEthereumChainParameter` object and pass it to the `web3-react`'s `activate` function:
Switching chains the other Connectors
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/connections.ts#L77-L84)
The metadata required to build `AddEthereumChainParameter` are defined in our constants file:
Defining the chain parameters
```
loading...
```

[View on GitHub](https://github.com/Uniswap/examples/blob/8c0e36ca8d2ba4718af944094191f39da62a9c5c/web3-react/src/libs/constants.ts#L27-L40)
## Next steps[​](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#next-steps "Direct link to Next steps")
Know you know how to support `web3-react`'s most common use cases! Stay tuned for follow up guides.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/web3-react/guides/03-switch-chains.md)
Was this helpful?
[PreviousSupported Connectors](https://docs.uniswap.org/sdk/web3-react/guides/connectors)[NextOverview](https://docs.uniswap.org/sdk/core/overview)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#introduction)
  * [Switching Chains](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#switching-chains)
  * [Next steps](https://docs.uniswap.org/sdk/web3-react/guides/switch-chains#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/web3-react/guides/03-switch-chains.md)
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
