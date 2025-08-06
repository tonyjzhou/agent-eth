# https://docs.uniswap.org/sdk/v1/reference/orchestration

[Skip to main content](https://docs.uniswap.org/sdk/v1/reference/orchestration#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v1/reference/orchestration)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Swaps](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Position Management](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Advanced](https://docs.uniswap.org/sdk/v1/reference/orchestration)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/orchestration)
    * [v3 SDK](https://docs.uniswap.org/sdk/v1/reference/orchestration)
    * [Swap Widget](https://docs.uniswap.org/sdk/v1/reference/orchestration)
    * [web3-react](https://docs.uniswap.org/sdk/v1/reference/orchestration)
    * [Core SDK](https://docs.uniswap.org/sdk/v1/reference/orchestration)
    * [v2 SDK](https://docs.uniswap.org/sdk/v1/reference/orchestration)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/reference/orchestration)
      * [Overview](https://docs.uniswap.org/sdk/v1/overview)
      * [Guides](https://docs.uniswap.org/sdk/v1/reference/orchestration)
        * [Getting Started](https://docs.uniswap.org/sdk/v1/guides/getting-started)
      * [Technical Reference](https://docs.uniswap.org/sdk/v1/reference/orchestration)
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
  * [Orchestration](https://docs.uniswap.org/sdk/v1/reference/orchestration)


On this page
Orchestration functions are plain-english wrappers for the function defined in [/sdk/1.0.0/reference/data](https://docs.uniswap.org/sdk/v1/reference/Data) and [Computation](https://docs.uniswap.org/sdk/1.0.0/reference/computation).
Functions suffixed with `WithData` are synchronous, and require token reserves to be passed in as arguments. Functions without the suffix are asychronous, and require token addresses to be passed in as arguments.
# tradeExactEthForTokensWithData
The function facilitates trading an exact amount of ETH for a specified token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature "Direct link to Function Signature")
```
exportfunctiontradeExactEthForTokensWithData(reserves: OptionalReserves, ethAmount: BigNumberish): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
reserves| `OptionalReserves`| Reserves data for the output token.  
ethAmount| `BigNumberish`| The input amount of ETH.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =tradeExactEthForTokensWithData(reserves,'1000000000000000000')
```

# tradeExactEthForTokens
The function facilitates trading an exact amount of ETH for a specified token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-1 "Direct link to Function Signature")
```
exportasyncfunctiontradeExactEthForTokens( tokenAddress:string, ethAmount: BigNumberish, chainIdOrProvider?: ChainIdOrProvider):Promise<TradeDetails>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-1 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddress| `string`| Address of output token.  
ethAmount| `BigNumberish`| The input amount of ETH.  
chainIdOrProvider?| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-1 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =awaittradeExactEthForTokens(tokenAddress,'1000000000000000000')
```

# tradeEthForExactTokensWithData
The function facilitates trading ETH for an exact amount of a specified token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-2 "Direct link to Function Signature")
```
exportfunctiontradeEthForExactTokensWithData(reserves: OptionalReserves, tokenAmount: BigNumberish): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-2 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
reserves| `OptionalReserves`| Reserves data for the output token.  
tokenAmount| `BigNumberish`| The output amount of tokens.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-2 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =tradeEthForExactTokensWithData(reserves,'1000000000000000000')
```

# tradeEthForExactTokens
The function facilitates trading ETH for an exact amount of a specified token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-3 "Direct link to Function Signature")
```
exportasyncfunctiontradeEthForExactTokens( tokenAddress:string, tokenAmount: BigNumberish, chainIdOrProvider?: ChainIdOrProvider):Promise<TradeDetails>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-3 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddress| `string`| Address of output token.  
tokenAmount| `BigNumberish`| The output amount of tokens.  
chainIdOrProvider?| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-3 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =awaittradeEthForExactTokens(tokenAddress,'1000000000000000000')
```

# tradeExactTokensForEthWithData
The function facilitates trading an exact amount of a specified token for ETH.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-4 "Direct link to Function Signature")
```
exportfunctiontradeExactTokensForEthWithData(reserves: OptionalReserves, tokenAmount: BigNumberish): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-4 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
reserves| `OptionalReserves`| Reserves data for the input token.  
tokenAmount| `BigNumberish`| The input amount of tokens.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-4 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =tradeExactTokensForEthWithData(reserves,'1000000000000000000')
```

# tradeExactTokensForEth
The function facilitates trading an exact amount of a specified token for ETH.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-5 "Direct link to Function Signature")
```
exportasyncfunctiontradeExactTokensForEth( tokenAddress:string, tokenAmount: BigNumberish, chainIdOrProvider?: ChainIdOrProvider):Promise<TradeDetails>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-5 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddress| `string`| Address of input token.  
tokenAmount| `BigNumberish`| The input amount of tokens.  
chainIdOrProvider?| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-5 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =awaittradeExactTokensForEth(tokenAddress,'1000000000000000000')
```

# tradeTokensForExactEthWithData
The function facilitates trading a specified token for an exact amount of ETH.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-6 "Direct link to Function Signature")
```
exportfunctiontradeTokensForExactEthWithData(reserves: OptionalReserves, ethAmount: BigNumberish): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-6 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
reserves| `OptionalReserves`| Reserves data for the input token.  
ethAmount| `BigNumberish`| The outpute amount of ETH.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-6 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =tradeTokensForExactEthWithData(reserves,'1000000000000000000')
```

# tradeTokensForExactEth
The function facilitates trading a specified token for an exact amount of ETH.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-7 "Direct link to Function Signature")
```
exportasyncfunctiontradeTokensForExactEth( tokenAddress:string, ethAmount: BigNumberish, chainIdOrProvider?: ChainIdOrProvider):Promise<TradeDetails>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-7 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddress| `string`| Address of input token.  
ethAmount| `BigNumberish`| The output amount of ETH.  
chainIdOrProvider?| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-7 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =awaittradeTokensForExactEth(tokenAddress,'1000000000000000000')
```

# tradeExactTokensForTokensWithData
The function facilitates trading an exact amount of a specified token for another token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-8 "Direct link to Function Signature")
```
exportfunctiontradeExactTokensForTokensWithData( reservesInput: OptionalReserves, reservesOutput: OptionalReserves, tokenAmount: BigNumberish): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-8 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
reservesInput| `OptionalReserves`| Reserves data for the input token.  
reservesOutput| `OptionalReserves`| Reserves data for the output token.  
tokenAmount| `BigNumberish`| The input amount of tokens.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-8 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =tradeExactTokensForTokensWithData( reservesInput, reservesOutput,'1000000000000000000')
```

# tradeExactTokensForTokens
The function facilitates trading an exact amount of a specified token for another token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-9 "Direct link to Function Signature")
```
exportasyncfunctiontradeExactTokensForTokens( tokenAddressInput:string, tokenAddressOutput:string, tokenAmount: BigNumberish, chainIdOrProvider?: ChainIdOrProvider):Promise<TradeDetails>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-9 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddressInput| `string`| Address of input token.  
tokenAddressOutput| `string`| Address of output token.  
tokenAmount| `BigNumberish`| The input amount of tokens.  
chainIdOrProvider?| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-9 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =awaittradeExactTokensForTokens( tokenAddressInput, tokenAddressOutput,'1000000000000000000')
```

# tradeTokensForExactTokensWithData
The function facilitates trading a specified token for an exact amount of another token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-10 "Direct link to Function Signature")
```
exportfunctiontradeTokensForExactTokensWithData( reservesInput: OptionalReserves, reservesOutput: OptionalReserves, tokenAmount: BigNumberish): TradeDetails
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-10 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
reservesInput| `OptionalReserves`| Reserves data for the input token.  
reservesOutput| `OptionalReserves`| Reserves data for the output token.  
tokenAmount| `BigNumberish`| The output amount of tokens.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-10 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =tradeTokensForExactTokensWithData( reservesInput, reservesOutput,'1000000000000000000')
```

# tradeTokensForExactTokens
The function facilitates trading an exact amount of a specified token for another token.
## Function Signature[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-11 "Direct link to Function Signature")
```
exportasyncfunctiontradeTokensForExactTokens( tokenAddressInput:string, tokenAddressOutput:string, tokenAmount: BigNumberish, chainIdOrProvider?: ChainIdOrProvider):Promise<TradeDetails>
```

## Input Parameters[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-11 "Direct link to Input Parameters")
Parameter| Type| Description  
---|---|---  
tokenAddressInput| `string`| Address of input token.  
tokenAddressOutput| `string`| Address of output token.  
tokenAmount| `BigNumberish`| The output amount of tokens.  
chainIdOrProvider?| `ChainIdOrProvider`| A supported chain id (`1`, `3`, `4`, or `42`), or an [underlying web3 provider](https://docs.ethers.io/ethers.js/html/api-providers.html#web3provider-inherits-from-jsonrpcprovider) connected to a chain with a supported chain id.  
## Example Usage[​](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-11 "Direct link to Example Usage")
```
const tradeDetails: TradeDetails =awaittradeTokensForExactTokens( tokenAddressInput, tokenAddressOutput,'1000000000000000000')
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/05-orchestration.md)
Was this helpful?
[PreviousFormat](https://docs.uniswap.org/sdk/v1/reference/format)[NextTransact](https://docs.uniswap.org/sdk/v1/reference/transact)
On this page
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-1)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-1)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-1)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-2)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-2)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-2)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-3)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-3)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-3)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-4)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-4)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-4)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-5)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-5)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-5)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-6)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-6)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-6)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-7)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-7)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-7)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-8)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-8)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-8)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-9)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-9)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-9)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-10)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-10)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-10)
  * [Function Signature](https://docs.uniswap.org/sdk/v1/reference/orchestration#function-signature-11)
  * [Input Parameters](https://docs.uniswap.org/sdk/v1/reference/orchestration#input-parameters-11)
  * [Example Usage](https://docs.uniswap.org/sdk/v1/reference/orchestration#example-usage-11)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v1/reference/05-orchestration.md)
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
