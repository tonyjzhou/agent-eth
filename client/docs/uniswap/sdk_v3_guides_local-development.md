# https://docs.uniswap.org/sdk/v3/guides/local-development

[Skip to main content](https://docs.uniswap.org/sdk/v3/guides/local-development#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
    * [v4 SDK](https://docs.uniswap.org/sdk/v3/guides/local-development)
      * [Overview](https://docs.uniswap.org/sdk/v4/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Position Management](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/local-development)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/local-development)
    * [v3 SDK](https://docs.uniswap.org/sdk/v3/guides/local-development)
      * [Overview](https://docs.uniswap.org/sdk/v3/overview)
      * [Guides](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Background](https://docs.uniswap.org/sdk/v3/guides/background)
        * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Web3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
        * [Swaps](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Pooling Liquidity](https://docs.uniswap.org/sdk/v3/guides/local-development)
        * [Advanced](https://docs.uniswap.org/sdk/v3/guides/local-development)
      * [Technical Reference](https://docs.uniswap.org/sdk/v3/guides/local-development)
    * [Swap Widget](https://docs.uniswap.org/sdk/v3/guides/local-development)
    * [web3-react](https://docs.uniswap.org/sdk/v3/guides/local-development)
    * [Core SDK](https://docs.uniswap.org/sdk/v3/guides/local-development)
    * [v2 SDK](https://docs.uniswap.org/sdk/v3/guides/local-development)
    * [v1 SDK](https://docs.uniswap.org/sdk/v3/guides/local-development)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * v3 SDK
  * Guides
  * [Local Development](https://docs.uniswap.org/sdk/v3/guides/local-development)


On this page
# Local Development
## Introduction[​](https://docs.uniswap.org/sdk/v3/guides/local-development#introduction "Direct link to Introduction")
Developing your dApps or smart contracts requires some tinkering to get a proper setup that is both a good simulation of how Mainnet will behave, but also customizable enough to suit the needs of a development environment.
One very common approach is to create your own custom chain offline and develop on top of it. The issue with this approach is that if you are integrating with protocols like Uniswap or others that are on Mainnet, it's difficult to simulate on your local chain as the smart contracts from Mainnet are not there.
Another approach is to use a testnet like Ethereum Goerli. While most protocols (including Uniswap) have versions of their smart contracts deployed on common testnets, there are certain behavioural differences. Not all pools that are on Mainnet are on Goerli for example. Also, it's difficult to get enough testnet ETH to account for real testing. And without lots of testnet ETH it's even more difficult to swap to other coins on Uniswap, if that's what you need to do in your development environment.
This guide focuses on yet another approach to local development: Mainnet Forks.
A Mainnet Fork is a local chain that copies the state of Ethereum Mainnet at a given block number. It then gives you access to cheat codes like wallets with thousands of ETH and RPC URLs that you can use as drop-in replacements of real Mainnet RPCs.
This approach combines the best of all other approaches. You have a local chain that you can manipulate to your liking and you have real deployments of all the protocols you need to test and develop your dApp or smart contracts.
info
This guide focuses on Ethereum Mainnet. But you can easily fork any other chain by simply replacing the RPC URL with one of the network you want to use.
For this guide, the following packages are used:
  * [`@uniswap/v3-sdk`](https://www.npmjs.com/package/@uniswap/v3-sdk)
  * [`@uniswap/sdk-core`](https://www.npmjs.com/package/@uniswap/sdk-core)
  * [`ethers@5`](https://www.npmjs.com/package/ethers)


Please note that we use ethers version 5, as this is still the most commonly used version of ethers.js. If you use version 6, you will need to slightly change the syntax in the examples below.
info
Forking a chain requires archival data and trace calls. Infura and normal geth instances are by default not archival. You can get a free archival RPC that you can use to follow this guide and fork Mainnet you can visit [Chainnodes](https://www.chainnodes.org/).
## Using Foundry and Anvil[​](https://docs.uniswap.org/sdk/v3/guides/local-development#using-foundry-and-anvil "Direct link to Using Foundry and Anvil")
There are several developer tools to fork Mainnet. [Anvil](https://github.com/foundry-rs/foundry/tree/master/crates/anvil) by foundry is a newcomer that's fast and easy to setup. This guide focuses on Anvil.
As a first step, follow the [installation guide](https://book.getfoundry.sh/getting-started/installation) in the foundry book.
Once you have done that, you will be able to fork Mainnet straight away. Run the below command in your terminal:
Make sure that you:
  * Replace your API Key (get one by heading to [Chainnodes](https://app.chainnodes.org/))
  * Replace the block number with a recent one, check [Etherscan](https://etherscan.io/) for that
  * If you fork a non-Ethereum Mainnet chain, check [Chainlist](https://chainlist.org/) for the correct chain id and replace both occurrences in the command below


```
anvil --fork-url https://mainnet.chainnodes.org/api_key --fork-block-number 17480237 --fork-chain-id 1 --chain-id 1
```

Run `anvil --help` to see all available options.
Once you have done that, you should see something like the below:
![anvil result after calling](https://docs.uniswap.org/assets/images/anvil-result-cbdb52a1c3172c2644331ae2e3bf7869.png)
Your local fork of Mainnet is now running!
And as you can see on the screenshot above, anvil prints a bunch of private keys that are loaded with 10k ETH each. We will use them going forward to send transactions, including swaps on Uniswap pools.
warning
Security consideration: This is a fork of Mainnet and the same chain id is used. You have no replay protection to Ethereum Mainnet. So you need to 1: Never use the anvil private keys on a real chain or send funds to it (they are leaked everywhere) and 2: Not send any transactions to your local fork chain with accounts that you use on Ethereum Mainnet or other real chains.
If you scroll down in your terminal, near the bottom of the anvil logs you will find your RPC URL. If you haven't changed any configs, it should be `127.0.0.1:8545`. This is the RPC URL that you can now use as a drop-in replacement everywhere in your development environment, and interact with it as if it was real Ethereum Mainnet. You can use the http provider `http://127.0.0.1:8545` as well as the Websocket provider `ws://127.0.0.1:8545`.
You can now make a sample RPC request to your http provider using [Postman](https://www.postman.com/) using the below:
POST `http://127.0.0.1:8545`
Body:
```
{"jsonrpc":"2.0","method":"eth_chainId","params":[],"id":1}
```

The result should look like the below (see image below as well):
```
{"jsonrpc":"2.0","id":1,"result":"0x1"}
```

![anvil result after calling](https://docs.uniswap.org/assets/images/postman-chainid-result-6b21251b7d4659fd348e4e0ed4869e52.png)
As you can see, the chain id is `1`, just like on Mainnet!
You can find the above example and more in [this Postman workspace](https://www.postman.com/chainnodes/workspace/uniswap-examples) under "Local Development".
## Using your Mainnet Fork[​](https://docs.uniswap.org/sdk/v3/guides/local-development#using-your-mainnet-fork "Direct link to Using your Mainnet Fork")
Now that you have a running Mainnet Fork, you will be able to use it everywhere in your development setup. Using one of the private keys provided by anvil, you have access to enough ETH to do endless swaps and smart contract calls.
If you need any other token, you now have the flexibility of swapping your ETH to any token that has a pool deployed on Mainnet. You basically take your fake ETH and swap it to the token you need.
Check out one of the [guides about swapping](https://docs.uniswap.org/sdk/v3/guides/swaps/trading) and replace the RPC URL with your local anvil HTTP link as above.
## Next Steps[​](https://docs.uniswap.org/sdk/v3/guides/local-development#next-steps "Direct link to Next Steps")
Using the above you are fully equipped to continue following the guides about how to use Uniswap while testing everything locally before going on Mainnet. You will also be able to reuse what you have learned when you develop your own protocols or dApps.
You can also continue tapping into other developer tools that make smart contract development easier like [forge](https://github.com/foundry-rs/foundry/tree/master/crates/forge) from foundry. To read more about foundry and their developer tooling, visit their [GitHub](https://github.com/foundry-rs/foundry) or the [foundry book](https://book.getfoundry.sh/).
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/02-local-development.md)
Was this helpful?
[PreviousBackground](https://docs.uniswap.org/sdk/v3/guides/background)[NextWeb3 Development Basics](https://docs.uniswap.org/sdk/v3/guides/web3-development-basics)
On this page
  * [Introduction](https://docs.uniswap.org/sdk/v3/guides/local-development#introduction)
  * [Using Foundry and Anvil](https://docs.uniswap.org/sdk/v3/guides/local-development#using-foundry-and-anvil)
  * [Using your Mainnet Fork](https://docs.uniswap.org/sdk/v3/guides/local-development#using-your-mainnet-fork)
  * [Next Steps](https://docs.uniswap.org/sdk/v3/guides/local-development#next-steps)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/sdk/v3/guides/02-local-development.md)
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
