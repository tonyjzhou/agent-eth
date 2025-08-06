# https://docs.uniswap.org/sdk/v1/reference/data

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/data#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v1/reference/data)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Swaps](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Position Management](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Advanced](https://docs.uniswap.org/sdk/v1/reference/data)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/data)
    * [v3 SDK](https://docs.uniswap.org/sdk/v1/reference/data)
    * [Swap Widget](https://docs.uniswap.org/sdk/v1/reference/data)
    * [web3-react](https://docs.uniswap.org/sdk/v1/reference/data)
    * [Core SDK](https://docs.uniswap.org/sdk/v1/reference/data)
    * [v2 SDK](https://docs.uniswap.org/sdk/v1/reference/data)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/reference/data)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Data](https://docs.uniswap.org/sdk/v1/reference/data)
        * [Computation](https://docs.uniswap.org/sdk/v1/reference/computation)
        * [Format](https://docs.uniswap.org/sdk/v1/reference/format)
        * [Orchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Transact](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Constants](https://docs.uniswap.org/sdk/v1/reference/constants)
        * [Types](https://docs.uniswap.org/sdk/v1/reference/types)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v1 SDK
  * Technical Reference
  * [Data](https://docs.uniswap.org/sdk/v1/reference/data)


On this page
# getTokenReserves
This function fetches Uniswap reserve data for a given token address on a given network.
  * If only a chain id is specified, the Ethereum node used to fulfill data requests is determined by [`ethers.getDefaultProvider`](https://docs.ethers.io/ethers.js/html/api-providers.html#connecting-to-ethereum), else it is the one specified by the passed provider.
  * This function throws an error if the provided tokenAddress is not a token with a Uniswap exchange.


## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/data#function-signature "Direct link to Function Signature")
```
exportasyncfunctiongetTokenReserves( tokenAddress:string, chainIdOrProvider: ChainIdOrProvider =1):Promise<TokenReservesNormalized>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/data#input-parameters "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddress| `string`| The checksummed address of a token with a Uniswap exchange.  
chainIdOrProvider| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/data#example-usage "Direct link to Example Usage")
```
const tokenAddress ='0x89d24A6b4CcB1B6fAA2625fE562bDD9a23260359'// DAI Mainnetconst chainIdOrProvider: ChainIdOrProvider =1// could be e.g. window.ethereum insteadconst tokenReserves: TokenReservesNormalized =awaitgetTokenReserves(tokenAddress, chainIdOrProvider)/*{ // details for the passed token token: {  chainId: 1,  address: '0x89d24A6b4CcB1B6fAA2625fE562bDD9a23260359',  decimals: 18 }, // details for the Uniswap exchange of the passed token exchange: {  chainId: 1,  address: '0x09cabEC1eAd1c0Ba254B09efb3EE13841712bE14',  decimals: 18 }, // details for the ETH portion of the reserves of the passed token ethReserve: {  token: {   chainId: 1,   address: 'ETH',   decimals: 18  },  amount: <BigNumber> }, // details for the token portion of the reserves of the passed token tokenReserve: {  token: {   chainId: 1,   address: '0x89d24A6b4CcB1B6fAA2625fE562bDD9a23260359',   decimals: 18  },  amount: <BigNumber> }}*/
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/02-data.md)
Was this helpful?
[PreviousGetting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)[NextComputation](https://docs.uniswap.org/sdk/v1/reference/computation)
On this page
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/data#function-signature)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/data#input-parameters)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/data#example-usage)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/02-data.md)
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
