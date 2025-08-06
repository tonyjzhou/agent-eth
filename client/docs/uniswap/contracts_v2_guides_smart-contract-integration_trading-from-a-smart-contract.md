# https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [Permit2](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
          * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
          * [Supporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [API](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Governance](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Smart Contract Integration
  * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)


On this page
When trading from a smart contract, the most important thing to keep in mind is that access to an external price source is _required_. Without this, trades can be frontrun for considerable loss.
_Read[safety considerations](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#safety-considerations) for more._
# Using the Router
The easiest way to safely swap tokens is to use the [router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02), which provides a variety of methods to safely swap to and from different assets. You'll notice that there is a function for each permutation of swapping to/from an exact amount of ETH/tokens.
First you must use an external price source to calculate the safety parameters for the function you'd like to call. This is either a minimum amount received when selling an exact input or the maximum amount you are willing to pay when a buying an exact output amount
It is also important to ensure that your contract controls enough ETH/tokens to make the swap, and has granted approval to the router to withdraw this many tokens.
_Check out the[Pricing](https://docs.uniswap.org/contracts/v2/concepts/advanced-topics/pricing#pricing-trades) page for a more in depth discussion on getting prices._
# Example
Imagine you want to swap 50 DAI for as much ETH as possible from your smart contract.
## transferFrom[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#transferfrom "Direct link to transferFrom")
Before swapping, our smart contracts needs to be in control of 50 DAI. The easiest way to accomplish this is by calling `transferFrom` on DAI with the owner set to `msg.sender`:
```
uint amountIn =50*10** DAI.decimals();require(DAI.transferFrom(msg.sender,address(this), amountIn),'transferFrom failed.');
```

## approve[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#approve "Direct link to approve")
Now that our contract owns 50 DAI, we need to approve to the [router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02) to withdraw this DAI:
```
require(DAI.approve(address(UniswapV2Router02), amountIn),'approve failed.');
```

## swapExactTokensForETH[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#swapexacttokensforeth "Direct link to swapExactTokensForETH")
Now we're ready to swap:
```
// amountOutMin must be retrieved from an oracle of some kindaddress[]memory path =newaddress[](2);path[0]=address(DAI);path[1]= UniswapV2Router02.WETH();UniswapV2Router02.swapExactTokensForETH(amountIn, amountOutMin, path, msg.sender, block.timestamp);
```

# Safety Considerations
Because Ethereum transactions occur in an adversarial environment, smart contracts that do not perform safety checks _can be exploited for profit_. If a smart contract assumes that the current price on Uniswap is a "fair" price without performing safety checks, _it is vulnerable to manipulation_. A bad actor could e.g. easily insert transactions before and after the swap (a "sandwich" attack) causing the smart contract to trade at a much worse price, profit from this at the trader's expense, and then return the contracts to their original state. (One important caveat is that these types of attacks are mitigated by trading in extremely liquid pools, and/or at low values.)
The best way to protect against these attacks is to use an external price feed or "price oracle". The best "oracle" is simply _traders' off-chain observation of the current price_ , which can be passed into the trade as a safety check. This strategy is best for situations _where users initiate trades on their own behalf_.
However, when an off-chain price can't be used, an on-chain oracle should be used instead. Determining the best oracle for a given situation is not a part of this guide, but for more details on the Uniswap V2 approach to oracles, see [Oracles](https://docs.uniswap.org/contracts/v2/concepts/core-concepts/oracles).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/02-trading-from-a-smart-contract.md)
Was this helpful?
[PreviousSmart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)[NextProviding Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
On this page
  * [transferFrom](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#transferfrom)
  * [approve](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#approve)
  * [swapExactTokensForETH](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract#swapexacttokensforeth)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/02-trading-from-a-smart-contract.md)
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
