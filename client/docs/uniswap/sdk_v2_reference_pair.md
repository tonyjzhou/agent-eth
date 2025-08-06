# https://docs.uniswap.org/sdk/v2/reference/pair

[Skip to main content](https://docs.uniswap.org/sdk/v2/reference/pair#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v2/reference/pair)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Swaps](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Position Management](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Advanced](https://docs.uniswap.org/sdk/v2/reference/pair)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/pair)
    * [v3 SDK](https://docs.uniswap.org/sdk/v2/reference/pair)
    * [Swap Widget](https://docs.uniswap.org/sdk/v2/reference/pair)
    * [web3-react](https://docs.uniswap.org/sdk/v2/reference/pair)
    * [Core SDK](https://docs.uniswap.org/sdk/v2/reference/pair)
    * [v2 SDK](https://docs.uniswap.org/sdk/v2/reference/pair)
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
        * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)
        * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Route](https://docs.uniswap.org/sdk/v2/reference/route)
        * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Other Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
    * [v1 SDK](https://docs.uniswap.org/sdk/v2/reference/pair)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Technical Reference
  * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)


On this page
```
constructor(tokenAmountA: CurrencyAmount, tokenAmountB: CurrencyAmount)
```

The Pair entity represents a Uniswap pair with a balance of each of its pair tokens.
# Example
```
import{ Pair }from'@uniswap/sdk-core'import{ChainId, Token, CurrencyAmount }from'@uniswap/v2-sdk'constHOT=newToken(ChainId.MAINNET,'0xc0FFee0000000000000000000000000000000000',18,'HOT','Caffeine')constNOT=newToken(ChainId.MAINNET,'0xDeCAf00000000000000000000000000000000000',18,'NOT','Caffeine')const pair =newPair(CurrencyAmount.fromRawAmount(HOT,'2000000000000000000'), CurrencyAmount.fromRawAmount(NOT,'1000000000000000000'))
```

# Static Methods
## getAddress[​](https://docs.uniswap.org/sdk/v2/reference/pair#getaddress "Direct link to getAddress")
```
getAddress(tokenA: Token, tokenB: Token):string
```

Computes the pair address for the passed [Tokens](https://docs.uniswap.org/sdk/v2/reference/token). See [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses).
# Properties
## liquidityToken[​](https://docs.uniswap.org/sdk/v2/reference/pair#liquiditytoken "Direct link to liquidityToken")
```
liquidityToken: Token
```

A Token representing the liquidity token for the pair. See [Pair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20).
## token0[​](https://docs.uniswap.org/sdk/v2/reference/pair#token0 "Direct link to token0")
```
token0: Token
```

See [Token0](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#token0).
## token1[​](https://docs.uniswap.org/sdk/v2/reference/pair#token1 "Direct link to token1")
```
token1: Token
```

See [Token1](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#token1).
## reserve0[​](https://docs.uniswap.org/sdk/v2/reference/pair#reserve0 "Direct link to reserve0")
```
reserve0: CurrencyAmount
```

The reserve of token0.
## reserve1[​](https://docs.uniswap.org/sdk/v2/reference/pair#reserve1 "Direct link to reserve1")
```
reserve1: CurrencyAmount
```

The reserve of token1.
# Methods
## reserveOf[​](https://docs.uniswap.org/sdk/v2/reference/pair#reserveof "Direct link to reserveOf")
```
reserveOf(token: Token): CurrencyAmount
```

Returns reserve0 or reserve1, depending on whether token0 or token1 is passed in.
## getOutputAmount[​](https://docs.uniswap.org/sdk/v2/reference/pair#getoutputamount "Direct link to getOutputAmount")
```
getOutputAmount(inputAmount: CurrencyAmount):[CurrencyAmount, Pair]
```

Pricing function for exact input amounts. Returns maximum output amount based on current reserves and the new Pair that would exist if the trade were executed.
## getInputAmount[​](https://docs.uniswap.org/sdk/v2/reference/pair#getinputamount "Direct link to getInputAmount")
```
getInputAmount(outputAmount: CurrencyAmount):[CurrencyAmount, Pair]
```

Pricing function for exact output amounts. Returns minimum input amount based on current reserves and the new Pair that would exist if the trade were executed.
## getLiquidityMinted[​](https://docs.uniswap.org/sdk/v2/reference/pair#getliquidityminted "Direct link to getLiquidityMinted")
```
getLiquidityMinted(totalSupply: CurrencyAmount, tokenAmountA: CurrencyAmount, tokenAmountB: CurrencyAmount): CurrencyAmount
```

Calculates the exact amount of liquidity tokens minted from a given amount of token0 and token1.
  * totalSupply must be looked up on-chain.
  * The value returned from this function _cannot_ be used as an input to getLiquidityValue.


## getLiquidityValue[​](https://docs.uniswap.org/sdk/v2/reference/pair#getliquidityvalue "Direct link to getLiquidityValue")
```
getLiquidityValue( token: Token, totalSupply: CurrencyAmount, liquidity: CurrencyAmount, feeOn:boolean=false, kLast?: BigintIsh): CurrencyAmount
```

Calculates the exact amount of token0 or token1 that the given amount of liquidity tokens represent.
  * totalSupply must be looked up on-chain.
  * If the protocol charge is on, feeOn must be set to true, and kLast must be provided from an on-chain lookup.
  * Values returned from this function _cannot_ be used as inputs to getLiquidityMinted.


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/02-pair.md)
Was this helpful?
[PreviousGetting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)[NextRoute](https://docs.uniswap.org/sdk/v2/reference/route)
On this page
  * [getAddress](https://docs.uniswap.org/sdk/v2/reference/pair#getaddress)
  * [liquidityToken](https://docs.uniswap.org/sdk/v2/reference/pair#liquiditytoken)
  * [token0](https://docs.uniswap.org/sdk/v2/reference/pair#token0)
  * [token1](https://docs.uniswap.org/sdk/v2/reference/pair#token1)
  * [reserve0](https://docs.uniswap.org/sdk/v2/reference/pair#reserve0)
  * [reserve1](https://docs.uniswap.org/sdk/v2/reference/pair#reserve1)
  * [reserveOf](https://docs.uniswap.org/sdk/v2/reference/pair#reserveof)
  * [getOutputAmount](https://docs.uniswap.org/sdk/v2/reference/pair#getoutputamount)
  * [getInputAmount](https://docs.uniswap.org/sdk/v2/reference/pair#getinputamount)
  * [getLiquidityMinted](https://docs.uniswap.org/sdk/v2/reference/pair#getliquidityminted)
  * [getLiquidityValue](https://docs.uniswap.org/sdk/v2/reference/pair#getliquidityvalue)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/reference/02-pair.md)
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
