# https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/concepts/v4-vs-v3)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/quickstart/manage-liquidity/setup-liquidity)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/concepts/protocol-overview/how-uniswap-works)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/swaps)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/fees)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/using-the-api)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
          * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
          * [Supporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [API](https://docs.uniswap.org/contracts/v2/reference/API/overview)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/Governance/governance-reference)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Smart Contract Integration
  * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)


On this page
# getPair
The most obvious way to get the address for a pair is to call [getPair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#getpair) on the factory. If the pair exists, this function will return its address, else `address(0)` (`0x0000000000000000000000000000000000000000`).
  * The "canonical" way to determine whether or not a pair exists.
  * Requires an on-chain lookup.


# CREATE2
Thanks to some [fancy footwork in the factory](https://github.com/Uniswap/uniswap-v2-core/blob/master/contracts/UniswapV2Factory.sol#L32), we can also compute pair addresses _without any on-chain lookups_ because of [CREATE2](https://eips.ethereum.org/EIPS/eip-1014). The following values are required for this technique:
|   
---|---  
`address`| The [factory address](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#address)  
`salt`| `keccak256(abi.encodePacked(token0, token1))`  
`keccak256(init_code)`| `0x96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f`  
  * `token0` must be strictly less than `token1` by sort order.


  * Can be computed offline.
  * Requires the ability to perform `keccak256`.


## Examples[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses#examples "Direct link to Examples")
### Solidity[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses#solidity "Direct link to Solidity")
```
address factory =0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f;address token0 =0xCAFE000000000000000000000000000000000000;// change me!address token1 =0xF00D000000000000000000000000000000000000;// change me!address pair =address(uint(keccak256(abi.encodePacked( hex'ff', factory,keccak256(abi.encodePacked(token0, token1)), hex'96e8ac4277198ff8b6f785478aa9a39f403cb768dd02cbee326c3e7da348845f'))));
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/06-getting-pair-addresses.md)
Was this helpful?
[PreviousFlash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)[NextSupporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
On this page
  * [Examples](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses#examples)
    * [Solidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses#solidity)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/06-getting-pair-addresses.md)
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
