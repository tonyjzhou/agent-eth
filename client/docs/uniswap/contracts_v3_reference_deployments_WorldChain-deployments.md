# https://docs.uniswap.org/contracts/v3/reference/deployments/WorldChain-deployments

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/deployments/WorldChain-deployments#__docusaurus_skipToContent_fallback)
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
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Overview](https://docs.uniswap.org/contracts/v3/reference/overview)
        * [Core](https://docs.uniswap.org/contracts/v3/reference/core/UniswapV3Factory)
        * [Governance](https://docs.uniswap.org/contracts/v3/reference/governance/overview)
        * [Periphery](https://docs.uniswap.org/contracts/v3/reference/periphery/base/BlockTimestamp)
        * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
          * [AVAX Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/avax-deployments)
          * [Arbitrum Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/arbitrum-deployments)
          * [BNB Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/bnb-deployments)
          * [Base Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/base-deployments)
          * [Blast Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/blast-deployments)
          * [CELO Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/celo-deployments)
          * [Ethereum Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/ethereum-deployments)
          * [Optimism Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/optimism-deployments)
          * [Polygon Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/polygon-deployments)
          * [Unichain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/unichain-deployments)
          * [WorldChain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/WorldChain-deployments)
          * [ZKsync Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/ZKsync-deployments)
          * [Zora Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/Zora-deployments)
        * [Error Codes](https://docs.uniswap.org/contracts/v3/reference/error-codes)
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v3 Protocol
  * Technical Reference
  * [Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/)
  * [WorldChain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/WorldChain-deployments)


# Uniswap Contract Deployments
The latest version of `@uniswap/v3-core`, `@uniswap/v3-periphery`, and `@uniswap/swap-router-contracts` are deployed at the addresses listed below. Integrators should **no longer assume that they are deployed to the same addresses across chains** and be extremely careful to confirm mappings below.
Contract| WorldChain  
---|---  
[UniswapV3Factory](https://github.com/Uniswap/uniswap-v3-core/blob/v1.0.0/contracts/UniswapV3Factory.sol)| `0x7a5028BDa40e7B173C278C5342087826455ea25a`  
[Multicall2](https://worldchain-mainnet.explorer.alchemy.com/address/0x0a22c04215c97E3F532F4eF30e0aD9458792dAB9)| `0x0a22c04215c97E3F532F4eF30e0aD9458792dAB9`  
[ProxyAdmin](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.1-solc-0.7-2/contracts/proxy/ProxyAdmin.sol)| `0x8B52DaCB7B5d9A959CDcD5419061c0eDD1296c29`  
[TickLens](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/lens/TickLens.sol)| `0xE61df0CaC9d85876aCE5E3037005D80943570623`  
[NFTDescriptor](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/libraries/NFTDescriptor.sol)| `0x38c68A1D60C47973EcE5bc1725B65D8Bec438192`  
[NonfungibleTokenPositionDescriptor](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/NonfungibleTokenPositionDescriptor.sol)| `0x70410a302c4a5c52C659b780941c947Abd437FeB`  
[TransparentUpgradeableProxy](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.1-solc-0.7-2/contracts/proxy/TransparentUpgradeableProxy.sol)| `0xe6FcB4952b2d3Fab6DA4BC165831f5575e093feC`  
[NonfungiblePositionManager](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/NonfungiblePositionManager.sol)| `0xec12a9F9a09f50550686363766Cc153D03c27b5e`  
[V3Migrator](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/V3Migrator.sol)| `0x9EBDdCBa71C9027E1eB45135672a30bcFEec9de3`  
[QuoterV2](https://github.com/Uniswap/v3-periphery/blob/main/contracts/lens/QuoterV2.sol)| `0x10158D43e6cc414deE1Bd1eB0EfC6a5cBCfF244c`  
[SwapRouter02](https://github.com/Uniswap/swap-router-contracts/blob/main/contracts/SwapRouter02.sol)| `0x091AD9e2e6e5eD44c1c66dB50e49A601F9f36cF6`  
[Permit2](https://github.com/Uniswap/permit2)| `0x000000000022D473030F116dDEE9F6B43aC78BA3`  
[UniversalRouter](https://github.com/Uniswap/universal-router)| `0x8ac7bee993bb44dab564ea4bc9ea67bf9eb5e743`  
These addresses are final and were deployed from these npm package versions:
  * [`@uniswap/v3-core@1.0.0`](https://github.com/Uniswap/uniswap-v3-core/tree/v1.0.0)
  * [`@uniswap/v3-periphery@1.0.0`](https://github.com/Uniswap/uniswap-v3-periphery/tree/v1.0.0)
  * [`@uniswap/swap-router-contracts@1.1.0`](https://github.com/Uniswap/swap-router-contracts/tree/v1.1.0)


# Universal Router
The `UniversalRouter` contract is the current preferred entrypoint for ERC20 and NFT swaps, replacing, among other contracts, `SwapRouter02`. An up-to-date list of [deploy addresses by chain is hosted on GitHub](https://github.com/Uniswap/sdks/blob/main/sdks/universal-router-sdk/src/utils/constants.ts).
# Uniswap Pool Deployments
Every Uniswap pool is a unique instance of the `UniswapV3Pool` contract and is deployed at its own unique address. The contract source code of the pool will be auto-verified on etherscan. For example, here is the [ETH/USDC 0.3% pool](https://etherscan.io/address/0x8ad599c3a0ff1de082011efddc58f1908eb6e6d8) on Ethereum mainnet.
You can look up the address of an existing pool on [Uniswap Info](https://info.uniswap.org/#/) or by calling the [`getPool`](https://docs.uniswap.org/contracts/v3/reference/reference/core/interfaces/IUniswapV3Factory.md#getpool) function on the `UniswapV3Factory` contract.
```
getPool("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48","0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",3000)
```

# Wrapped Native Token Addresses
The Uniswap Protocol supports trading of ERC20 tokens. In order to swap a native asset like ETH (or MATIC on Polygon), the Uniswap protocol wraps these assets in an ERC20 wrapped native token contract. The protocol uses the following WETH9 addresses on Ethereum and WMATIC addresses on Polygon.
Network| ChainId| Wrapped Native Token| Address  
---|---|---|---  
WorldChain| `480`| WETH| `0x4200000000000000000000000000000000000006`  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/deployments/WorldChain-Deployments.md)
Was this helpful?
[PreviousUnichain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/unichain-deployments)[NextZKsync Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/ZKsync-deployments)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/deployments/WorldChain-Deployments.md)
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
