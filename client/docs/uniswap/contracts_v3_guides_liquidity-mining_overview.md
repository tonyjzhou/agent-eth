# https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
      * [Quickstart](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
          * [Overview](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [Permit2](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Guides
  * Liquidity Mining
  * [Overview](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)


On this page
# Overview
## Introduction[​](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#introduction "Direct link to Introduction")
As a DeFi project, token creator, or other interested party, you may want to _incentivize in-range liquidity provision_ on a Uniswap V3 pool. This guide describes one particular incentivization scheme at a high level, as implemented in [uniswap-v3-staker](https://github.com/Uniswap/uniswap-v3-staker).
## The Setting[​](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#the-setting "Direct link to The Setting")
Let's start by defining some terms. We refer to programs which incentivize liquidity as `Incentive`s; they're characterized by the following parameters:
  * `rewardToken`: Perhaps the most important parameter, would-be incentivizers must pick the ERC20 token which they would like to distribute as a reward for providing liquidity.
  * `pool`: The address of the Uniswap V3 pool in which liquidity must be provided.
  * `startTime`: The UNIX timestamp at which rewards start to be distributed.
  * `endTime`: The UNIX timestamp at which rewards start to decay.
  * `refundee`: The address which has the right to reclaim any leftover rewards after the `Incentive` has concluded.


Finally, every `Incentive` has an associated `reward`, the total amount of `rewardToken`s that are allocated to be distributed over the lifecycle of the program.
## Reward Math[​](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#reward-math "Direct link to Reward Math")
Now that we have an idea of what an `Incentive` looks like, let's explore how rewards are actually allocated to participants. The next section will touch on the participation mechanics, so for now let's abstract this away and just focus on the high-level design.
Recall that `Incentive` creators pick a `reward` amount and a program duration. This directly corresponds to picking _an amount of`rewardToken` s to distribute per second_; let's call this the reward rate. So, for every second between `startTime` and `endTime`, a constant amount of tokens are distributed proportionally _among all in-range liquidity at that second_. Crucially, this counts _all_ liquidity, not just liquidity that opts in to participating in the program. So, incentive creators should pick a reward rate that they deem worthwhile to distribute across (potentially) all in-range LPs for the duration of the program.
## Staking[​](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#staking "Direct link to Staking")
So, how do users participate in these programs? Note that this section requires a basic understanding of [how Uniswap V3 position NFTs work](https://docs.uniswap.org/contracts/v3/reference/periphery/NonfungiblePositionManager)
The first action a user must take in order to begin participating in an `Incentive` is to _deposit_ their position NFT into the [canonical staking contract address](https://github.com/Uniswap/uniswap-v3-staker#deployments), effectively temporarily giving custody over their NFT to this contract. This is necessary because, as we'll see later on, the staking contract needs to be able to guarantee that liquidity cannot be removed from NFTs participating in the program.
Once deposited, a user may then _stake_ their NFT into any number of active `Incentive`s for the Uniswap V3 pool their NFT is tied to (note that this can happen atomically with an initial _deposit_). Staked NFTs then immediately start to earn rewards, according to the algorithm outlined above. Users may periodically claim accrued `rewardToken`s while the program is ongoing, or wait to claim until the program has concluded to minimize overhead.
## Program Conclusion[​](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#program-conclusion "Direct link to Program Conclusion")
There are two conditions that must be met for a program to be considered concluded:
  1. `block.timestamp >= endTime`: In other words, the program's duration must have expired. However, this doesn't mark the official end of the program, as some users may still be participating right up unti this `endTime` boundary and beyond, to maximize their rewards. This leads directly to the second condition.
  2. All NFTs must be unstaked: A program can conclude only when every NFT which participated in it is unstaked. To ensure this is always possible, after the `endTime` of a program _anyone_ may unstake _any_ NFT (though of course they may not claim outstanding `rewardToken`s due to the NFT owner). This ensures that even if all users do not unstake themselves, someone can unstake them manually so that the program can end.


It's important that most or all programs fully conclude, primarily so that the `refundee` can reclaim any unallocated rewards. What are the conditions under which unallocated rewards will remain? Well, recall that the reward rate is the same across _all_ in-range liquidity. However, only program participants may actually claim accrued tokens, so it's likely that all programs will end up with a balance of `rewardToken`s that cannot be claimed. So, `refundee`s will typically be incentivized to bring programs to an official conclusion. This slightly cumbersome design is a consequence of the difficulty of consistently allocating rewards proportional to Uniswap V3 liquidity.
A final note: stakers who remain in the program after `endTime` may actually see their rewards marginally augmented or (more likely) gradually diluted. The magnitude of these changes depend on stakers' share of the total active liquidity, the time spend staked after `endTime`, and the sequence of unstaking. In the worst case, rewards decay proportionally to the duration. For example, at 2x the duration, ½ of rewards could remain, at 3x, ⅓ could remain, etc. While somewhat complex, this behavior can largely be ignored from a game-theoretic standpoint. Stakers should simply attempt to unstake and claim rewards as soon as possible after `endTime`, an outcome that is likely in any case, as `refundee`s will be eager to reclaim leftover rewards, and mass unstake stragglers.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/liquidity-mining/overview.md)
Was this helpful?
[PreviousThe Full Contract](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/the-full-contract)[NextGetting Started](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
On this page
  * [Introduction](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#introduction)
  * [The Setting](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#the-setting)
  * [Reward Math](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#reward-math)
  * [Staking](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#staking)
  * [Program Conclusion](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview#program-conclusion)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/liquidity-mining/overview.md)
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
