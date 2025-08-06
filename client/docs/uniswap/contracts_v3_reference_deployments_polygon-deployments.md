# https://docs.uniswap.org/contracts/v3/reference/deployments/polygon-deployments

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/deployments/polygon-deployments#__docusaurus_skipToContent_fallback)
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
  * [Polygon Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/polygon-deployments)


# Uniswap Contract Deployments
The latest version of `@uniswap/v3-core`, `@uniswap/v3-periphery`, and `@uniswap/swap-router-contracts` are deployed at the addresses listed below. Integrators should **no longer assume that they are deployed to the same addresses across chains** and be extremely careful to confirm mappings below.
Contract| Polygon Addresses| Polygon Mumbai  
---|---|---  
[UniswapV3Factory](https://github.com/Uniswap/uniswap-v3-core/blob/v1.0.0/contracts/UniswapV3Factory.sol)| `0x1F98431c8aD98523631AE4a59f267346ea31F984`| `0x1F98431c8aD98523631AE4a59f267346ea31F984`  
[Multicall](https://polygonscan.com/address/0x1F98415757620B543A52E61c46B32eB19261F984#code)| `0x1F98415757620B543A52E61c46B32eB19261F984`| `0x1F98415757620B543A52E61c46B32eB19261F984`  
[ProxyAdmin](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.1-solc-0.7-2/contracts/proxy/ProxyAdmin.sol)| `0xB753548F6E010e7e680BA186F9Ca1BdAB2E90cf2`| `0xB753548F6E010e7e680BA186F9Ca1BdAB2E90cf2`  
[TickLens](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/lens/TickLens.sol)| `0xbfd8137f7d1516D3ea5cA83523914859ec47F573`| `0xbfd8137f7d1516D3ea5cA83523914859ec47F573`  
[Quoter](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/lens/Quoter.sol)| `0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6`| `0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6`  
[SwapRouter](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/SwapRouter.sol)| `0xE592427A0AEce92De3Edee1F18E0157C05861564`| `0xE592427A0AEce92De3Edee1F18E0157C05861564`  
[NFTDescriptor](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/libraries/NFTDescriptor.sol)| `0x42B24A95702b9986e82d421cC3568932790A48Ec`| `0x42B24A95702b9986e82d421cC3568932790A48Ec`  
[NonfungibleTokenPositionDescriptor](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/NonfungibleTokenPositionDescriptor.sol)| `0x91ae842A5Ffd8d12023116943e72A606179294f3`| `0x91ae842A5Ffd8d12023116943e72A606179294f3`  
[TransparentUpgradeableProxy](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v3.4.1-solc-0.7-2/contracts/proxy/TransparentUpgradeableProxy.sol)| `0xEe6A57eC80ea46401049E92587E52f5Ec1c24785`| `0xEe6A57eC80ea46401049E92587E52f5Ec1c24785`  
[NonfungiblePositionManager](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/NonfungiblePositionManager.sol)| `0xC36442b4a4522E871399CD717aBDD847Ab11FE88`| `0xC36442b4a4522E871399CD717aBDD847Ab11FE88`  
[V3Migrator](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/V3Migrator.sol)| `0xA5644E29708357803b5A882D272c41cC0dF92B34`| `0xA5644E29708357803b5A882D272c41cC0dF92B34`  
[QuoterV2](https://github.com/Uniswap/v3-periphery/blob/main/contracts/lens/QuoterV2.sol)| `0x61fFE014bA17989E743c5F6cB21bF9697530B21e`| `0x61fFE014bA17989E743c5F6cB21bF9697530B21e`  
[SwapRouter02](https://github.com/Uniswap/swap-router-contracts/blob/main/contracts/SwapRouter02.sol)| `0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45`| `0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45`  
[Permit2](https://github.com/Uniswap/permit2)| `0x000000000022D473030F116dDEE9F6B43aC78BA3`| `0x000000000022D473030F116dDEE9F6B43aC78BA3`  
[UniversalRouter](https://github.com/Uniswap/universal-router)| `0x1095692A6237d83C6a72F3F5eFEdb9A670C49223`| `0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7FAD`  
[v3StakerAddress](https://github.com/Uniswap/v3-staker)| `0xe34139463bA50bD61336E0c446Bd8C0867c6fE65`| `0xe34139463bA50bD61336E0c446Bd8C0867c6fE65`  
These addresses are final and were deployed from these npm package versions:
  * [`@uniswap/v3-core@1.0.0`](https://github.com/Uniswap/uniswap-v3-core/tree/v1.0.0)
  * [`@uniswap/v3-periphery@1.0.0`](https://github.com/Uniswap/uniswap-v3-periphery/tree/v1.0.0)
  * [`@uniswap/swap-router-contracts@1.1.0`](https://github.com/Uniswap/swap-router-contracts/tree/v1.1.0)


# Uniswap v3 Staker
An up-to-date list of [deploy addresses by chain is hosted on GitHub](https://github.com/Uniswap/v3-staker/releases/tag/v1.0.2) for the `UniswapV3Staker` contract.
# Universal Router
The `UniversalRouter` contract is the current preferred entrypoint for ERC20 and NFT swaps, replacing, among other contracts, `SwapRouter02`. An up-to-date list of [deploy addresses by chain is hosted on GitHub](https://github.com/Uniswap/universal-router/tree/main/deploy-addresses).
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
Polygon| `137`| WMATIC| `0x0d500B1d8E8eF31E21C99d1Db9A6444d3ADf1270`  
Polygon Mumbai| `80001`| WMATIC| `0x9c3C9283D3e44854697Cd22D3Faa240Cfb032889`  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/deployments/Polygon-Deployments.md)
Was this helpful?
[PreviousOptimism Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/optimism-deployments)[NextUnichain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/unichain-deployments)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/deployments/Polygon-Deployments.md)
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
