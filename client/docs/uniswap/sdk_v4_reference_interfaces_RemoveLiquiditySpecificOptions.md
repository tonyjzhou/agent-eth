# https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
        * [Swaps](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
        * [Position Management](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
        * [Advanced](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
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
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
    * [web3-react](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * interfaces
  * [RemoveLiquiditySpecificOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions)


On this page
# RemoveLiquiditySpecificOptions
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / RemoveLiquiditySpecificOptions
Defined in: [PositionManager.ts:87](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L87)
Options for producing the calldata to exit a position.
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#properties "Direct link to Properties")
### burnToken?[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#burntoken "Direct link to burnToken?")
> `optional` **burnToken** : `boolean`
Defined in: [PositionManager.ts:96](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L96)
Whether the NFT should be burned if the entire position is being exited, by default false.
### liquidityPercentage[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#liquiditypercentage "Direct link to liquidityPercentage")
> **liquidityPercentage** : `Percent`
Defined in: [PositionManager.ts:91](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L91)
The percentage of position liquidity to exit.
### permit?[​](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#permit "Direct link to permit?")
> `optional` **permit** : [`NFTPermitOptions`](https://docs.uniswap.org/sdk/v4/reference/interfaces/NFTPermitOptions)
Defined in: [PositionManager.ts:101](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/PositionManager.ts#L101)
The optional permit of the token ID being exited, in case the exit transaction is being sent by an account that does not own the NFT
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions.md)
Was this helpful?
[PreviousPermitDetails](https://docs.uniswap.org/sdk/v4/reference/interfaces/PermitDetails)[NextTransferOptions](https://docs.uniswap.org/sdk/v4/reference/interfaces/TransferOptions)
On this page
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#properties)
    * [burnToken?](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#burntoken)
    * [liquidityPercentage](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#liquiditypercentage)
    * [permit?](https://docs.uniswap.org/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions#permit)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/interfaces/RemoveLiquiditySpecificOptions.md)
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
