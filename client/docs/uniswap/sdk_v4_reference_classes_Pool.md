# https://docs.uniswap.org/sdk/v4/reference/classes/Pool

[Skip to main content](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
        * [Swaps](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
        * [Position Management](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
        * [Advanced](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
      * [Technical Reference](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
        * [Overview](https://docs.uniswap.org/sdk/v4/reference/overview)
        * [classes](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Hook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)
          * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
          * [Position](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
          * [Route](https://docs.uniswap.org/sdk/v4/reference/classes/Route)
          * [Trade](https://docs.uniswap.org/sdk/v4/reference/classes/Trade)
          * [V4BaseActionsParser](https://docs.uniswap.org/sdk/v4/reference/classes/V4BaseActionsParser)
          * [V4Planner](https://docs.uniswap.org/sdk/v4/reference/classes/V4Planner)
          * [V4PositionManager](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionManager)
          * [V4PositionPlanner](https://docs.uniswap.org/sdk/v4/reference/classes/V4PositionPlanner)
        * [enumerations](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
        * [interfaces](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
    * [v3 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
    * [Swap Widget](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
    * [web3-react](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
    * [Core SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
    * [v2 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
    * [v1 SDK](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v4 SDK
  * Technical Reference
  * classes
  * [Pool](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)


On this page
# Pool
[@uniswap/v4-sdk](https://docs.uniswap.org/sdk/v4/reference/overview) / Pool
Defined in: [entities/pool.ts:33](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L33)
Represents a V4 pool
## Constructors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#constructors "Direct link to Constructors")
### new Pool()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#new-pool "Direct link to new Pool\(\)")
> **new Pool**(`currencyA`, `currencyB`, `fee`, `tickSpacing`, `hooks`, `sqrtRatioX96`, `liquidity`, `tickCurrent`, `ticks`): [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
Defined in: [entities/pool.ts:103](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L103)
Construct a pool
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters "Direct link to Parameters")
Parameter| Type| Default value| Description  
---|---|---|---  
`currencyA`| `Currency`| `undefined`| One of the currencys in the pool  
`currencyB`| `Currency`| `undefined`| The other currency in the pool  
`fee`| `number`| `undefined`| The fee in hundredths of a bips of the input amount of every swap that is collected by the pool  
`tickSpacing`| `number`| `undefined`| The tickSpacing of the pool  
`hooks`| `string`| `undefined`| The address of the hook contract  
`sqrtRatioX96`| `BigintIsh`| `undefined`| The sqrt of the current ratio of amounts of currency1 to currency0  
`liquidity`| `BigintIsh`| `undefined`| The current value of in range liquidity  
`tickCurrent`| `number`| `undefined`| The current tick of the pool  
`ticks`| `TickDataProvider`| (`Tick`| `TickConstructorArgs`)[]  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns "Direct link to Returns")
[`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)
## Properties[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#properties "Direct link to Properties")
### currency0[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency0 "Direct link to currency0")
> `readonly` **currency0** : `Currency`
Defined in: [entities/pool.ts:34](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L34)
### currency1[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency1 "Direct link to currency1")
> `readonly` **currency1** : `Currency`
Defined in: [entities/pool.ts:35](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L35)
### fee[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#fee "Direct link to fee")
> `readonly` **fee** : `number`
Defined in: [entities/pool.ts:36](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L36)
### hooks[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#hooks "Direct link to hooks")
> `readonly` **hooks** : `string`
Defined in: [entities/pool.ts:39](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L39)
### liquidity[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#liquidity "Direct link to liquidity")
> `readonly` **liquidity** : `JSBI`
Defined in: [entities/pool.ts:40](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L40)
### poolId[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#poolid "Direct link to poolId")
> `readonly` **poolId** : `string`
Defined in: [entities/pool.ts:44](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L44)
### poolKey[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#poolkey "Direct link to poolKey")
> `readonly` **poolKey** : [`PoolKey`](https://docs.uniswap.org/sdk/v4/reference/overview#poolkey)
Defined in: [entities/pool.ts:43](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L43)
### sqrtRatioX96[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#sqrtratiox96 "Direct link to sqrtRatioX96")
> `readonly` **sqrtRatioX96** : `JSBI`
Defined in: [entities/pool.ts:38](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L38)
### tickCurrent[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#tickcurrent "Direct link to tickCurrent")
> `readonly` **tickCurrent** : `number`
Defined in: [entities/pool.ts:41](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L41)
### tickDataProvider[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#tickdataprovider "Direct link to tickDataProvider")
> `readonly` **tickDataProvider** : `TickDataProvider`
Defined in: [entities/pool.ts:42](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L42)
### tickSpacing[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#tickspacing "Direct link to tickSpacing")
> `readonly` **tickSpacing** : `number`
Defined in: [entities/pool.ts:37](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L37)
## Accessors[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#accessors "Direct link to Accessors")
### chainId[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#chainid "Direct link to chainId")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature "Direct link to Get Signature")
> **get** **chainId**(): `number`
Defined in: [entities/pool.ts:214](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L214)
Returns the chain ID of the currencies in the pool.
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-1 "Direct link to Returns")
`number`
### currency0Price[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency0price "Direct link to currency0Price")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature-1 "Direct link to Get Signature")
> **get** **currency0Price**(): `Price`<`Currency`, `Currency`>
Defined in: [entities/pool.ts:166](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L166)
Returns the current mid price of the pool in terms of currency0, i.e. the ratio of currency1 over currency0
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-2 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
### currency1Price[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency1price "Direct link to currency1Price")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature-2 "Direct link to Get Signature")
> **get** **currency1Price**(): `Price`<`Currency`, `Currency`>
Defined in: [entities/pool.ts:185](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L185)
Returns the current mid price of the pool in terms of currency1, i.e. the ratio of currency0 over currency1
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-3 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
### token0[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token0 "Direct link to token0")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature-3 "Direct link to Get Signature")
> **get** **token0**(): `Currency`
Defined in: [entities/pool.ts:143](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L143)
backwards compatibility with v2/3 sdks
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-4 "Direct link to Returns")
`Currency`
### token0Price[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token0price "Direct link to token0Price")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature-4 "Direct link to Get Signature")
> **get** **token0Price**(): `Price`<`Currency`, `Currency`>
Defined in: [entities/pool.ts:178](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L178)
backwards compatibility with v2/3 sdks
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-5 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
### token1[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token1 "Direct link to token1")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature-5 "Direct link to Get Signature")
> **get** **token1**(): `Currency`
Defined in: [entities/pool.ts:146](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L146)
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-6 "Direct link to Returns")
`Currency`
### token1Price[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token1price "Direct link to token1Price")
#### Get Signature[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#get-signature-6 "Direct link to Get Signature")
> **get** **token1Price**(): `Price`<`Currency`, `Currency`>
Defined in: [entities/pool.ts:197](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L197)
backwards compatibility with v2/3 sdks
##### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-7 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
## Methods[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#methods "Direct link to Methods")
### getInputAmount()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getinputamount "Direct link to getInputAmount\(\)")
> **getInputAmount**(`outputAmount`, `sqrtPriceLimitX96`?): `Promise`<[`CurrencyAmount`<`Currency`>, [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)]>
Defined in: [entities/pool.ts:257](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L257)
Given a desired output amount of a currency, return the computed input amount and a pool with state updated after the trade Works only for vanilla hookless v3 pools, otherwise throws an error
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-1 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`outputAmount`| `CurrencyAmount`<`Currency`>| the output amount for which to quote the input amount  
`sqrtPriceLimitX96`?| `JSBI`| The Q64.96 sqrt price limit. If zero for one, the price cannot be less than this value after the swap. If one for zero, the price cannot be greater than this value after the swap  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-8 "Direct link to Returns")
`Promise`<[`CurrencyAmount`<`Currency`>, [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)]>
The input amount and the pool with updated state
### getOutputAmount()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getoutputamount "Direct link to getOutputAmount\(\)")
> **getOutputAmount**(`inputAmount`, `sqrtPriceLimitX96`?): `Promise`<[`CurrencyAmount`<`Currency`>, [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)]>
Defined in: [entities/pool.ts:219](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L219)
Works only for vanilla hookless v3 pools, otherwise throws an error
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-2 "Direct link to Parameters")
Parameter| Type  
---|---  
`inputAmount`| `CurrencyAmount`<`Currency`>  
`sqrtPriceLimitX96`?| `JSBI`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-9 "Direct link to Returns")
`Promise`<[`CurrencyAmount`<`Currency`>, [`Pool`](https://docs.uniswap.org/sdk/v4/reference/classes/Pool)]>
### getPoolId()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getpoolid "Direct link to getPoolId\(\)")
> `static` **getPoolId**(`currencyA`, `currencyB`, `fee`, `tickSpacing`, `hooks`): `string`
Defined in: [entities/pool.ts:71](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L71)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-3 "Direct link to Parameters")
Parameter| Type  
---|---  
`currencyA`| `Currency`  
`currencyB`| `Currency`  
`fee`| `number`  
`tickSpacing`| `number`  
`hooks`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-10 "Direct link to Returns")
`string`
### getPoolKey()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getpoolkey "Direct link to getPoolKey\(\)")
> `static` **getPoolKey**(`currencyA`, `currencyB`, `fee`, `tickSpacing`, `hooks`): [`PoolKey`](https://docs.uniswap.org/sdk/v4/reference/overview#poolkey)
Defined in: [entities/pool.ts:49](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L49)
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-4 "Direct link to Parameters")
Parameter| Type  
---|---  
`currencyA`| `Currency`  
`currencyB`| `Currency`  
`fee`| `number`  
`tickSpacing`| `number`  
`hooks`| `string`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-11 "Direct link to Returns")
[`PoolKey`](https://docs.uniswap.org/sdk/v4/reference/overview#poolkey)
### involvesCurrency()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#involvescurrency "Direct link to involvesCurrency\(\)")
> **involvesCurrency**(`currency`): `boolean`
Defined in: [entities/pool.ts:155](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L155)
Returns true if the currency is either currency0 or currency1
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-5 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`currency`| `Currency`| The currency to check  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-12 "Direct link to Returns")
`boolean`
True if currency is either currency0 or currency1
### involvesToken()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#involvestoken "Direct link to involvesToken\(\)")
> **involvesToken**(`currency`): `boolean`
Defined in: [entities/pool.ts:159](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L159)
backwards compatibility with v2/3 sdks
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-6 "Direct link to Parameters")
Parameter| Type  
---|---  
`currency`| `Currency`  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-13 "Direct link to Returns")
`boolean`
### priceOf()[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#priceof "Direct link to priceOf\(\)")
> **priceOf**(`currency`): `Price`<`Currency`, `Currency`>
Defined in: [entities/pool.ts:206](https://github.com/Uniswap/sdks/blob/9cf6edb2df79338ae58f7ea7ca979c35a8a9bd56/sdks/v4-sdk/src/entities/pool.ts#L206)
Return the price of the given currency in terms of the other currency in the pool.
#### Parameters[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#parameters-7 "Direct link to Parameters")
Parameter| Type| Description  
---|---|---  
`currency`| `Currency`| The currency to return price of  
#### Returns[​](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#returns-14 "Direct link to Returns")
`Price`<`Currency`, `Currency`>
The price of the given currency, in terms of the other.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Pool.md)
Was this helpful?
[PreviousHook](https://docs.uniswap.org/sdk/v4/reference/classes/Hook)[NextPosition](https://docs.uniswap.org/sdk/v4/reference/classes/Position)
On this page
  * [Constructors](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#constructors)
    * [new Pool()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#new-pool)
  * [Properties](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#properties)
    * [currency0](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency0)
    * [currency1](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency1)
    * [fee](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#fee)
    * [hooks](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#hooks)
    * [liquidity](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#liquidity)
    * [poolId](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#poolid)
    * [poolKey](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#poolkey)
    * [sqrtRatioX96](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#sqrtratiox96)
    * [tickCurrent](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#tickcurrent)
    * [tickDataProvider](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#tickdataprovider)
    * [tickSpacing](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#tickspacing)
  * [Accessors](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#accessors)
    * [chainId](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#chainid)
    * [currency0Price](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency0price)
    * [currency1Price](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#currency1price)
    * [token0](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token0)
    * [token0Price](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token0price)
    * [token1](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token1)
    * [token1Price](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#token1price)
  * [Methods](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#methods)
    * [getInputAmount()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getinputamount)
    * [getOutputAmount()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getoutputamount)
    * [getPoolId()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getpoolid)
    * [getPoolKey()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#getpoolkey)
    * [involvesCurrency()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#involvescurrency)
    * [involvesToken()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#involvestoken)
    * [priceOf()](https://docs.uniswap.org/sdk/v4/reference/classes/Pool#priceof)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v4/reference/classes/Pool.md)
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
