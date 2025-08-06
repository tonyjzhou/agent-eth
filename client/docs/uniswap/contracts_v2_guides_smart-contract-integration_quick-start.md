# https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start

[Skip to main content](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Quickstart](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [UniswapX](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [Universal Router](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [Permit2](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Overview](https://docs.uniswap.org/contracts/v2/overview)
      * [Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Protocol Overview](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Core Concepts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Advanced Topics](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
      * [Guides](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Interface Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Smart Contract Integration](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
          * [Implement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
          * [Providing Liquidity](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/providing-liquidity)
          * [Building an Oracle](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/building-an-oracle)
          * [Flash Swaps](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/using-flash-swaps)
          * [Pair Addresses](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/getting-pair-addresses)
          * [Supporting meta transactions](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/supporting-meta-transactions)
      * [Technical Reference](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [API](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Governance](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
        * [Smart Contracts](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v2 Protocol
  * Guides
  * Smart Contract Integration
  * [Smart Contract Quick start](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start)


On this page
# Smart Contract Quick start
Developing smart contracts for Ethereum involves a variety of off-chain tools used for producing and testing bytecode that runs on the [Ethereum Virtual Machine (EVM)](https://eth.wiki/en/concepts/evm/ethereum-virtual-machine-\(evm\)-awesome-list). Some tools also include workflows for deploying this bytecode to the Ethereum network and testnets. There are many options for these tools. This guide walks you through writing and testing a simple smart contract that interacts with the Uniswap Protocol using one specific set of tools (`truffle` + `npm` + `mocha`).
## Requirements[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#requirements "Direct link to Requirements")
To follow this guide, you must have the following installed:
  * [nodejs >= v12.x & npm >= 6.x](https://nodejs.org/en/)


## Bootstrapping a project[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#bootstrapping-a-project "Direct link to Bootstrapping a project")
You can start from scratch, but it's easier to use a tool like `truffle` to bootstrap an empty project. Create an empty directory and run `npx truffle init` inside that directory to unbox the default [Truffle box](https://www.trufflesuite.com/boxes).
```
mkdir democd demonpx truffle init
```

## Setting up npm[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#setting-up-npm "Direct link to Setting up npm")
In order to reference the Uniswap V2 contracts, you should use the npm artifacts we deploy containing the core and periphery smart contracts and interfaces. To add npm dependencies, we must first initialize the npm package. We can run `npm init` in the same directory to create a `package.json` file. You can accept all the defaults and modify them later.
```
npm init
```

## Adding dependencies[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#adding-dependencies "Direct link to Adding dependencies")
Now that we have an npm package, we can add our dependencies. Let's add both the [`@uniswap/v2-core`](https://www.npmjs.com/package/@uniswap/v2-core) and [`@uniswap/v2-periphery`](https://www.npmjs.com/package/@uniswap/v2-periphery) packages.
```
npm i --save @uniswap/v2-corenpm i --save @uniswap/v2-periphery
```

If you check the `node_modules/@uniswap` directory, you can now find the Uniswap V2 contracts.
```
moody@MacBook-Pro ~/I/u/demo> ls node_modules/@uniswap/v2-core/contractsUniswapV2ERC20.sol  UniswapV2Pair.sol   libraries/UniswapV2Factory.sol interfaces/      test/moody@MacBook-Pro ~/I/u/demo> ls node_modules/@uniswap/v2-periphery/contracts/UniswapV2Migrator.sol examples/       test/UniswapV2Router01.sol interfaces/UniswapV2Router02.sol libraries/
```

These packages include both the smart contract source code and the build artifacts.
## Writing our contract[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#writing-our-contract "Direct link to Writing our contract")
We can now get started writing our example contract. For writing Solidity, we recommend IntelliJ or VSCode with a solidity plugin, but you can use any text editor. Let's write a contract that returns the value of some amount of liquidity shares for a given token pair. First create a couple of files:
```
mkdir contracts/interfacestouch contracts/interfaces/ILiquidityValueCalculator.soltouch contracts/LiquidityValueCalculator.sol
```

This will be the interface of the contract we implement. Put it in `contracts/interfaces/ILiquidityValueCalculator.sol`.
```
pragmasolidity^0.6.6;interfaceILiquidityValueCalculator{functioncomputeLiquidityShareValue(uint liquidity,address tokenA,address tokenB)externalreturns(uint tokenAAmount,uint tokenBAmount);}
```

Now let's start with the constructor. You need to know where the `UniswapV2Factory` is deployed in order to compute the address of the pair and look up the total supply of liquidity shares, plus the amounts for the reserves. We can store this address as a parameter passed to the constructor.
The factory address is constant on mainnet and all testnets, so it may be tempting to make this value a constant in your contract, but since we need to unit test the contract it should be an argument. You can use solidity immutables to save on gas when accessing this variable.
```
pragmasolidity^0.6.6;import'./interfaces/ILiquidityValueCalculator.sol';contractLiquidityValueCalculatoris ILiquidityValueCalculator {addresspublic factory;constructor(address factory_)public{    factory = factory_;}}
```

Now we need to be able to look up the total supply of liquidity for a pair, and its token balances. Let's put this in a separate function. To implement it, we must:
  1. Look up the pair address
  2. Get the reserves of the pair
  3. Get the total supply of the pair liquidity
  4. Sort the reserves in the order of tokenA, tokenB


The [`UniswapV2Library`](https://docs.uniswap.org/contracts/v2/reference/smart-contracts/library) has some helpful methods for this.
```
pragmasolidity^0.6.6;import'./interfaces/ILiquidityValueCalculator.sol';import'@uniswap/v2-periphery/contracts/libraries/UniswapV2Library.sol';import'@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol';contractLiquidityValueCalculatoris ILiquidityValueCalculator {functionpairInfo(address tokenA,address tokenB)internalviewreturns(uint reserveA,uint reserveB,uint totalSupply){    IUniswapV2Pair pair =IUniswapV2Pair(UniswapV2Library.pairFor(factory, tokenA, tokenB));    totalSupply = pair.totalSupply();(uint reserves0,uint reserves1,)= pair.getReserves();(reserveA, reserveB)= tokenA == pair.token0()?(reserves0, reserves1):(reserves1, reserves0);}}
```

Finally, we just need to compute the share value. We will leave that as an exercise to the reader.
```
pragmasolidity^0.6.6;import'./interfaces/ILiquidityValueCalculator.sol';import'@uniswap/v2-periphery/contracts/libraries/UniswapV2Library.sol';import'@uniswap/v2-core/contracts/interfaces/IUniswapV2Pair.sol';contractLiquidityValueCalculatoris ILiquidityValueCalculator {addresspublic factory;constructor(address factory_)public{    factory = factory_;}functionpairInfo(address tokenA,address tokenB)internalviewreturns(uint reserveA,uint reserveB,uint totalSupply){    IUniswapV2Pair pair =IUniswapV2Pair(UniswapV2Library.pairFor(factory, tokenA, tokenB));    totalSupply = pair.totalSupply();(uint reserves0,uint reserves1,)= pair.getReserves();(reserveA, reserveB)= tokenA == pair.token0()?(reserves0, reserves1):(reserves1, reserves0);}functioncomputeLiquidityShareValue(uint liquidity,address tokenA,address tokenB)external override returns(uint tokenAAmount,uint tokenBAmount){revert('TODO');}}
```

## Writing tests[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#writing-tests "Direct link to Writing tests")
In order to test your contract, you need to:
  1. Bring up a testnet
  2. Deploy the `UniswapV2Factory`
  3. Deploy at least 2 ERC20 tokens for a pair
  4. Create a pair for the factory
  5. Deploy your `LiquidityValueCalculator` contract
  6. Call `LiquidityValueCalculator#computeLiquidityShareValue`
  7. Verify the result with an assertion


#1 is handled for you automatically by the `truffle test` command.
Note that you should only deploy the precompiled Uniswap contracts in the `build` directories for unit tests. This is because solidity appends a metadata hash to compiled contract artifacts which includes the hash of the contract source code path, and compilations on other machines will not result in the exact same bytecode. This is problematic because in Uniswap V2 we use the hash of the bytecode in the v2-periphery [`UniswapV2Library`](https://github.com/Uniswap/uniswap-v2-periphery/blob/master/contracts/libraries/UniswapV2Library.sol#L24), to compute the pair address.
To get the bytecode for deploying UniswapV2Factory, you can import the file via:
```
constUniswapV2FactoryBytecode=require('@uniswap/v2-core/build/UniswapV2Factory.json').bytecode
```

We recommend using a standard ERC20 from `@openzeppelin/contracts` for deploying an ERC20.
You can read more about deploying contracts and writing tests using Truffle [here](https://www.trufflesuite.com/docs/truffle/testing/writing-tests-in-javascript).
## Compiling and deploying the contract[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#compiling-and-deploying-the-contract "Direct link to Compiling and deploying the contract")
Learn more about compiling and deploying contracts using Truffle [here](https://www.trufflesuite.com/docs/truffle/getting-started/compiling-contracts) and [here](https://www.trufflesuite.com/docs/truffle/getting-started/running-migrations) respectively.
## WIP[​](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#wip "Direct link to WIP")
This guide is a WIP. Please contribute to this guide with the edit button below!
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/01-quick-start.md)
Was this helpful?
[PreviousIframe Integration](https://docs.uniswap.org/contracts/v2/guides/interface-integration/iframe-integration)[NextImplement a Swap](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/trading-from-a-smart-contract)
On this page
  * [Requirements](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#requirements)
  * [Bootstrapping a project](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#bootstrapping-a-project)
  * [Setting up npm](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#setting-up-npm)
  * [Adding dependencies](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#adding-dependencies)
  * [Writing our contract](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#writing-our-contract)
  * [Writing tests](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#writing-tests)
  * [Compiling and deploying the contract](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#compiling-and-deploying-the-contract)
  * [WIP](https://docs.uniswap.org/contracts/v2/guides/smart-contract-integration/quick-start#wip)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v2/guides/smart-contract-integration/01-quick-start.md)
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
