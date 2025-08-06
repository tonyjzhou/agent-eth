# https://docs.uniswap.org/sdk/v1/reference/constants

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/constants#__docusaurus_skipToContent_fallback)
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
    * [Core SDK](https://docs.uniswap.org/sdk/core/overview)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/overview)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/guides/getting-started)
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
  * [Constants](https://docs.uniswap.org/sdk/v1/reference/constants)


# Constants
Below is an exhaustive list of all external constants used in the SDK.
```
import BigNumber from'bignumber.js'importERC20from'./abis/ERC20.json'importFACTORYfrom'./abis/FACTORY.json'importEXCHANGEfrom'./abis/EXCHANGE.json'exportconstETH='ETH'exportenumSUPPORTED_CHAIN_ID{ Mainnet =1, Ropsten =3, Rinkeby =4, Kovan =42,}exportconstFACTORY_ADDRESS:{[key:number]:string}={}exportconstFACTORY_ABI:string=JSON.stringify(FACTORY)exportconstEXCHANGE_ABI:string=JSON.stringify(EXCHANGE)exportenumTRADE_TYPE{ETH_TO_TOKEN='ETH_TO_TOKEN',TOKEN_TO_ETH='TOKEN_TO_ETH',TOKEN_TO_TOKEN='TOKEN_TO_TOKEN',}exportenumTRADE_EXACT{INPUT='INPUT',OUTPUT='OUTPUT',}exportenumTRADE_METHODS{ ethToTokenSwapInput ='ethToTokenSwapInput', ethToTokenTransferInput ='ethToTokenTransferInput', ethToTokenSwapOutput ='ethToTokenSwapOutput', ethToTokenTransferOutput ='ethToTokenTransferOutput', tokenToEthSwapInput ='tokenToEthSwapInput', tokenToEthTransferInput ='tokenToEthTransferInput', tokenToEthSwapOutput ='tokenToEthSwapOutput', tokenToEthTransferOutput ='tokenToEthTransferOutput', tokenToTokenSwapInput ='tokenToTokenSwapInput', tokenToTokenTransferInput ='tokenToTokenTransferInput', tokenToTokenSwapOutput ='tokenToTokenSwapOutput', tokenToTokenTransferOutput ='tokenToTokenTransferOutput',}exportconstTRADE_METHOD_IDS:{[key:string]:string}={}exportenumFIXED_UNDERFLOW_BEHAVIOR{ZERO='ZERO',LESS_THAN='LESS_THAN',ONE_DIGIT='ONE_DIGIT',}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/07-constants.md)
Was this helpful?
[PreviousTransact](https://docs.uniswap.org/sdk/v1/reference/transact)[NextTypes](https://docs.uniswap.org/sdk/v1/reference/types)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/07-constants.md)
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
