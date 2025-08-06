# https://docs.uniswap.org/sdk/v1/reference/transact

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/transact#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v1/reference/transact)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Swaps](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Position Management](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Advanced](https://docs.uniswap.org/sdk/v1/reference/transact)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/transact)
    * [v3 SDK](https://docs.uniswap.org/sdk/v1/reference/transact)
    * [Swap Widget](https://docs.uniswap.org/sdk/v1/reference/transact)
    * [web3-react](https://docs.uniswap.org/sdk/v1/reference/transact)
    * [Core SDK](https://docs.uniswap.org/sdk/v1/reference/transact)
    * [v2 SDK](https://docs.uniswap.org/sdk/v1/reference/transact)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/reference/transact)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/reference/transact)
        * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/transact)
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
  * [Transact](https://docs.uniswap.org/sdk/v1/reference/transact)


On this page
# getExecutionDetails
The function formats trade data for execution against the relevant Uniswap exchange.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/transact#function-signature "Direct link to Function Signature")
```
exportfunctiongetExecutionDetails( trade: TradeDetails, maxSlippage?:number, deadline?:number, recipient?:string): ExecutionDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/transact#input-parameters "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
trade| `TradeDetails`| The trade to execute.  
maxSlippage?| `number`| The maximum slippage to allow, in basis points. Defaults to 200 (2%).  
deadline?| `number`| When the transaction will expire. Defaults to 10 minutes in the future.  
recipient?| `number`| An optional recipient address. Defaults to the `msg.sender`  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/transact#example-usage "Direct link to Example Usage")
Method arguments are returned as one of: `BigNumber`, `number`, or `string`. `BigNumber`s are large number objects, `numbers` are small numbers in base 10, and `string`s are addresses.
```
const tradeDetails: TradeDetails =tradeExactEthForTokensWithData(reserves,'1000000000000000000')const executionDetails: ExecutionDetails =awaitgetExecutionDetails(tradeDetails)/*{ // the address of the relevant exchange exchangeAddress: 0x09cabEC1eAd1c0Ba254B09efb3EE13841712bE14, // the name of the method that must be called methodName: "ethToTokenSwapInput", // the id of the method name methodId: "0xf39b5b9b", // the ether value that must be sent with the transaction value: <BigNumber>, // method arguments as an array methodArguments: MethodArgument[]}*/
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/06-transact.md)
Was this helpful?
[PreviousOrchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)[NextConstants](https://docs.uniswap.org/sdk/v1/reference/constants)
On this page
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/transact#function-signature)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/transact#input-parameters)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/transact#example-usage)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/06-transact.md)
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
