# https://docs.uniswap.org/sdk/v2/guides/fetching-data

[Skip to main content](https://docs.uniswap.org/sdk/v2/guides/fetching-data#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/sdk/v2/overview)
      * [Guides](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [SDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)
        * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)
        * [Pricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
        * [Trading](https://docs.uniswap.org/sdk/v2/guides/trading)
        * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)
      * [Technical Reference](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Getting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
        * [Pair](https://docs.uniswap.org/sdk/v2/reference/pair)
        * [Route](https://docs.uniswap.org/sdk/v2/reference/route)
        * [Trade](https://docs.uniswap.org/sdk/v2/reference/trade)
        * [Other Exports](https://docs.uniswap.org/sdk/v2/reference/other-exports)
    * [v1 SDK](https://docs.uniswap.org/sdk/v1/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v2 SDK
  * Guides
  * [Fetching Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data)


On this page
> Looking for a [quickstart](https://docs.uniswap.org/sdk/v2/guides/quick-start)?
While the SDK is fully self-contained, there are two cases where it needs _on-chain data_ to function. This guide will detail both of these cases, and offer a sample that you can use to fetch this data.
# Case 1: Tokens
Unsurprisingly, the SDK needs some notion of an ERC-20 token to be able to function. This immediately raises the question of _where data about tokens comes from_.
As an example, let's try to represent DAI in a format the SDK can work with. To do so, we need at least 3 pieces of data: a **chainId** , a **token address** , and how many **decimals** the token has. We also may be interested in the **symbol** and/or **name** of the token.
## Identifying Data[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#identifying-data "Direct link to Identifying Data")
The first two pieces of data — **chainId** and **token address** — must be provided by us. Thinking about it, this makes sense, as there's really no other way to unambiguously identify a token.
So, in the case of DAI, we know that the **chainId** is `1` (we're on mainnet), and the **token address** is `0x6B175474E89094C44Da98b954EedeAC495271d0F`. Note that it's very important to externally verify token addresses. Don't use addresses from sources you don't trust!
## Required Data[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#required-data "Direct link to Required Data")
The next piece of data we need is **decimals**.
### Provided by the User[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#provided-by-the-user "Direct link to Provided by the User")
One option here is to simply pass in the correct value, which we may know is `18`. At this point, we're ready to represent DAI as a [Token](https://docs.uniswap.org/sdk/core/reference/classes/Token):
```
import{ ChainId, Token }from'@uniswap/sdk-core'const chainId = ChainId.MAINNETconst tokenAddress ='0x6B175474E89094C44Da98b954EedeAC495271d0F'// must be checksummedconst decimals =18constDAI=newToken(chainId, tokenAddress, decimals)
```

If we don't know or don't want to hardcode the value, we could look it up ourselves via any method of retrieving on-chain data in a function that looks something like:
```
import{ ChainId }from'@uniswap/sdk-core'asyncfunctiongetDecimals(chainId: ChainId, tokenAddress:string):Promise<number>{// Setup provider, import necessary ABI ...const tokenContract =newethers.Contract(tokenAddress, erc20abi, provider)return tokenContract["decimals"]()}
```

## Optional Data[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#optional-data "Direct link to Optional Data")
Finally, we can talk about **symbol** and **name**. Because these fields aren't used anywhere in the SDK itself, they're optional, and can be provided if you want to use them in your application. However, the SDK will not fetch them for you, so you'll have to provide them:
```
import{ ChainId, Token }from'@uniswap/sdk-core'constDAI=newToken(ChainId.MAINNET,'0x6B175474E89094C44Da98b954EedeAC495271d0F',18,'DAI','Dai Stablecoin')
```

# Case 2: Pairs
Now that we've explored how to define a token, let's talk about pairs. To read more about what Uniswap pairs are, see [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
As an example, let's try to represent the DAI-WETH pair.
## Identifying Data[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#identifying-data-1 "Direct link to Identifying Data")
Each pair consists of two tokens (see previous section). Note that WETH used by the router is [exported by the SDK Core as WETH9](https://docs.uniswap.org/sdk/core/reference/overview).
## Required Data[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#required-data-1 "Direct link to Required Data")
The data we need is the _reserves_ of the pair. To read more about reserves, see [getReserves](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair#getreserves).
### Provided by the User[​](https://docs.uniswap.org/sdk/v2/guides/fetching-data#provided-by-the-user-1 "Direct link to Provided by the User")
One option here is to simply pass in values which we've fetched ourselves to create a [Pair](https://docs.uniswap.org/sdk/v2/reference/pair). In this example we use ethers to fetch the data directly from the blockchain:
```
import{ ChainId, Token,WETH9, CurrencyAmount }from'@uniswap/sdk-core'import{ Pair }from'@uniswap/v2-sdk'constDAI=newToken(ChainId.MAINNET,'0x6B175474E89094C44Da98b954EedeAC495271d0F',18)asyncfunctioncreatePair():Promise<Pair>{const pairAddress = Pair.getAddress(DAI,WETH9[DAI.chainId])// Setup provider, import necessary ABI ...const pairContract =newethers.Contract(pairAddress, uniswapV2poolABI, provider)const reserves =await pairContract["getReserves"]()const[reserve0, reserve1]= reservesconst tokens =[DAI,WETH9[DAI.chainId]]const[token0, token1]= tokens[0].sortsBefore(tokens[1])? tokens :[tokens[1], tokens[0]]const pair =newPair(CurrencyAmount.fromRawAmount(token0, reserve0), CurrencyAmount.fromRawAmount(token1, reserve1))return pair}
```

Note that these values can change as frequently as every block, and should be kept up-to-date.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/02-fetching-data.md)
Was this helpful?
[PreviousSDK Quick start](https://docs.uniswap.org/sdk/v2/guides/quick-start)[NextPricing](https://docs.uniswap.org/sdk/v2/guides/pricing)
On this page
  * [Identifying Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data#identifying-data)
  * [Required Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data#required-data)
    * [Provided by the User](https://docs.uniswap.org/sdk/v2/guides/fetching-data#provided-by-the-user)
  * [Optional Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data#optional-data)
  * [Identifying Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data#identifying-data-1)
  * [Required Data](https://docs.uniswap.org/sdk/v2/guides/fetching-data#required-data-1)
    * [Provided by the User](https://docs.uniswap.org/sdk/v2/guides/fetching-data#provided-by-the-user-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/02-fetching-data.md)
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
