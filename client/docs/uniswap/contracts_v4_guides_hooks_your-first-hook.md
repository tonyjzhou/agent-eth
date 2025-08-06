# https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
          * [Building Your First Hook](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
          * [Hook Deployment](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
        * [Unlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
        * [Reading Pool State](https://docs.uniswap.org/contracts/v4/guides/read-pool-state)
        * [Custom Accounting](https://docs.uniswap.org/contracts/v4/guides/custom-accounting)
        * [Swap routing](https://docs.uniswap.org/contracts/v4/guides/swap-routing)
        * [ERC-6909](https://docs.uniswap.org/contracts/v4/guides/ERC-6909)
        * [Position Manager](https://docs.uniswap.org/contracts/v4/guides/position-manager)
        * [StateView](https://docs.uniswap.org/contracts/v4/guides/state-view)
        * [Flash Accounting](https://docs.uniswap.org/contracts/v4/guides/flash-accounting)
        * [Access msg.sender Inside a Hook](https://docs.uniswap.org/contracts/v4/guides/accessing-msg.sender-using-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [Permit2](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Guides
  * Hooks
  * [Building Your First Hook](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)


On this page
# Introduction
Uniswap introduced the v4 of their protocol introducing several new concepts such as hooks, flash accounting, singleton architecture and more. The most interesting of these for developers is hooks, and that’s what we’ll be learning about today.
In this guide, we’ll be conceptualizing, understanding and building a basic points hook, which will give you some idea of how to build your own hook.
## What are we building?[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#what-are-we-building "Direct link to What are we building?")
Let’s start by conceptualizing what we’re building today and why. Let’s say you have a token named `TOKEN` that you want to incentivize people to buy. One way of doing so is awarding people points when they buy your token. Prior to v4, you’d have to do this off-chain or via your own helper contract outside of the swap logic, but in v4 you can enable universal access using hooks.
Let’s start by defining when users will be rewarded with these points:
  1. When the user swaps `ETH` into `TOKEN` they will get awarded points equal to how much `ETH` they swapped the token with.
  2. When the user adds liquidity, we award them with points equal to the amount of `ETH` they added.


In order to keep track of these points, we’ll mint the `POINTS` token to the user, this has an added benefit for the user to be able to track it in their wallet.
# Hook Design
Let’s figure out how our hook will work.
From the Uniswap v4 Documentation, there are several hooks available for developers to integrate with. In our use case, we specifically need the ability to read swaps and figure out what amounts they are swapping for and who they are.
For our hook, we’ll be using `afterSwap` and `afterAddLiquidity` hooks. Why these instead of the `before...` hooks? We’ll dig deeper into this later in this guide.
_Note: To initiate the swap in the first place, this is where[`UniversalRouter`](https://docs.uniswap.org/contracts/universal-router/overview) comes into play where we will pass in the [`V4_SWAP`](https://github.com/Uniswap/universal-router/blob/main/contracts/libraries/Commands.sol#L35) command to `UniversalRouter.execute`._
# Let’s create our hook!
We’ll call this hook `PointsHook` and create it in such a way that any pool paired with `TOKEN` can use it.
## Setting things up[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#setting-things-up "Direct link to Setting things up")
The Uniswap [v4-template repo](https://github.com/uniswapfoundation/v4-template) provides a basic foundry environment with required imports already pre-loaded. Click on [`Use this template`](https://github.com/new?template_name=v4-template&template_owner=uniswapfoundation) to create a new repository with it.
Or simply clone it and install the dependencies:
```
git clone https://github.com/uniswapfoundation/v4-template.gitcd v4-template# requires foundryforge installforge test
```

After that let's create a new contract `PointsHook.sol` in `src` folder with the following codes:
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;import{BaseHook}from"v4-periphery/src/utils/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/src/types/PoolId.sol";import{BalanceDelta}from"v4-core/src/types/BalanceDelta.sol";import{BeforeSwapDelta, BeforeSwapDeltaLibrary}from"v4-core/src/types/BeforeSwapDelta.sol";contractPointsHookis BaseHook {constructor(IPoolManager _poolManager)BaseHook(_poolManager){}functiongetHookPermissions()publicpure    overridereturns(Hooks.Permissions memory){return      Hooks.Permissions({        beforeInitialize:false,        afterInitialize:false,        beforeAddLiquidity:false,        afterAddLiquidity:true,        beforeRemoveLiquidity:false,        afterRemoveLiquidity:false,        beforeSwap:false,        afterSwap:true,        beforeDonate:false,        afterDonate:false,        beforeSwapReturnDelta:false,        afterSwapReturnDelta:false,        afterAddLiquidityReturnDelta:false,        afterRemoveLiquidityReturnDelta:false});}}
```

The above code does the following:
  * import the relevant dependencies
  * initialize the constructor by passing in the instance of PoolManager
  * override `getHookPermissions` from `BaseHook.sol` to return a struct of permissions to signal which hook functions are to be implemented. It will also be used at deployment to validate the address correctly represents the expected permissions.


Awesome! Now it's all set to start building the hook!
## Basic Structure[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#basic-structure "Direct link to Basic Structure")
So far, we’ve created the file named `PointsHook.sol` which only contains the outline of a hook contract. Let’s add our `afterSwap` and `afterAddLiquidity` hooks to it.
```
contractPointsHookis BaseHook {constructor(IPoolManager _poolManager)BaseHook(_poolManager){}functiongetHookPermissions()publicpure    overridereturns(Hooks.Permissions memory){return      Hooks.Permissions({        beforeInitialize:false,        afterInitialize:false,        beforeAddLiquidity:false,        afterAddLiquidity:true,        beforeRemoveLiquidity:false,        afterRemoveLiquidity:false,        beforeSwap:false,        afterSwap:true,        beforeDonate:false,        afterDonate:false,        beforeSwapReturnDelta:false,        afterSwapReturnDelta:false,        afterAddLiquidityReturnDelta:false,        afterRemoveLiquidityReturnDelta:false});}function_afterSwap(address,    PoolKey calldata key,    IPoolManager.SwapParams calldata,    BalanceDelta delta,bytescalldata)internal override returns(bytes4,int128){return(BaseHook.afterSwap.selector,0);}function_afterAddLiquidity(address sender,    PoolKey calldata key,    IPoolManager.ModifyLiquidityParams calldata params,    BalanceDelta delta,    BalanceDelta feesAccrued,bytescalldata hookData)internal override returns(bytes4, BalanceDelta){return(BaseHook.afterAddLiquidity.selector, delta);}}
```

You’ll notice that both hooks return their own selector in the functions, this is a pattern used by the protocol to signal “successful” invocation. We’ll talk about rest of the return parameters when we start adding the functionality.
Most of the code at this point should be self-explanatory. It’s not doing anything yet, but it’s a great place to start adding the functionality we need.
## Points Logic[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#points-logic "Direct link to Points Logic")
First, let’s setup the `POINTS` token that we’ll reward users with via creating another contract `PointsToken.sol` and import relevant dependencies like `ERC20` and `Owned`.
```
contractPointsTokenis ERC20, Owned {constructor()ERC20("Points Token","POINTS",18)Owned(msg.sender){}functionmint(address to,uint256 amount)external onlyOwner {_mint(to, amount);}}
```

Let’s make it so that our hook can mint some!
```
contractPointsHookis BaseHook {  PointsToken public pointsToken;constructor(IPoolManager _poolManager)BaseHook(_poolManager){    pointsToken =newPointsToken();}[...]}
```

Next, we need to calculate how many points to assign based on the `ETH` value of the swap or liquidity action. We’ll be awarding `POINTS` in 1:1 ratio for the `ETH`, so if the user swapped 1 `ETH`, we’ll give them 1 `POINTS`. Let’s also create a function to award these to the user.
```
functiongetPointsForAmount(uint256 amount)internalpurereturns(uint256){return amount;// 1:1 with ETH}functionawardPoints(address to,uint256 amount)internal{    pointsToken.mint(to,getPointsForAmount(amount));}
```

## Hook Logic[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#hook-logic "Direct link to Hook Logic")
Now we need to actually get the value that the user is swapping or adding liquidity with. We’ll be using the two hooks to achieve that functionality.
### Getting the user address[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#getting-the-user-address "Direct link to Getting the user address")
Before we go into the logic for the hook, we have a side quest! How do we actually get the address of the user? The `PositionManager` doesn’t pass the user address directly to the hook, mainly because of the complexity of getting that data in the first place.
You’d have noticed, both of our hooks have a `hookData` field at the end. This allows any arbitrary data to be passed to the hook at the time of invocation, and we’ll use this field to encode the user address.
Let’s create some helper functions to encode and decode this data:
```
functiongetHookData(address user)publicpurereturns(bytesmemory){return abi.encode(user);}functionparseHookData(bytescalldata data)publicpurereturns(address user){return abi.decode(data,(address));}
```

### Hook Logic: `afterSwap`[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#hook-logic-afterswap "Direct link to hook-logic-afterswap")
In order for us to award these points to the user, we need a few things and we also need to create a few conditions.
Let’s start with the most basic ones. We want the user to be swapping in the `ETH/TOKEN` pool and be buying the `TOKEN` in order to get awarded these `POINTS` token. Next, we need to figure out who the user is and how much ETH they are spending, and finally award the points accordingly.
```
function_afterSwap(address,    PoolKey calldata key,    IPoolManager.SwapParams calldata swapParams,    BalanceDelta delta,bytescalldata hookData)internal override onlyPoolManager returns(bytes4,int128){// We only award points in the ETH/TOKEN pools.if(!key.currency0.isAddressZero()){return(BaseHook.afterSwap.selector,0);}// We only award points if the user is buying the TOKENif(!swapParams.zeroForOne){return(BaseHook.afterSwap.selector,0);}// Let's figure out who's the useraddress user =parseHookData(hookData);// How much ETH are they spending?uint256 ethSpendAmount =uint256(int256(-delta.amount0()));// And award the points!awardPoints(user, ethSpendAmount);return(BaseHook.afterSwap.selector,0);}
```

That middle section about figuring out how much `ETH` the user spent seems a little fishy, what’s going on there? Let’s break it down.
When `amountSpecified` is less than 0, it means this is an `exact input for output` swap, essentially where the user is coming in with an exact amount of ETH. In the other case, it’s an `exact output for input` swap, where the user is expecting a specific amount out. In our case, we get this from the precalculated `delta` that Uniswap V4 gives us as a part of the `afterSwap` hook.
### Hook Logic: `afterAddLiquidity`[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#hook-logic-afteraddliquidity "Direct link to hook-logic-afteraddliquidity")
Similar to what we did for the `afterSwap` hook, now we need to award users for adding liquidity. We’ll do the exact same thing here, except we’ll award the points based on the added liquidity.
```
function_afterAddLiquidity(address sender,    PoolKey calldata key,    IPoolManager.ModifyLiquidityParams calldata params,    BalanceDelta delta,    BalanceDelta feesAccrued,bytescalldata hookData)internal override onlyPoolManager returns(bytes4, BalanceDelta){// We only award points in the ETH/TOKEN pools.if(!key.currency0.isAddressZero()){return(BaseHook.afterAddLiquidity.selector, delta);}// Let's figure out who's the useraddress user =parseHookData(hookData);// How much ETH are they spending?uint256 ethSpendAmount =uint256(int256(-delta.amount0()));// And award the points!awardPoints(user, ethSpendAmount);return(BaseHook.afterAddLiquidity.selector, delta);}
```

note
It is important to note that the delta should be passed to awardPoints function as it avoids amount errors in case of partial fills.
# Testing
We’re using Foundry for building our hook, and we’ll continue using it to write our tests. One of the great things about Foundry is that you can write tests in Solidity itself instead of context switching between another language.
### Test Suite[​](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#test-suite "Direct link to Test Suite")
The v4-template repo you cloned already has an existing base test file, let’s start by copying it into `PointsHook.t.sol`.
```
contractPointsHookTestis Test, Fixtures {usingEasyPosmfor IPositionManager;usingStateLibraryfor IPoolManager;  PointsHook hook;  PointsToken pointsToken;  PoolId poolId;uint256 tokenId;int24 tickLower;int24 tickUpper;functionsetUp()public{// creates the pool manager, utility routers, and test tokensdeployFreshManagerAndRouters();deployMintAndApprove2Currencies();deployAndApprovePosm(manager);// Deploy the hook to an address with the correct flagsaddress flags =address(uint160(Hooks.AFTER_SWAP_FLAG | Hooks.AFTER_ADD_LIQUIDITY_FLAG)^(0x4444<<144)// Namespace the hook to avoid collisions);bytesmemory constructorArgs = abi.encode(manager);//Add all the necessary constructor arguments from the hookdeployCodeTo("PointsHook.sol:PointsHook", constructorArgs, flags);    hook =PointsHook(flags);    pointsToken = hook.pointsToken();// Create the pool    key =PoolKey(      Currency.wrap(address(0)),      currency1,3000,60,IHooks(hook));    poolId = key.toId();    manager.initialize(key, SQRT_PRICE_1_1);// Provide full-range liquidity to the pool    tickLower = TickMath.minUsableTick(key.tickSpacing);    tickUpper = TickMath.maxUsableTick(key.tickSpacing);deal(address(this),200 ether);(uint256 amount0,uint256 amount1)= LiquidityAmounts.getAmountsForLiquidity(        SQRT_PRICE_1_1,        TickMath.getSqrtPriceAtTick(tickLower),        TickMath.getSqrtPriceAtTick(tickUpper),uint128(100e18));(tokenId,)= posm.mint(      key,      tickLower,      tickUpper,100e18,      amount0 +1,      amount1 +1,address(this),      block.timestamp,      hook.getHookData(address(this)));}functiontest_PointsHook_Swap()public{// [code here]}}
```

So far this test setup is fairly simple, we create a bunch of tokens and deploy v4 along with the position manager inside our test. Then, we create a pool with our hook and add some liquidity using the position manager.
Now, let’s write our test. We’ll start by testing the points awarded during the swap.
```
functiontest_PointsHook_Swap()public{// We already have some points because we added some liquidity during setup.// So, we'll subtract those from the total points to get the points awarded for this swap.uint256 startingPoints = pointsToken.balanceOf(address(this));// Let's swap some ETH for the token.bool zeroForOne =true;int256 amountSpecified =-1e18;// negative number indicates exact input swap!    BalanceDelta swapDelta =swap(      key,      zeroForOne,      amountSpecified,      hook.getHookData(address(this)));uint256 endingPoints = pointsToken.balanceOf(address(this));// Let's make sure we got the right amount of points!assertEq(      endingPoints - startingPoints,uint256(-amountSpecified),"Points awarded for swap should be 1:1 with ETH");}
```

This test case is fairly straightforward and simply swaps 1 ETH for the target token and compares if we got the right amount of points awarded for it.
Next, we should test our liquidity addition.
```
functiontest_PointsHook_Liquidity()public{// We already have some points because we added some liquidity during setup.// So, we'll subtract those from the total points to get the points awarded for this swap.uint256 startingPoints = pointsToken.balanceOf(address(this));uint128 liqToAdd =100e18;(uint256 amount0,uint256 amount1)= LiquidityAmounts.getAmountsForLiquidity(        SQRT_PRICE_1_1,        TickMath.getSqrtPriceAtTick(tickLower),        TickMath.getSqrtPriceAtTick(tickUpper),        liqToAdd);    posm.mint(      key,      tickLower,      tickUpper,      liqToAdd,      amount0 +1,      amount1 +1,address(this),      block.timestamp,      hook.getHookData(address(this)));uint256 endingPoints = pointsToken.balanceOf(address(this));// Let's make sure we got the right amount of points!assertApproxEqAbs(endingPoints - startingPoints,uint256(liqToAdd),10);}
```

This test case looks very similar to the `afterSwap` one, except we’re testing based on the liquidity added. You’ll notice at the end we’re testing for approximate equality within 10 points. This is to account for minor differences in actual liquidity added due to ticks and pricing.
# Next Steps
Congratulations on building your very first hook! You could explore further by going to [Hook Deployment](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment) to learn about how hook flags work and see how we will deploy a hook in action.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/hooks/01-your-first-hook.md)
Was this helpful?
[PreviousSubscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)[NextHook Deployment](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
On this page
  * [What are we building?](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#what-are-we-building)
  * [Setting things up](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#setting-things-up)
  * [Basic Structure](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#basic-structure)
  * [Points Logic](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#points-logic)
  * [Hook Logic](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#hook-logic)
    * [Getting the user address](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#getting-the-user-address)
    * [Hook Logic: `afterSwap`](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#hook-logic-afterswap)
    * [Hook Logic: `afterAddLiquidity`](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#hook-logic-afteraddliquidity)
    * [Test Suite](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook#test-suite)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/hooks/01-your-first-hook.md)
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
