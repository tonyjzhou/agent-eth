# https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses

[Skip to main content](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses#__docusaurus_skipToContent_fallback)
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
  * [Pair Addresses](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses)


On this page
# getPair
The most obvious way to get the address for a pair is to call [getPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#getpair) on the factory. If the pair exists, this function will return its address, else `address(0)` (`0x0000000000000000000000000000000000000000`).
  * The "canonical" way to determine whether or not a pair exists.
  * Requires an on-chain lookup.


# CREATE2
Thanks to some [fancy footwork in the factory](https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Factory.sol#L32), we can also compute pair addresses _without any on-chain lookups_ because of [CREATE2](https://eips.ethereum.org/EIPS/eip-1014). The following values are required for this technique:
|   
---|---  
`address`| The [factory address](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)  
`salt`| `keccak256(abi.encodePacked(token0, token1))`  
`keccak256(init_code)`| `0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f`  
  * `token0` must be strictly less than `token1` by sort order.


  * Can be computed offline.
  * Requires the ability to perform `keccak256`.


## Examples[​](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses#examples "Direct link to Examples")
### TypeScript[​](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses#typescript "Direct link to TypeScript")
This example makes use of the [Uniswap V2 SDK](https://docs.uniswap.org/sdk/v2/reference/getting-started). In reality, the SDK computes pair addresses behind the scenes, obviating the need to compute them manually like this.
```
import{FACTORY_ADDRESS,INIT_CODE_HASH}from'@uniswap/v2-sdk'import{ pack, keccak256 }from'@ethersproject/solidity'import{ getCreate2Address }from'@ethersproject/address'const token0 ='0xCAFE000000000000000000000000000000000000'// change me!const token1 ='0xF00D000000000000000000000000000000000000'// change me!const pair =getCreate2Address(FACTORY_ADDRESS,keccak256(['bytes'],[pack(['address','address'],[token0, token1])]),INIT_CODE_HASH)
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/05-getting-pair-addresses.md)
Was this helpful?
[PreviousTrading](https://docs.uniswap.org/sdk/v2/guides/trading)[NextGetting Started](https://docs.uniswap.org/sdk/v2/reference/getting-started)
On this page
  * [Examples](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses#examples)
    * [TypeScript](https://docs.uniswap.org/sdk/v2/guides/getting-pair-addresses#typescript)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v2/guides/05-getting-pair-addresses.md)
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
