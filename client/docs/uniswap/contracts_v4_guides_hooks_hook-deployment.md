# https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment

[Skip to main content](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
      * [Quickstart](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
        * [Hooks](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
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
      * [Technical Reference](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [UniswapX](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [Universal Router](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [Permit2](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v4 Protocol
  * Guides
  * Hooks
  * [Hook Deployment](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment)


On this page
# Hook Deployment
## Hook Flags[​](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#hook-flags "Direct link to Hook Flags")
As mentioned in [Concept of Hooks](https://docs.uniswap.org/contracts/v4/concepts/hooks), hook contracts indicate their implemented functions by **encoding its behavior in the address of the contract**. The `PoolManager` uses these permissions to determine which hook functions to call for a given pool. See `PoolManager` deployment addresses [here](https://docs.uniswap.org/contracts/v4/deployments).
Each hook function e.g. `beforeSwap` - corresponds to a certain _flag_. For example, the `beforeSwap` function is correlated to the [`BEFORE_SWAP_FLAG`](https://github.com/Uniswap/v4-core/blob/main/src/libraries/Hooks.sol#L37) which has a value of `1 << 7`.
These flags represent specific bits in the address of the hook smart contract - and the value of the bit (a one or a zero) represents whether that flag is true or false. An example:
Addresses on Ethereum are 20 bytes long (160 bits). So for example the address:
```
0x00000000000000000000000000000000000000C0
```

represented in binary is:
```
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000011000000
```

In binary it goes from right-to-left - so the trailing 8 bits of this address are `1100 0000` where:
1st Bit to 6th Bit = `0`
7th Bit and 8th Bit = `1`
The `AFTER_SWAP` flag is represented by the 7th bit - which is set to `1` for the example contract address. In the `PoolManager` swap execution flow, it will observe the flag and make a call to the hook's `afterSwap` function.
Similarly, the 8th bit which is also a `1`, actually corresponds to the `BEFORE_SWAP` i.e. the `beforeSwap` hook function - which will also be called by the `PoolManager` during a `swap` workflow.
A full list of all flags can be found [here](https://github.com/Uniswap/v4-core/blob/main/src/libraries/Hooks.sol).
## Hook Miner[​](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#hook-miner "Direct link to Hook Miner")
Because of encoded addresses, hook developers must _mine_ an address to a their particular pattern.
For local testing, `deployCodeTo` cheatcode in Foundry can be used to deploy hook contract to any address.
But when deploying hooks to an actual network, address mining is required to find the proper deployment address There is a helper library [`HookMiner.sol`](https://github.com/Uniswap/v4-periphery/blob/main/src/utils/HookMiner.sol) that can be used to mine for correct addresses.
Let's see it in action for a Foundry script. We will make use of the example - Points Hook from [Building Your First Hook](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook).
First we set up the contract for Foundry script and import the relevant dependencies:
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.19;import"forge-std/Script.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{HookMiner}from"v4-periphery/src/utils/HookMiner.sol";import{Constants}from"./base/Constants.sol";import{PointsHook}from"../src/PointsHook.sol";/// @notice Mines the address and deploys the PointsHook.sol Hook contractcontractPointsHookScriptis Script, Constants {functionsetUp()public{}functionrun()public{
```

Specify the flags needed to be encoded in the address:
```
uint160 flags =uint160(  Hooks.AFTER_ADD_LIQUIDITY_FLAG | Hooks.AFTER_SWAP_FLAG);
```

Mine the address by finding a `salt` that produces a hook address with the desired `flags`, use the Foundry deterministic deployer when deploying via Foundry script. For most chains, CREATE2_DEPLOYER contract address is [0x4e59b44847b379578588920ca78fbf26c0b4956c](https://book.getfoundry.sh/guides/deterministic-deployments-using-create2#getting-started).
```
bytesmemory constructorArgs = abi.encode(POOLMANAGER);(address hookAddress,bytes32 salt)=  HookMiner.find(CREATE2_DEPLOYER, flags,type(PointsHook).creationCode, constructorArgs);
```

**CREATE2_DEPLOYER**
  * `CREATE2_DEPLOYER` is the address that will deploy the hook. In `forge test`, this will be the test contract `address(this)` or the pranking address.
  * In `forge script`, this should be `0x4e59b44847b379578588920cA78FbF26c0B4956C` (CREATE2 Deployer Proxy)


Refer to this for more details on deploying contracts with CREATE2: [Deploying Contracts with CREATE2](https://docs.openzeppelin.com/cli/2.8/deploying-with-create2)
Deploy the hook using CREATE2 with the `salt`, and compare the deployed address with the address mined:
```
vm.broadcast();PointsHook pointsHook =newPointsHook{salt: salt}(IPoolManager(POOLMANAGER));require(address(pointsHook)== hookAddress,"PointsHookScript: hook address mismatch");
```

## A Complete Foundry Script Contract[​](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#a-complete-foundry-script-contract "Direct link to A Complete Foundry Script Contract")
```
// SPDX-License-Identifier: MITpragmasolidity^0.8.19;import"forge-std/Script.sol";import{Hooks}from"v4-core/src/libraries/Hooks.sol";import{IPoolManager}from"v4-core/src/interfaces/IPoolManager.sol";import{HookMiner}from"v4-periphery/src/utils/HookMiner.sol";import{Constants}from"./base/Constants.sol";import{PointsHook}from"../src/PointsHook.sol";/// @notice Mines the address and deploys the PointsHook.sol Hook contractcontractPointsHookScriptis Script, Constants {functionsetUp()public{}functionrun()public{// hook contracts must have specific flags encoded in the addressuint160 flags =uint160(      Hooks.BEFORE_SWAP_FLAG | Hooks.AFTER_SWAP_FLAG | Hooks.BEFORE_ADD_LIQUIDITY_FLAG| Hooks.BEFORE_REMOVE_LIQUIDITY_FLAG);// Mine a salt that will produce a hook address with the correct flagsbytesmemory constructorArgs = abi.encode(POOLMANAGER);(address hookAddress,bytes32 salt)=      HookMiner.find(CREATE2_DEPLOYER, flags,type(PointsHook).creationCode, constructorArgs);// Deploy the hook using CREATE2    vm.broadcast();    PointsHook pointsHook =newPointsHook{salt: salt}(IPoolManager(POOLMANAGER));require(address(pointsHook)== hookAddress,"PointsHookScript: hook address mismatch");}}
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/hooks/05-hook-deployment.mdx)
Was this helpful?
[PreviousBuilding Your First Hook](https://docs.uniswap.org/contracts/v4/guides/hooks/your-first-hook)[NextUnlock Callback & Deltas](https://docs.uniswap.org/contracts/v4/guides/unlock-callback)
On this page
  * [Hook Flags](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#hook-flags)
  * [Hook Miner](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#hook-miner)
  * [A Complete Foundry Script Contract](https://docs.uniswap.org/contracts/v4/guides/hooks/hook-deployment#a-complete-foundry-script-contract)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v4/guides/hooks/05-hook-deployment.mdx)
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
