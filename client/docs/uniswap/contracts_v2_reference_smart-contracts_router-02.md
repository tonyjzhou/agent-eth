# https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02

[Skip to main content](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
    * [Permit2](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
      * [Guides](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [API](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Governance](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
          * [Factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory)
          * [Pair](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair)
          * [Pair (ERC-20)](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/Pair-ERC-20)
          * [Library](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library)
          * [Router01](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-01)
          * [Router02](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
          * [Common Errors](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/common-errors)
          * [V2 Deployment Addresses](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/v2-deployments)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Technical Reference
  * Smart Contracts
  * [Router02](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02)


On this page
Because routers are stateless and do not hold token balances, they can be replaced safely and trustlessly, if necessary. This may happen if more efficient smart contract patterns are discovered, or if additional functionality is desired. For this reason, routers have _release numbers_ , starting at `01`. This is currently recommended release, `02`.
# Code
[`UniswapV2Router02.sol`](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/UniswapV2Router02.sol)
# Address
`UniswapV2Router02` is deployed at `0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D` on the Ethereum [mainnet](https://etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D), and the [Ropsten](https://ropsten.etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D), [Rinkeby](https://rinkeby.etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D), [Görli](https://goerli.etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D), and [Kovan](https://kovan.etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D) testnets. It was built from commit [6961711](https://github.com/Uniswap/uniswap-v2-periphery/tree/69617118cda519dab608898d62aaa79877a61004).
# Read-Only Functions
## factory[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#factory "Direct link to factory")
```
functionfactory()externalpurereturns(address);
```

Returns [factory address](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/factory#address).
## WETH[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#weth "Direct link to WETH")
```
functionWETH()externalpurereturns(address);
```

Returns the [canonical WETH address](https://blog.0xproject.com/canonical-weth-a9aa7d0279dd) on the Ethereum [mainnet](https://etherscan.io/address/0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2), or the [Ropsten](https://ropsten.etherscan.io/address/0xc778417e063141139fce010982780140aa0cd5ab), [Rinkeby](https://rinkeby.etherscan.io/address/0xc778417e063141139fce010982780140aa0cd5ab), [Görli](https://goerli.etherscan.io/address/0xb4fbf271143f4fbf7b91a5ded31805e42b2208d6), or [Kovan](https://kovan.etherscan.io/address/0xd0a1e359811322d97991e03f863a0c30c2cf029c) testnets.
## quote[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#quote "Direct link to quote")
See [quote](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#quote).
## getAmountOut[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountout "Direct link to getAmountOut")
See [getAmountOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountout).
## getAmountIn[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountin "Direct link to getAmountIn")
See [getAmountIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountin).
## getAmountsOut[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountsout "Direct link to getAmountsOut")
```
functiongetAmountsOut(uint amountIn,address[]memory path)publicviewreturns(uint[]memory amounts);
```

See [getAmountsOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsout).
## getAmountsIn[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountsin "Direct link to getAmountsIn")
```
functiongetAmountsIn(uint amountOut,address[]memory path)publicviewreturns(uint[]memory amounts);
```

See [getAmountsIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library#getamountsin).
# State-Changing Functions
## addLiquidity[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#addliquidity "Direct link to addLiquidity")
```
functionaddLiquidity(address tokenA,address tokenB,uint amountADesired,uint amountBDesired,uint amountAMin,uint amountBMin,address to,uint deadline)externalreturns(uint amountA,uint amountB,uint liquidity);
```

Adds liquidity to an ERC-20⇄ERC-20 pool.
  * To cover all possible scenarios, `msg.sender` should have already given the router an allowance of at least amountADesired/amountBDesired on tokenA/tokenB.
  * Always adds assets at the ideal ratio, according to the price when the transaction is executed.
  * If a pool for the passed tokens does not exists, one is created automatically, and exactly amountADesired/amountBDesired tokens are added.

Name| Type|   
---|---|---  
tokenA| `address`| A pool token.  
tokenB| `address`| A pool token.  
amountADesired| `uint`| The amount of tokenA to add as liquidity if the B/A price is <= amountBDesired/amountADesired (A depreciates).  
amountBDesired| `uint`| The amount of tokenB to add as liquidity if the A/B price is <= amountADesired/amountBDesired (B depreciates).  
amountAMin| `uint`| Bounds the extent to which the B/A price can go up before the transaction reverts. Must be <= amountADesired.  
amountBMin| `uint`| Bounds the extent to which the A/B price can go up before the transaction reverts. Must be <= amountBDesired.  
to| `address`| Recipient of the liquidity tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amountA| `uint`| The amount of tokenA sent to the pool.  
amountB| `uint`| The amount of tokenB sent to the pool.  
liquidity| `uint`| The amount of liquidity tokens minted.  
## addLiquidityETH[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#addliquidityeth "Direct link to addLiquidityETH")
```
functionaddLiquidityETH(address token,uint amountTokenDesired,uint amountTokenMin,uint amountETHMin,address to,uint deadline)externalpayablereturns(uint amountToken,uint amountETH,uint liquidity);
```

Adds liquidity to an ERC-20⇄WETH pool with ETH.
  * To cover all possible scenarios, `msg.sender` should have already given the router an allowance of at least amountTokenDesired on token.
  * Always adds assets at the ideal ratio, according to the price when the transaction is executed.
  * `msg.value` is treated as a amountETHDesired.
  * Leftover ETH, if any, is returned to `msg.sender`.
  * If a pool for the passed token and WETH does not exists, one is created automatically, and exactly amountTokenDesired/`msg.value` tokens are added.

Name| Type|   
---|---|---  
token| `address`| A pool token.  
amountTokenDesired| `uint`| The amount of token to add as liquidity if the WETH/token price is <= `msg.value`/amountTokenDesired (token depreciates).  
`msg.value` (amountETHDesired)| `uint`| The amount of ETH to add as liquidity if the token/WETH price is <= amountTokenDesired/`msg.value` (WETH depreciates).  
amountTokenMin| `uint`| Bounds the extent to which the WETH/token price can go up before the transaction reverts. Must be <= amountTokenDesired.  
amountETHMin| `uint`| Bounds the extent to which the token/WETH price can go up before the transaction reverts. Must be <= `msg.value`.  
to| `address`| Recipient of the liquidity tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amountToken| `uint`| The amount of token sent to the pool.  
amountETH| `uint`| The amount of ETH converted to WETH and sent to the pool.  
liquidity| `uint`| The amount of liquidity tokens minted.  
## removeLiquidity[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidity "Direct link to removeLiquidity")
```
functionremoveLiquidity(address tokenA,address tokenB,uint liquidity,uint amountAMin,uint amountBMin,address to,uint deadline)externalreturns(uint amountA,uint amountB);
```

Removes liquidity from an ERC-20⇄ERC-20 pool.
  * `msg.sender` should have already given the router an allowance of at least liquidity on the pool.

Name| Type|   
---|---|---  
tokenA| `address`| A pool token.  
tokenB| `address`| A pool token.  
liquidity| `uint`| The amount of liquidity tokens to remove.  
amountAMin| `uint`| The minimum amount of tokenA that must be received for the transaction not to revert.  
amountBMin| `uint`| The minimum amount of tokenB that must be received for the transaction not to revert.  
to| `address`| Recipient of the underlying assets.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amountA| `uint`| The amount of tokenA received.  
amountB| `uint`| The amount of tokenB received.  
## removeLiquidityETH[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityeth "Direct link to removeLiquidityETH")
```
functionremoveLiquidityETH(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline)externalreturns(uint amountToken,uint amountETH);
```

Removes liquidity from an ERC-20⇄WETH pool and receive ETH.
  * `msg.sender` should have already given the router an allowance of at least liquidity on the pool.

Name| Type|   
---|---|---  
token| `address`| A pool token.  
liquidity| `uint`| The amount of liquidity tokens to remove.  
amountTokenMin| `uint`| The minimum amount of token that must be received for the transaction not to revert.  
amountETHMin| `uint`| The minimum amount of ETH that must be received for the transaction not to revert.  
to| `address`| Recipient of the underlying assets.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amountToken| `uint`| The amount of token received.  
amountETH| `uint`| The amount of ETH received.  
## removeLiquidityWithPermit[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquiditywithpermit "Direct link to removeLiquidityWithPermit")
```
functionremoveLiquidityWithPermit(address tokenA,address tokenB,uint liquidity,uint amountAMin,uint amountBMin,address to,uint deadline,bool approveMax,uint8 v,bytes32 r,bytes32 s)externalreturns(uint amountA,uint amountB);
```

Removes liquidity from an ERC-20⇄ERC-20 pool without pre-approval, thanks to [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20#permit).
Name| Type|   
---|---|---  
tokenA| `address`| A pool token.  
tokenB| `address`| A pool token.  
liquidity| `uint`| The amount of liquidity tokens to remove.  
amountAMin| `uint`| The minimum amount of tokenA that must be received for the transaction not to revert.  
amountBMin| `uint`| The minimum amount of tokenB that must be received for the transaction not to revert.  
to| `address`| Recipient of the underlying assets.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
approveMax| `bool`| Whether or not the approval amount in the signature is for liquidity or `uint(-1)`.  
v| `uint8`| The v component of the permit signature.  
r| `bytes32`| The r component of the permit signature.  
s| `bytes32`| The s component of the permit signature.  
| |   
amountA| `uint`| The amount of tokenA received.  
amountB| `uint`| The amount of tokenB received.  
## removeLiquidityETHWithPermit[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethwithpermit "Direct link to removeLiquidityETHWithPermit")
```
functionremoveLiquidityETHWithPermit(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline,bool approveMax,uint8 v,bytes32 r,bytes32 s)externalreturns(uint amountToken,uint amountETH);
```

Removes liquidity from an ERC-20⇄WETTH pool and receive ETH without pre-approval, thanks to [permit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/pair-erc-20#permit).
Name| Type|   
---|---|---  
token| `address`| A pool token.  
liquidity| `uint`| The amount of liquidity tokens to remove.  
amountTokenMin| `uint`| The minimum amount of token that must be received for the transaction not to revert.  
amountETHMin| `uint`| The minimum amount of ETH that must be received for the transaction not to revert.  
to| `address`| Recipient of the underlying assets.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
approveMax| `bool`| Whether or not the approval amount in the signature is for liquidity or `uint(-1)`.  
v| `uint8`| The v component of the permit signature.  
r| `bytes32`| The r component of the permit signature.  
s| `bytes32`| The s component of the permit signature.  
| |   
amountToken| `uint`| The amount of token received.  
amountETH| `uint`| The amount of ETH received.  
## removeLiquidityETHSupportingFeeOnTransferTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethsupportingfeeontransfertokens "Direct link to removeLiquidityETHSupportingFeeOnTransferTokens")
```
functionremoveLiquidityETHSupportingFeeOnTransferTokens(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline)externalreturns(uint amountETH);
```

Identical to [removeLiquidityETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityeth), but succeeds for tokens that take a fee on transfer.
  * `msg.sender` should have already given the router an allowance of at least liquidity on the pool.

Name| Type|   
---|---|---  
token| `address`| A pool token.  
liquidity| `uint`| The amount of liquidity tokens to remove.  
amountTokenMin| `uint`| The minimum amount of token that must be received for the transaction not to revert.  
amountETHMin| `uint`| The minimum amount of ETH that must be received for the transaction not to revert.  
to| `address`| Recipient of the underlying assets.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amountETH| `uint`| The amount of ETH received.  
## removeLiquidityETHWithPermitSupportingFeeOnTransferTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethwithpermitsupportingfeeontransfertokens "Direct link to removeLiquidityETHWithPermitSupportingFeeOnTransferTokens")
```
functionremoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline,bool approveMax,uint8 v,bytes32 r,bytes32 s)externalreturns(uint amountETH);
```

Identical to [removeLiquidityETHWithPermit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethwithpermit), but succeeds for tokens that take a fee on transfer.
Name| Type|   
---|---|---  
token| `address`| A pool token.  
liquidity| `uint`| The amount of liquidity tokens to remove.  
amountTokenMin| `uint`| The minimum amount of token that must be received for the transaction not to revert.  
amountETHMin| `uint`| The minimum amount of ETH that must be received for the transaction not to revert.  
to| `address`| Recipient of the underlying assets.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
approveMax| `bool`| Whether or not the approval amount in the signature is for liquidity or `uint(-1)`.  
v| `uint8`| The v component of the permit signature.  
r| `bytes32`| The r component of the permit signature.  
s| `bytes32`| The s component of the permit signature.  
| |   
amountETH| `uint`| The amount of ETH received.  
## swapExactTokensForTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensfortokens "Direct link to swapExactTokensForTokens")
```
functionswapExactTokensForTokens(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);
```

Swaps an exact amount of input tokens for as many output tokens as possible, along the route determined by the path. The first element of path is the input token, the last is the output token, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  * `msg.sender` should have already given the router an allowance of at least amountIn on the input token.

Name| Type|   
---|---|---  
amountIn| `uint`| The amount of input tokens to send.  
amountOutMin| `uint`| The minimum amount of output tokens that must be received for the transaction not to revert.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the output tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amounts| `uint[] memory`| The input token amount and all subsequent output token amounts.  
## swapTokensForExactTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swaptokensforexacttokens "Direct link to swapTokensForExactTokens")
```
functionswapTokensForExactTokens(uint amountOut,uint amountInMax,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);
```

Receive an exact amount of output tokens for as few input tokens as possible, along the route determined by the path. The first element of path is the input token, the last is the output token, and any intermediate elements represent intermediate tokens to trade through (if, for example, a direct pair does not exist).
  * `msg.sender` should have already given the router an allowance of at least amountInMax on the input token.

Name| Type|   
---|---|---  
amountOut| `uint`| The amount of output tokens to receive.  
amountInMax| `uint`| The maximum amount of input tokens that can be required before the transaction reverts.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the output tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amounts| `uint[] memory`| The input token amount and all subsequent output token amounts.  
## swapExactETHForTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexactethfortokens "Direct link to swapExactETHForTokens")
```
functionswapExactETHForTokens(uint amountOutMin,address[]calldata path,address to,uint deadline)externalpayablereturns(uint[]memory amounts);
```

Swaps an exact amount of ETH for as many output tokens as possible, along the route determined by the path. The first element of path must be [WETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#weth), the last is the output token, and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
Name| Type|   
---|---|---  
`msg.value` (amountIn)| `uint`| The amount of ETH to send.  
amountOutMin| `uint`| The minimum amount of output tokens that must be received for the transaction not to revert.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the output tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amounts| `uint[] memory`| The input token amount and all subsequent output token amounts.  
## swapTokensForExactETH[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swaptokensforexacteth "Direct link to swapTokensForExactETH")
```
functionswapTokensForExactETH(uint amountOut,uint amountInMax,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);
```

Receive an exact amount of ETH for as few input tokens as possible, along the route determined by the path. The first element of path is the input token, the last must be [WETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#weth), and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  * `msg.sender` should have already given the router an allowance of at least amountInMax on the input token.
  * If the to address is a smart contract, it must have the ability to receive ETH.

Name| Type|   
---|---|---  
amountOut| `uint`| The amount of ETH to receive.  
amountInMax| `uint`| The maximum amount of input tokens that can be required before the transaction reverts.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of ETH.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amounts| `uint[] memory`| The input token amount and all subsequent output token amounts.  
## swapExactTokensForETH[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensforeth "Direct link to swapExactTokensForETH")
```
functionswapExactTokensForETH(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);
```

Swaps an exact amount of tokens for as much ETH as possible, along the route determined by the path. The first element of path is the input token, the last must be [WETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#weth), and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  * If the to address is a smart contract, it must have the ability to receive ETH.

Name| Type|   
---|---|---  
amountIn| `uint`| The amount of input tokens to send.  
amountOutMin| `uint`| The minimum amount of output tokens that must be received for the transaction not to revert.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the ETH.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amounts| `uint[] memory`| The input token amount and all subsequent output token amounts.  
## swapETHForExactTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapethforexacttokens "Direct link to swapETHForExactTokens")
```
functionswapETHForExactTokens(uint amountOut,address[]calldata path,address to,uint deadline)externalpayablereturns(uint[]memory amounts);
```

Receive an exact amount of tokens for as little ETH as possible, along the route determined by the path. The first element of path must be [WETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#weth), the last is the output token and any intermediate elements represent intermediate pairs to trade through (if, for example, a direct pair does not exist).
  * Leftover ETH, if any, is returned to `msg.sender`.

Name| Type|   
---|---|---  
amountOut| `uint`| The amount of tokens to receive.  
`msg.value` (amountInMax)| `uint`| The maximum amount of ETH that can be required before the transaction reverts.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the output tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
| |   
amounts| `uint[] memory`| The input token amount and all subsequent output token amounts.  
## swapExactTokensForTokensSupportingFeeOnTransferTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensfortokenssupportingfeeontransfertokens "Direct link to swapExactTokensForTokensSupportingFeeOnTransferTokens")
```
functionswapExactTokensForTokensSupportingFeeOnTransferTokens(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)external;
```

Identical to [swapExactTokensForTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensfortokens), but succeeds for tokens that take a fee on transfer.
  * `msg.sender` should have already given the router an allowance of at least amountIn on the input token.

Name| Type|   
---|---|---  
amountIn| `uint`| The amount of input tokens to send.  
amountOutMin| `uint`| The minimum amount of output tokens that must be received for the transaction not to revert.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the output tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
## swapExactETHForTokensSupportingFeeOnTransferTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexactethfortokenssupportingfeeontransfertokens "Direct link to swapExactETHForTokensSupportingFeeOnTransferTokens")
```
functionswapExactETHForTokensSupportingFeeOnTransferTokens(uint amountOutMin,address[]calldata path,address to,uint deadline)externalpayable;
```

Identical to [swapExactETHForTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexactethfortokens), but succeeds for tokens that take a fee on transfer.
Name| Type|   
---|---|---  
`msg.value` (amountIn)| `uint`| The amount of ETH to send.  
amountOutMin| `uint`| The minimum amount of output tokens that must be received for the transaction not to revert.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the output tokens.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
## swapExactTokensForETHSupportingFeeOnTransferTokens[​](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensforethsupportingfeeontransfertokens "Direct link to swapExactTokensForETHSupportingFeeOnTransferTokens")
```
functionswapExactTokensForETHSupportingFeeOnTransferTokens(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)external;
```

Identical to [swapExactTokensForETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensforeth), but succeeds for tokens that take a fee on transfer.
  * If the to address is a smart contract, it must have the ability to receive ETH.

Name| Type|   
---|---|---  
amountIn| `uint`| The amount of input tokens to send.  
amountOutMin| `uint`| The minimum amount of output tokens that must be received for the transaction not to revert.  
path| `address[] calldata`| An array of token addresses. `path.length` must be >= 2. Pools for each consecutive pair of addresses must exist and have liquidity.  
to| `address`| Recipient of the ETH.  
deadline| `uint`| Unix timestamp after which the transaction will revert.  
# Interface
```
import'@uniswap/v2-periphery/contracts/interfaces/IUniswapV2Router02.sol';
```

```
pragmasolidity>=0.6.2;interfaceIUniswapV2Router01{functionfactory()externalpurereturns(address);functionWETH()externalpurereturns(address);functionaddLiquidity(address tokenA,address tokenB,uint amountADesired,uint amountBDesired,uint amountAMin,uint amountBMin,address to,uint deadline)externalreturns(uint amountA,uint amountB,uint liquidity);functionaddLiquidityETH(address token,uint amountTokenDesired,uint amountTokenMin,uint amountETHMin,address to,uint deadline)externalpayablereturns(uint amountToken,uint amountETH,uint liquidity);functionremoveLiquidity(address tokenA,address tokenB,uint liquidity,uint amountAMin,uint amountBMin,address to,uint deadline)externalreturns(uint amountA,uint amountB);functionremoveLiquidityETH(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline)externalreturns(uint amountToken,uint amountETH);functionremoveLiquidityWithPermit(address tokenA,address tokenB,uint liquidity,uint amountAMin,uint amountBMin,address to,uint deadline,bool approveMax,uint8 v,bytes32 r,bytes32 s)externalreturns(uint amountA,uint amountB);functionremoveLiquidityETHWithPermit(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline,bool approveMax,uint8 v,bytes32 r,bytes32 s)externalreturns(uint amountToken,uint amountETH);functionswapExactTokensForTokens(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);functionswapTokensForExactTokens(uint amountOut,uint amountInMax,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);functionswapExactETHForTokens(uint amountOutMin,address[]calldata path,address to,uint deadline)externalpayablereturns(uint[]memory amounts);functionswapTokensForExactETH(uint amountOut,uint amountInMax,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);functionswapExactTokensForETH(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)externalreturns(uint[]memory amounts);functionswapETHForExactTokens(uint amountOut,address[]calldata path,address to,uint deadline)externalpayablereturns(uint[]memory amounts);functionquote(uint amountA,uint reserveA,uint reserveB)externalpurereturns(uint amountB);functiongetAmountOut(uint amountIn,uint reserveIn,uint reserveOut)externalpurereturns(uint amountOut);functiongetAmountIn(uint amountOut,uint reserveIn,uint reserveOut)externalpurereturns(uint amountIn);functiongetAmountsOut(uint amountIn,address[]calldata path)externalviewreturns(uint[]memory amounts);functiongetAmountsIn(uint amountOut,address[]calldata path)externalviewreturns(uint[]memory amounts);}interfaceIUniswapV2Router02is IUniswapV2Router01 {functionremoveLiquidityETHSupportingFeeOnTransferTokens(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline)externalreturns(uint amountETH);functionremoveLiquidityETHWithPermitSupportingFeeOnTransferTokens(address token,uint liquidity,uint amountTokenMin,uint amountETHMin,address to,uint deadline,bool approveMax,uint8 v,bytes32 r,bytes32 s)externalreturns(uint amountETH);functionswapExactTokensForTokensSupportingFeeOnTransferTokens(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)external;functionswapExactETHForTokensSupportingFeeOnTransferTokens(uint amountOutMin,address[]calldata path,address to,uint deadline)externalpayable;functionswapExactTokensForETHSupportingFeeOnTransferTokens(uint amountIn,uint amountOutMin,address[]calldata path,address to,uint deadline)external;}
```

# ABI
```
import IUniswapV2Router02 from'@uniswap/v2-periphery/build/IUniswapV2Router02.json'
```

<https://unpkg.com/@uniswap/v2-periphery@1.1.0-beta.0/build/IUniswapV2Router02.json>
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/06-router02.md)
Was this helpful?
[PreviousRouter01](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-01)[NextCommon Errors](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/common-errors)
On this page
  * [factory](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#factory)
  * [WETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#weth)
  * [quote](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#quote)
  * [getAmountOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountout)
  * [getAmountIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountin)
  * [getAmountsOut](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountsout)
  * [getAmountsIn](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#getamountsin)
  * [addLiquidity](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#addliquidity)
  * [addLiquidityETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#addliquidityeth)
  * [removeLiquidity](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidity)
  * [removeLiquidityETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityeth)
  * [removeLiquidityWithPermit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquiditywithpermit)
  * [removeLiquidityETHWithPermit](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethwithpermit)
  * [removeLiquidityETHSupportingFeeOnTransferTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethsupportingfeeontransfertokens)
  * [removeLiquidityETHWithPermitSupportingFeeOnTransferTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#removeliquidityethwithpermitsupportingfeeontransfertokens)
  * [swapExactTokensForTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensfortokens)
  * [swapTokensForExactTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swaptokensforexacttokens)
  * [swapExactETHForTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexactethfortokens)
  * [swapTokensForExactETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swaptokensforexacteth)
  * [swapExactTokensForETH](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensforeth)
  * [swapETHForExactTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapethforexacttokens)
  * [swapExactTokensForTokensSupportingFeeOnTransferTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensfortokenssupportingfeeontransfertokens)
  * [swapExactETHForTokensSupportingFeeOnTransferTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexactethfortokenssupportingfeeontransfertokens)
  * [swapExactTokensForETHSupportingFeeOnTransferTokens](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/router-02#swapexacttokensforethsupportingfeeontransfertokens)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/reference/smart-contracts/06-router02.md)
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
