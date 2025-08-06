# https://docs.uniswap.org/contracts/v3/reference/deployments/unichain-deployments

[Skip to main content](https://docs.uniswap.org/contracts/v3/reference/deployments/unichain-deployments#__docusaurus_skipToContent_fallback)
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
  * [Unichain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/unichain-deployments)


# Uniswap Contract Deployments
The latest version of `@uniswap/v3-core`, `@uniswap/v3-periphery`, and `@uniswap/swap-router-contracts` are deployed at the addresses listed below. Integrators should **no longer assume that they are deployed to the same addresses across chains** and be extremely careful to confirm mappings below.
Contract| Unichain Addresses| Unichain Sepolia Addresses  
---|---|---  
[UniswapV3Factory](https://github.com/Uniswap/uniswap-v3-core/blob/v1.0.0/contracts/UniswapV3Factory.sol)| [`0x1f98400000000000000000000000000000000003`](https://uniscan.xyz/address/0x1f98400000000000000000000000000000000003)| [`0x1F98431c8aD98523631AE4a59f267346ea31F984`](https://sepolia.uniscan.xyz/address/0x1F98431c8aD98523631AE4a59f267346ea31F984)  
[UniswapInterfaceMulticall](https://github.com/Uniswap/v3-periphery/blob/main/contracts/lens/UniswapInterfaceMulticall.sol)| [`0xb7610f9b733e7d45184be3a1bc966960ccc54f0b`](https://uniscan.xyz/address/0xb7610f9b733e7d45184be3a1bc966960ccc54f0b)| [`0x9D0F15f2cf58655fDDcD1EE6129C547fDaeD01b1`](https://sepolia.uniscan.xyz/address/0x9D0F15f2cf58655fDDcD1EE6129C547fDaeD01b1)  
[TickLens](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/lens/TickLens.sol)| [`0xd5d76fa166ab8d8ad4c9f61aaa81457b66cbe443`](https://uniscan.xyz/address/0xd5d76fa166ab8d8ad4c9f61aaa81457b66cbe443)| [`0x5f739c790a48E97eec0efb81bab5D152c0A0ecA0`](https://sepolia.uniscan.xyz/address/0x5f739c790a48E97eec0efb81bab5D152c0A0ecA0)  
[QuoterV2](https://github.com/Uniswap/v3-periphery/blob/main/contracts/lens/QuoterV2.sol)| [`0x385a5cf5f83e99f7bb2852b6a19c3538b9fa7658`](https://uniscan.xyz/address/0x385a5cf5f83e99f7bb2852b6a19c3538b9fa7658)| [`0x6Dd37329A1A225a6Fca658265D460423DCafBF89`](https://sepolia.uniscan.xyz/address/0x6Dd37329A1A225a6Fca658265D460423DCafBF89)  
[SwapRouter02](https://github.com/Uniswap/swap-router-contracts/blob/main/contracts/SwapRouter02.sol)| [`0x73855d06de49d0fe4a9c42636ba96c62da12ff9c`](https://uniscan.xyz/address/0x73855d06de49d0fe4a9c42636ba96c62da12ff9c)| [`0xd1AAE39293221B77B0C71fBD6dCb7Ea29Bb5B166`](https://sepolia.uniscan.xyz/address/0xd1AAE39293221B77B0C71fBD6dCb7Ea29Bb5B166)  
[NonfungiblePositionManager](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/NonfungiblePositionManager.sol)| [`0x943e6e07a7e8e791dafc44083e54041d743c46e9`](https://uniscan.xyz/address/0x943e6e07a7e8e791dafc44083e54041d743c46e9)| [`0xB7F724d6dDDFd008eFf5cc2834edDE5F9eF0d075`](https://sepolia.uniscan.xyz/address/0xB7F724d6dDDFd008eFf5cc2834edDE5F9eF0d075)  
[NFT Descriptor](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/libraries/NFTDescriptor.sol)| [`0x0dfa04b28ab68ffd0e6e17fac6ec16d4846a2004`](https://uniscan.xyz/address/0x0dfa04b28ab68ffd0e6e17fac6ec16d4846a2004)| [`0x2B6221E68D48cDC10CF2e52D913f9380D62555BA`](https://sepolia.uniscan.xyz/address/0x2B6221E68D48cDC10CF2e52D913f9380D62555BA)  
[V3Migrator](https://github.com/Uniswap/uniswap-v3-periphery/blob/v1.0.0/contracts/V3Migrator.sol)| [`0xb9d0c246f306b1aaf02ae6ba112d5ef25e5b60dc`](https://uniscan.xyz/address/0xb9d0c246f306b1aaf02ae6ba112d5ef25e5b60dc)| [`0xb5FA244C9d6D04B2FBac84418b3c4910ED1Ae5f2`](https://sepolia.uniscan.xyz/address/0xb5FA244C9d6D04B2FBac84418b3c4910ED1Ae5f2)  
[UniswapV3Staker](https://github.com/Uniswap/v3-staker)| -| [`0xd693d8df6CF768248c16f98745Ee0c8E06460487`](https://sepolia.uniscan.xyz/address/0xd693d8df6CF768248c16f98745Ee0c8E06460487)  
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
The Uniswap Protocol supports trading of ERC20 tokens. In order to swap a native asset like ETH, the Uniswap protocol wraps these assets in an ERC20 wrapped native token contract.
Network| ChainId| Wrapped Native Token| Address  
---|---|---|---  
Unichain| `130`| WETH| [`0x4200000000000000000000000000000000000006`](https://unichain.blockscout.com/token/0x4200000000000000000000000000000000000006)  
Unichain Sepolia| `1301`| WETH| [`0x4200000000000000000000000000000000000006`](https://unichain-sepolia.blockscout.com/token/0x4200000000000000000000000000000000000006)  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/deployments/Unichain-Deployments.md)
Was this helpful?
[PreviousPolygon Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/polygon-deployments)[NextWorldChain Deployments](https://docs.uniswap.org/contracts/v3/reference/deployments/WorldChain-deployments)
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/reference/deployments/Unichain-Deployments.md)
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
