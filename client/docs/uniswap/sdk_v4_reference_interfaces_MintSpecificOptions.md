# https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
        * [Swaps](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
        * [Position Management](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
        * [Advanced](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
          * [AllowanceTransferPermitBatch](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitBatch)
          * [AllowanceTransferPermitSingle](https://docs.uniswap.org/sdk/v4/reference/interfaces/AllowanceTransferPermitSingle)
          * [BatchPermitOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/BatchPermitOptions)
          * [BestTradeOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/BestTradeOptions)
          * [CollectSpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/CollectSpecificOptions)
          * [CommonAddLiquidityOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/CommonAddLiquidityOptions)
          * [CommonOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/CommonOptions)
          * [MethodParameters](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)
          * [MintSpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
          * [ModifyPositionSpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/ModifyPositionSpecificOptions)
          * [NFTPermitData](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitData)
          * [NFTPermitOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitOptions)
          * [NFTPermitValues](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitValues)
          * [PermitDetails](https://docs.uniswap.org/sdk/v4/reference/interfaces/PermitDetails)
          * [RemoveLiquiditySpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
          * [TransferOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/TransferOptions)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
    * [web3-react](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * interfaces
  * [MintSpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions)


On this page
# MintSpecificOptions
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / MintSpecificOptions
Defined in: [PositionManager.ts:47](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L47)
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#properties "Direct link to Properties")
### createPool?[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#createpool "Direct link to createPool?")
> `optional` **createPool** : `boolean`
Defined in: [PositionManager.ts:56](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L56)
Creates pool if not initialized before mint.
### migrate?[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#migrate "Direct link to migrate?")
> `optional` **migrate** : `boolean`
Defined in: [PositionManager.ts:66](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L66)
Whether the mint is part of a migration from V3 to V4.
### recipient[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#recipient "Direct link to recipient")
> **recipient** : `string`
Defined in: [PositionManager.ts:51](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L51)
The account that should receive the minted NFT.
### sqrtPriceX96?[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#sqrtpricex96 "Direct link to sqrtPriceX96?")
> `optional` **sqrtPriceX96** : `BigintIsh`
Defined in: [PositionManager.ts:61](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L61)
Initial price to set on the pool if creating
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/interfaces/MintSpecificOptions.md)
Was this helpful?
[PreviousMethodParameters](https://docs.uniswap.org/sdk/v4/reference/interfaces/MethodParameters)[NextModifyPositionSpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/ModifyPositionSpecificOptions)
On this page
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#properties)
    * [createPool?](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#createpool)
    * [migrate?](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#migrate)
    * [recipient](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#recipient)
    * [sqrtPriceX96?](https://docs.uniswap.org/sdk/v4/reference/interfaces/MintSpecificOptions#sqrtpricex96)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/interfaces/MintSpecificOptions.md)
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
