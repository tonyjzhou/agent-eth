# https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0

[Skip to main content](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#__docusaurus_skipToContent_fallback)
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
            * [BalanceDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BalanceDelta)
            * [BeforeSwapDelta](https://docs.uniswap.org/contracts/v4/reference/core/types/BeforeSwapDelta)
            * [Currency](https://docs.uniswap.org/contracts/v4/reference/core/types/Currency)
            * [PoolId](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolId)
            * [PoolKey](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)
            * [Slot0](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0)
            * [BalanceDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
            * [BeforeSwapDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/beforeswapdelta-guide)
            * [Currency Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/currency-guide)
            * [PoolKey Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/poolkey-guide)
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
  * Core
  * Types
  * [Slot0](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0)


On this page
# Slot0
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Slot0.sol) - Generated with [forge doc](https://book.getfoundry.sh/reference/forge/forge-doc)
_Slot0 is a packed version of solidity structure. Using the packaged version saves gas by not storing the structure fields in memory slots. Layout: 24 bits empty | 24 bits lpFee | 12 bits protocolFee 1- >0 | 12 bits protocolFee 0->1 | 24 bits tick | 160 bits sqrtPriceX96 Fields in the direction from the least significant bit: The current price uint160 sqrtPriceX96; The current tick int24 tick; Protocol fee, expressed in hundredths of a bip, upper 12 bits are for 1->0, and the lower 12 are for 0->1 the maximum is 1000 - meaning the maximum protocol fee is 0.1% the protocolFee is taken from the input first, then the lpFee is taken from the remaining input uint24 protocolFee; The current LP fee of the pool. If the pool is dynamic, this does not include the dynamic fee flag. uint24 lpFee;_
```
type Slot0 isbytes32;
```

# Slot0Library
[Git Source](https://github.com/uniswap/v4-core/blob/80311e34080fee64b6fc6c916e9a51a437d0e482/src/types/Slot0.sol)
Library for getting and setting values in the Slot0 type
## State Variables[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#state-variables "Direct link to State Variables")
### MASK_160_BITS[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#mask_160_bits "Direct link to MASK_160_BITS")
```
uint160internalconstant MASK_160_BITS =0x00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF;
```

### MASK_24_BITS[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#mask_24_bits "Direct link to MASK_24_BITS")
```
uint24internalconstant MASK_24_BITS =0xFFFFFF;
```

### TICK_OFFSET[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#tick_offset "Direct link to TICK_OFFSET")
```
uint8internalconstant TICK_OFFSET =160;
```

### PROTOCOL_FEE_OFFSET[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#protocol_fee_offset "Direct link to PROTOCOL_FEE_OFFSET")
```
uint8internalconstant PROTOCOL_FEE_OFFSET =184;
```

### LP_FEE_OFFSET[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#lp_fee_offset "Direct link to LP_FEE_OFFSET")
```
uint8internalconstant LP_FEE_OFFSET =208;
```

## Functions[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#functions "Direct link to Functions")
### sqrtPriceX96[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#sqrtpricex96 "Direct link to sqrtPriceX96")
```
functionsqrtPriceX96(Slot0 _packed)internalpurereturns(uint160 _sqrtPriceX96);
```

### tick[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#tick "Direct link to tick")
```
functiontick(Slot0 _packed)internalpurereturns(int24 _tick);
```

### protocolFee[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#protocolfee "Direct link to protocolFee")
```
functionprotocolFee(Slot0 _packed)internalpurereturns(uint24 _protocolFee);
```

### lpFee[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#lpfee "Direct link to lpFee")
```
functionlpFee(Slot0 _packed)internalpurereturns(uint24 _lpFee);
```

### setSqrtPriceX96[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#setsqrtpricex96 "Direct link to setSqrtPriceX96")
```
functionsetSqrtPriceX96(Slot0 _packed,uint160 _sqrtPriceX96)internalpurereturns(Slot0 _result);
```

### setTick[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#settick "Direct link to setTick")
```
functionsetTick(Slot0 _packed,int24 _tick)internalpurereturns(Slot0 _result);
```

### setProtocolFee[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#setprotocolfee "Direct link to setProtocolFee")
```
functionsetProtocolFee(Slot0 _packed,uint24 _protocolFee)internalpurereturns(Slot0 _result);
```

### setLpFee[​](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#setlpfee "Direct link to setLpFee")
```
functionsetLpFee(Slot0 _packed,uint24 _lpFee)internalpurereturns(Slot0 _result);
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/Slot0.md)
Was this helpful?
[PreviousPoolKey](https://docs.uniswap.org/contracts/v4/reference/core/types/PoolKey)[NextBalanceDelta Guide](https://docs.uniswap.org/contracts/v4/reference/core/types/balancedelta-guide)
On this page
  * [State Variables](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#state-variables)
    * [MASK_160_BITS](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#mask_160_bits)
    * [MASK_24_BITS](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#mask_24_bits)
    * [TICK_OFFSET](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#tick_offset)
    * [PROTOCOL_FEE_OFFSET](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#protocol_fee_offset)
    * [LP_FEE_OFFSET](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#lp_fee_offset)
  * [Functions](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#functions)
    * [sqrtPriceX96](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#sqrtpricex96)
    * [tick](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#tick)
    * [protocolFee](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#protocolfee)
    * [lpFee](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#lpfee)
    * [setSqrtPriceX96](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#setsqrtpricex96)
    * [setTick](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#settick)
    * [setProtocolFee](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#setprotocolfee)
    * [setLpFee](https://docs.uniswap.org/contracts/v4/reference/core/types/Slot0#setlpfee)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/reference/core/types/Slot0.md)
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
