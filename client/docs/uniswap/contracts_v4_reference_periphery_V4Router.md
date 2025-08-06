# https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [test](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [PositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [UniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)
          * [V4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [base](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [lens](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [utils](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [Permit2](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Technical Reference
  * Periphery
  * [V4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)


On this page
# V4Router
[Git Source](https://github.com/uniswap/v4-periphery/blob/ea2bf2e1ba6863bb809fc2ff791744f308c4a26d/src/V4Router.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IV4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IV4Router), [BaseActionsRouter](https://docs.uniswap.org/contracts/v4/reference/periphery/base/BaseActionsRouter), [DeltaResolver](https://docs.uniswap.org/contracts/v4/reference/periphery/base/DeltaResolver)
Abstract contract that contains all internal logic needed for routing through Uniswap v4 pools
_the entry point to executing actions in this contract is calling`BaseActionsRouter._executeActions` An inheriting contract should call _executeActions at the point that they wish actions to be executed_
## Functions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#constructor "Direct link to constructor")
```
constructor(IPoolManager _poolManager)BaseActionsRouter(_poolManager);
```

### _handleAction[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_handleaction "Direct link to _handleAction")
```
function_handleAction(uint256 action,bytescalldata params)internal override;
```

### _swapExactInputSingle[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactinputsingle "Direct link to _swapExactInputSingle")
```
function_swapExactInputSingle(IV4Router.ExactInputSingleParams calldata params)private;
```

### _swapExactInput[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactinput "Direct link to _swapExactInput")
```
function_swapExactInput(IV4Router.ExactInputParams calldata params)private;
```

### _swapExactOutputSingle[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactoutputsingle "Direct link to _swapExactOutputSingle")
```
function_swapExactOutputSingle(IV4Router.ExactOutputSingleParams calldata params)private;
```

### _swapExactOutput[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactoutput "Direct link to _swapExactOutput")
```
function_swapExactOutput(IV4Router.ExactOutputParams calldata params)private;
```

### _swap[​](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swap "Direct link to _swap")
```
function_swap(PoolKey memory poolKey,bool zeroForOne,int256 amountSpecified,bytescalldata hookData)privatereturns(int128 reciprocalAmount);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/V4Router.md)
Was this helpful?
[PreviousUniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)[NextBaseActionsRouter](https://docs.uniswap.org/contracts/v4/reference/periphery/base/BaseActionsRouter)
On this page
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#constructor)
    * [_handleAction](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_handleaction)
    * [_swapExactInputSingle](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactinputsingle)
    * [_swapExactInput](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactinput)
    * [_swapExactOutputSingle](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactoutputsingle)
    * [_swapExactOutput](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swapexactoutput)
    * [_swap](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router#_swap)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/V4Router.md)
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
