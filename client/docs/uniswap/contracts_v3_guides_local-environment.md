# https://docs.uniswap.org/contracts/v3/guides/local-environment

[Skip to main content](https://docs.uniswap.org/contracts/v3/guides/local-environment#__docusaurus_skipToContent_fallback)
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
    * [v3 Protocol](https://docs.uniswap.org/contracts/v3/overview)
      * [Overview](https://docs.uniswap.org/contracts/v3/overview)
      * [Guides](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)
        * [Implement A Swap](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
        * [Providing Liquidity](https://docs.uniswap.org/contracts/v3/guides/providing-liquidity/setting-up)
        * [Liquidity Mining](https://docs.uniswap.org/contracts/v3/guides/liquidity-mining/overview)
        * [Implement Flash Swaps](https://docs.uniswap.org/contracts/v3/guides/flash-integrations/inheritance-constructors)
        * [Governance Proposals](https://docs.uniswap.org/contracts/v3/guides/governance/liscense-modifications)
      * [Technical Reference](https://docs.uniswap.org/contracts/v3/reference/overview)
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
  * v3 Protocol
  * Guides
  * [Set Up Your Local Environment](https://docs.uniswap.org/contracts/v3/guides/local-environment)


On this page
One of the most common questions we get asked is what development toolset to use to build on-chain integrations with Uniswap. There’s no right answer to this question but for this guide we’ll recommend a common one: `Node.js` , `NPM` and `Hardhat`.
At the end of this guide you’ll have a development environment set up that you can use to build the rest of the examples in the Guides section of the docs, or start your own integration project!
To get you started as quickly as possible, we have provided the `Quick Start` section below where you can clone some boiler plate and get building. To start from scratch and learn the underlying concepts, jump to the `Start from Scratch` section.
# Quick Start
The Uniswap [boilerplate repo](https://github.com/Uniswap/uniswap-first-contract-example) provides a basic Hardhat environment with required imports already pre-loaded for you. You can simply clone it and install the dependencies:
```
git clone https://github.com/Uniswap/uniswap-first-contract-examplecd uniswap-first-contract-examplenpm install
```

Then hop to the `Local Node with a Mainnet Fork` to complete your set up and start developing.
# Start from Scratch
In the following sections, we’ll walk through the steps to create the same environment set up as the boiler plate from scratch and learn the underlying concepts.
## Set Up Dependencies[​](https://docs.uniswap.org/contracts/v3/guides/local-environment#set-up-dependencies "Direct link to Set Up Dependencies")
Node is one of the most common Javascript runtimes. For our purposes it will provide scripting we can use to compile and test our contracts. If you haven’t already, install NodeJS and its package manager NPM ([instructions](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)). Once those dependencies are set up, we can initialize our project:
```
$ npm init
```

[Hardhat](https://hardhat.org/) is an Ethereum development toolset that provides a number of powerful features including Solidity compilation, testing and deployment, all in a single convenient wrapper. We’ll use NPM to add Hardhat to our project:
```
$ npm add --save-dev hardhat
```

With Hardhat installed we can invoke it to scaffold our development environment. When you first run Hardhat you’ll have the option of starting with a templated Javascript or Typescript project or an empty project. Since Hardhat relies heavily on folder structure, we recommend starting with either of the templated options. Initialize Hardhat and follow the prompts to make your selection and answer yes to the follow up prompts:
```
$ npx hardhat init
```

Once the Hardhat initialization completes, take a look around at what got set up. The folder structure should be intuitive, `./contracts` is where you’ll write your Solidity contracts, `./test` is where you’ll write your tests and `./scripts` is where you can write scripts to perform actions like deploying. Out of the box, Hardhat is configured to use this folder structure so don’t change it unless you know what you’re doing!
Next we’ll use NPM to add the Uniswap V3 contracts which will allow us to seamlessly integrate with the protocol in our new contracts:
```
$ npm add @uniswap/v3-periphery @uniswap/v3-core
```

The Uniswap V3 contracts were written using a past version of the solidity compiler. Since we’re building integrations on V3 we have to tell Hardhat to use the correct compiler to build these files. Go to the `./hardhat.config.js` file and change the Solidity version to “0.7.6”:
```
// ...module.exports={solidity:"0.7.6",};
```

That’s it! You should now have a functional development environment to start building on chain Uniswap integrations. Let’s run a quick test to confirm everything is set up properly.
## Compile a Basic Contract[​](https://docs.uniswap.org/contracts/v3/guides/local-environment#compile-a-basic-contract "Direct link to Compile a Basic Contract")
To confirm that our environment is configured correctly we’ll attempt to compile a basic Swap contract. Create a new file, `./contracts/Swap.sol` and paste the following code into it (a detailed guide to this contract can be found [here](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)):
```
// SPDX-License-Identifier: GPL-2.0-or-laterpragma solidity =0.7.6;pragma abicoder v2;import'@uniswap/v3-periphery/contracts/interfaces/ISwapRouter.sol';import'@uniswap/v3-periphery/contracts/libraries/TransferHelper.sol';contract SimpleSwap{ISwapRouterpublic immutable swapRouter;  address public constant DAI=0x6B175474E89094C44Da98b954EedeAC495271d0F;  address public constant WETH9=0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2;  uint24 public constant feeTier =3000;constructor(ISwapRouter _swapRouter){    swapRouter = _swapRouter;}functionswapWETHForDAI(uint256 amountIn) external returns(uint256 amountOut){// Transfer the specified amount of WETH9 to this contract.TransferHelper.safeTransferFrom(WETH9, msg.sender,address(this), amountIn);// Approve the router to spend WETH9.TransferHelper.safeApprove(WETH9,address(swapRouter), amountIn);// Note: To use this example, you should explicitly set slippage limits, omitting for simplicity    uint256 minOut =/* Calculate min output */0;    uint160 priceLimit =/* Calculate price limit */0;// Create the params that will be used to execute the swapISwapRouter.ExactInputSingleParams memory params =ISwapRouter.ExactInputSingleParams({tokenIn:WETH9,tokenOut:DAI,fee: feeTier,recipient: msg.sender,deadline: block.timestamp,amountIn: amountIn,amountOutMinimum: minOut,sqrtPriceLimitX96: priceLimit});// The call to `exactInputSingle` executes the swap.    amountOut = swapRouter.exactInputSingle(params);}}
```

To compile all the contracts in the `./contracts` folder, we’ll use the Hardhat compile command:
```
$ npx hardhat compile
```

If the environment is compiled correctly you should see the message:
```
Compiled { x } Solidity files successfully
```

# Local Node with a Mainnet Fork
When building and testing integrations with on chain protocols, developers often hit a problem: the liquidity on the live chain is critical to thoroughly testing their code but testing against a live network like Mainnet can be extremely expensive.
See [the SDK getting started guide](https://docs.uniswap.org/sdk/v3/guides/local-development) for a full example on how to use forks.
With your local node up and running, you can use the `--network localhost` flag in tests to point the Hardhat testing suite to that local node:
```
$ npx hardhat test --network localhost
```

# Next Steps
With your environment set up you’re ready to start building. Jump over to the guides section to learn about the Uniswap functions you can integrate with. Remember to add all contracts (.sol files) to the `./contracts` folder and their subsequent tests to the `./tests` folder. You can then test them against your local forked node by running:
```
$ npx hardhat test --network localhost
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/local-environment.mdx)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/v3/overview)[NextSingle Swaps](https://docs.uniswap.org/contracts/v3/guides/swaps/single-swaps)
On this page
  * [Set Up Dependencies](https://docs.uniswap.org/contracts/v3/guides/local-environment#set-up-dependencies)
  * [Compile a Basic Contract](https://docs.uniswap.org/contracts/v3/guides/local-environment#compile-a-basic-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v3/guides/local-environment.mdx)
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
