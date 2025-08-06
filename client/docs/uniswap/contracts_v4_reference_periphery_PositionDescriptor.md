# https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#__docusaurus_skipToContent_fallback)
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
        * [Errors](https://docs.uniswap.org/contracts/v4/reference/errors/)
        * [Core](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Libraries](https://docs.uniswap.org/contracts/v4/reference/core/libraries/BitMath)
          * [Types](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
          * [ERC6909](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909)
          * [ERC6909Claims](https://docs.uniswap.org/contracts/v4/reference/core/ERC6909Claims)
          * [Extsload](https://docs.uniswap.org/contracts/v4/reference/core/Extsload)
          * [Exttload](https://docs.uniswap.org/contracts/v4/reference/core/Exttload)
          * [IPoolManager](https://docs.uniswap.org/contracts/v4/reference/core/IPoolManager)
          * [NoDelegateCall](https://docs.uniswap.org/contracts/v4/reference/core/NoDelegateCall)
          * [PoolManager](https://docs.uniswap.org/contracts/v4/reference/core/PoolManager)
          * [ProtocolFees](https://docs.uniswap.org/contracts/v4/reference/core/ProtocolFees)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/core/interfaces/IERC20Minimal)
          * [test](https://docs.uniswap.org/contracts/v4/reference/core/test/ActionsRouter)
        * [Periphery](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)
          * [PositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
          * [UniswapV4DeployerCompetition](https://docs.uniswap.org/contracts/v4/reference/periphery/UniswapV4DeployerCompetition)
          * [V4Router](https://docs.uniswap.org/contracts/v4/reference/periphery/V4Router)
          * [base](https://docs.uniswap.org/contracts/v4/reference/periphery/base/BaseActionsRouter)
          * [interfaces](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IEIP712_v4)
          * [lens](https://docs.uniswap.org/contracts/v4/reference/periphery/lens/StateView)
          * [libraries](https://docs.uniswap.org/contracts/v4/reference/periphery/libraries/ActionConstants)
          * [utils](https://docs.uniswap.org/contracts/v4/reference/periphery/utils/BaseHook)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
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
  * v4 Protocol
  * Technical Reference
  * Periphery
  * [PositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor)


On this page
# PositionDescriptor
[Git Source](https://github.com/uniswap/v4-periphery/blob/ea2bf2e1ba6863bb809fc2ff791744f308c4a26d/src/PositionDescriptor.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
**Inherits:** [IPositionDescriptor](https://docs.uniswap.org/contracts/v4/reference/periphery/interfaces/IPositionDescriptor)
Produces a string containing the data URI for a JSON metadata string
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#state-variables "Direct link to State Variables")
### DAI[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#dai "Direct link to DAI")
```
addressprivateconstant DAI =0x6B175474E89094C44Da98b954EedeAC495271d0F;
```

### USDC[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#usdc "Direct link to USDC")
```
addressprivateconstant USDC =0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48;
```

### USDT[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#usdt "Direct link to USDT")
```
addressprivateconstant USDT =0xdAC17F958D2ee523a2206206994597C13D831ec7;
```

### TBTC[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#tbtc "Direct link to TBTC")
```
addressprivateconstant TBTC =0x8dAEBADE922dF735c38C80C7eBD708Af50815fAa;
```

### WBTC[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#wbtc "Direct link to WBTC")
```
addressprivateconstant WBTC =0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599;
```

### wrappedNative[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#wrappednative "Direct link to wrappedNative")
```
addresspublic immutable wrappedNative;
```

### nativeCurrencyLabelBytes[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#nativecurrencylabelbytes "Direct link to nativeCurrencyLabelBytes")
```
bytes32private immutable nativeCurrencyLabelBytes;
```

### poolManager[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#poolmanager "Direct link to poolManager")
```
IPoolManager public immutable poolManager;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#functions "Direct link to Functions")
### constructor[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#constructor "Direct link to constructor")
```
constructor(IPoolManager _poolManager,address _wrappedNative,bytes32 _nativeCurrencyLabelBytes);
```

### nativeCurrencyLabel[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#nativecurrencylabel "Direct link to nativeCurrencyLabel")
Returns the native currency label as a string
```
functionnativeCurrencyLabel()publicviewreturns(stringmemory);
```

### tokenURI[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#tokenuri "Direct link to tokenURI")
Produces the URI describing a particular token ID
_Note this URI may be a data: URI with the JSON contents directly inlined_
```
functiontokenURI(IPositionManager positionManager,uint256 tokenId)externalview override returns(stringmemory);
```

**Parameters**
Name| Type| Description  
---|---|---  
`positionManager`| `IPositionManager`| The position manager for which to describe the token  
`tokenId`| `uint256`| The ID of the token for which to produce a description, which may not be valid  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `string`| The URI of the ERC721-compliant metadata  
### flipRatio[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#flipratio "Direct link to flipRatio")
Returns true if currency0 has higher priority than currency1
```
functionflipRatio(address currency0,address currency1)publicviewreturns(bool);
```

**Parameters**
Name| Type| Description  
---|---|---  
`currency0`| `address`| The first currency address  
`currency1`| `address`| The second currency address  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `bool`| True if currency0 has higher priority than currency1  
### currencyRatioPriority[​](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#currencyratiopriority "Direct link to currencyRatioPriority")
Returns the priority of a currency. For certain currencies on mainnet, the smaller the currency, the higher the priority And those with the higher priority values (more positive values) will be in the numerator of the price ratio
```
functioncurrencyRatioPriority(address currency)publicviewreturns(int256);
```

**Parameters**
Name| Type| Description  
---|---|---  
`currency`| `address`| The currency address  
**Returns**
Name| Type| Description  
---|---|---  
`<none>`| `int256`| The priority of the currency  
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/PositionDescriptor.md)
Was this helpful?
[PreviousTickOverflowSafetyEchidnaTest](https://docs.uniswap.org/contracts/v4/reference/core/test/TickOverflowSafetyEchidnaTest)[NextPositionManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionManager)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#state-variables)
    * [DAI](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#dai)
    * [USDC](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#usdc)
    * [USDT](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#usdt)
    * [TBTC](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#tbtc)
    * [WBTC](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#wbtc)
    * [wrappedNative](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#wrappednative)
    * [nativeCurrencyLabelBytes](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#nativecurrencylabelbytes)
    * [poolManager](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#poolmanager)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#functions)
    * [constructor](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#constructor)
    * [nativeCurrencyLabel](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#nativecurrencylabel)
    * [tokenURI](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#tokenuri)
    * [flipRatio](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#flipratio)
    * [currencyRatioPriority](https://docs.uniswap.org/contracts/v4/reference/periphery/PositionDescriptor#currencyratiopriority)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/periphery/PositionDescriptor.md)
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
