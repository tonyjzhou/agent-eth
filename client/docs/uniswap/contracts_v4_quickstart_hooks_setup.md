# https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup

[Skip to main content](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#__docusaurus_skipToContent_fallback)
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
          * [Set Up Local Environment](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)
          * [Swap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap)
          * [Liquidity Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/liquidity)
          * [AsyncSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/async-swap)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/reference/errors/)
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
  * Quickstart
  * Hooks
  * [Set Up Local Environment](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup)


On this page
Before writing the hook let's first have a local environment properly configured e.g. deploying pool manager, utility routers and test tokens.
At the end of this guide a development environment will be set up to be used to build the rest of the examples in the Guides section of the docs.
To get started as quickly as possible for building Uniswap v4 hooks, there is a `Quick Start` section below to clone a boilerplate and get building. To start from scratch and learn the underlying concepts, jump to the `Start from Scratch` section.
# Quick Start
The Uniswap [v4-template repo](https://github.com/uniswapfoundation/v4-template) provides a basic foundry environment with required imports already pre-loaded. Click on [`Use this template`](https://github.com/new?template_name=v4-template&template_owner=uniswapfoundation) to create a new repository with it.
Or simply clone it and install the dependencies:
```
git clone https://github.com/uniswapfoundation/v4-template.gitcd v4-template# requires foundryforge installforge test
```

Then hop to the [Local Node via Anvil](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#local-node-via-anvil) to complete the set up and start developing.
# Start from Scratch
In the following sections, let's walk through the steps to create the same environment set up as the boilerplate from scratch and learn the underlying concepts.
## Setting up Foundry[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#setting-up-foundry "Direct link to Setting up Foundry")
First thing is to set up a new Foundry project.
If there is no Foundry installed - follow the [Foundry Book](https://book.getfoundry.sh/getting-started/installation) for installation.
Once Foundry is setup, initialize a new project:
```
forge init counter-hookcd counter-hook
```

Then install the Uniswap `v4-core` and `v4-periphery` contracts as dependencies:
```
forge install Uniswap/v4-core && forge install Uniswap/v4-periphery
```

Next, set up the remappings so that the shorthand syntax for importing contracts from the dependencies work nicely:
```
forge remappings > remappings.txt
```

> If there is something wrong with the inferred remappings, please replace with the following in `remappings.txt`:
```
@uniswap/v4-core/=lib/v4-core/forge-gas-snapshot/=lib/v4-core/lib/forge-gas-snapshot/src/forge-std/=lib/v4-core/lib/forge-std/src/permit2/=lib/v4-periphery/lib/permit2/solmate/=lib/v4-core/lib/solmate/v4-core/=lib/v4-core/v4-periphery/=lib/v4-periphery/
```

After that, remove the default Counter contract and its associated test and script file that Foundry initially set up. To do that, either manually delete those files, or just run the following:
```
rm ./**/Counter*.sol
```

Finally, since v4 uses transient storage which is only available after Ethereum's cancun hard fork and on Solidity versions >= 0.8.24 - some config must be set in `foundry.toml` config file.
To do that, add the following lines to `foundry.toml`:
```
# foundry.tomlsolc_version = "0.8.26"evm_version = "cancun"ffi = true
```

Awesome! Now it's all set to start building the hook! Let’s run a quick test to confirm everything is set up properly.
## Compile a Basic Hook Contract[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#compile-a-basic-hook-contract "Direct link to Compile a Basic Hook Contract")
To confirm that the environment is configured correctly let's write a basic Counter Hook contract. Create a new file, `./src/CounterHook.sol` and paste the following code into it:
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.24;import{BaseHook}from"v4-periphery/src/utils/BaseHook.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{PoolKey}from"v4-core/src/types/PoolKey.sol";import{PoolId, PoolIdLibrary}from"v4-core/src/types/PoolId.sol";import{BalanceDelta}from"v4-core/src/types/BalanceDelta.sol";import{BeforeSwapDelta, BeforeSwapDeltaLibrary}from"v4-core/src/types/BeforeSwapDelta.sol";contractCounterHookis BaseHook {usingPoolIdLibraryfor PoolKey;// NOTE: ---------------------------------------------------------// state variables should typically be unique to a pool// a single hook contract should be able to service multiple pools// ---------------------------------------------------------------mapping(PoolId =>uint256 count)public beforeSwapCount;mapping(PoolId =>uint256 count)public afterSwapCount;mapping(PoolId =>uint256 count)public beforeAddLiquidityCount;mapping(PoolId =>uint256 count)public beforeRemoveLiquidityCount;constructor(IPoolManager _poolManager)BaseHook(_poolManager){}functiongetHookPermissions()publicpure override returns(Hooks.Permissions memory){return Hooks.Permissions({      beforeInitialize:false,      afterInitialize:false,      beforeAddLiquidity:true,      afterAddLiquidity:false,      beforeRemoveLiquidity:true,      afterRemoveLiquidity:false,      beforeSwap:true,      afterSwap:true,      beforeDonate:false,      afterDonate:false,      beforeSwapReturnDelta:false,      afterSwapReturnDelta:false,      afterAddLiquidityReturnDelta:false,      afterRemoveLiquidityReturnDelta:false});}// -----------------------------------------------// NOTE: see IHooks.sol for function documentation// -----------------------------------------------function_beforeSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata,bytescalldata)internal    overridereturns(bytes4, BeforeSwapDelta,uint24){    beforeSwapCount[key.toId()]++;return(BaseHook.beforeSwap.selector, BeforeSwapDeltaLibrary.ZERO_DELTA,0);}function_afterSwap(address, PoolKey calldata key, IPoolManager.SwapParams calldata, BalanceDelta,bytescalldata)internal    overridereturns(bytes4,int128){    afterSwapCount[key.toId()]++;return(BaseHook.afterSwap.selector,0);}function_beforeAddLiquidity(address,    PoolKey calldata key,    IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal override returns(bytes4){    beforeAddLiquidityCount[key.toId()]++;return BaseHook.beforeAddLiquidity.selector;}function_beforeRemoveLiquidity(address,    PoolKey calldata key,    IPoolManager.ModifyLiquidityParams calldata,bytescalldata)internal override returns(bytes4){    beforeRemoveLiquidityCount[key.toId()]++;return BaseHook.beforeRemoveLiquidity.selector;}}
```

To compile the Counter Hook contracts in the `./src` folder, use the foundry build command:
```
forge build
```

If the environment is compiled correctly it will display a message:
```
Compiler run successful!
```

## Local Node via Anvil[​](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#local-node-via-anvil "Direct link to Local Node via Anvil")
Other than writing unit tests, [Anvil](https://book.getfoundry.sh/anvil/) can be used to deploy and test hooks.
With the local node up and running, use the `--rpc-url 127.0.0.1:8545` flag in tests to point the Foundry testing suite to that local node:
```
# start anvil, a local EVM chainanvil# in a new terminal# foundry script for deploying v4 & hooks to anvilforge script script/Anvil.s.sol \  --rpc-url http://localhost:8545 \  --private-key <test_wallet_private_key> \  --broadcast# test on the anvil local nodeforge test --rpc-url 127.0.0.1:8545
```

# Next Steps
With the environment set up ready to be built on. Jump over to the guides section to learn about the Uniswap functions that can be integrated with Hook. Remember to add all contracts (.sol files) to the `./src` folder and their subsequent tests to the `./test` folder. Then test them against the local anvil node by running:
```
forge test --rpc-url 127.0.0.1:8545
```

# Appendix: OpenZeppelin Hooks Library
> [OpenZeppelin Hooks Library](https://docs.openzeppelin.com/uniswap-hooks/1.x/), included in [v4-template](https://github.com/uniswapfoundation/v4-template), provides secure and modular reference implementations for Uniswap v4 Hooks!
If you're starting from scratch, you can install the OpenZeppelin Hooks library:
```
$ forge install OpenZeppelin/uniswap-hooks
```

The library includes:
  * **BaseHook** : Core scaffolding with security checks and permission management
  * **BaseCustomAccounting** : For implementing hook-owned liquidity and custom token accounting
  * **BaseCustomCurve** : For replacing default concentrated liquidity math with custom swap logic
  * **BaseAsyncSwap** : For implementing non-atomic and asynchronous swaps
  * **BaseDynamicFee** : For implementing dynamic fee pools
  * **BaseOverrideFee** : For implementing dynamic fees on every swap
  * **BaseDynamicAfterFee** : For implementing post-swap fee adjustments based on actual swap output


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/00-setup.mdx)
Was this helpful?
[PreviousSwap](https://docs.uniswap.org/contracts/v4/quickstart/swap)[NextSwap Hooks](https://docs.uniswap.org/contracts/v4/quickstart/hooks/swap)
On this page
  * [Setting up Foundry](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#setting-up-foundry)
  * [Compile a Basic Hook Contract](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#compile-a-basic-hook-contract)
  * [Local Node via Anvil](https://docs.uniswap.org/contracts/v4/quickstart/hooks/setup#local-node-via-anvil)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/quickstart/04-hooks/00-setup.mdx)
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
