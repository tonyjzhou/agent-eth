# https://docs.uniswap.org/contracts/universal-router/technical-reference

[Skip to main content](https://docs.uniswap.org/contracts/universal-router/technical-reference#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/universal-router/technical-reference)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/universal-router/technical-reference)
      * [Quickstart](https://docs.uniswap.org/contracts/universal-router/technical-reference)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/universal-router/technical-reference)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/universal-router/technical-reference)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/universal-router/technical-reference)
      * [Technical Reference](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [v3 Protocol](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [Smart Wallet](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [UniswapX](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/technical-reference)
      * [Overview](https://docs.uniswap.org/contracts/universal-router/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [Permit2](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [v2 Protocol](https://docs.uniswap.org/contracts/universal-router/technical-reference)
    * [v1 Protocol](https://docs.uniswap.org/contracts/universal-router/technical-reference)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Universal Router
  * [Technical Reference](https://docs.uniswap.org/contracts/universal-router/technical-reference)


On this page
# Technical Reference
## Functions[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#functions "Direct link to Functions")
Transactions to the `UniversalRouter` all go through the `execute` function:
  * `execute(bytes calldata commands, bytes[] calldata inputs, uint256 deadline)`
  * `execute(bytes calldata commands, bytes[] calldata inputs)`


Both functions behave and process the commands exactly the same, the first one with a deadline. The function without the deadline parameter will not revert based on block.timestamp.
The `execute` function behaves like a minimal virtual machine. It interprets a list of 1-byte commands and their corresponding ABI-encoded inputs and executes them sequentially.
## Command Structure[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#command-structure "Direct link to Command Structure")
Each command byte uses the following bit structure:
0| 1 2| 3 4 5 6 7  
---|---|---  
f| r| command  
### `f`[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#f "Direct link to f")
A single bit flag, that signals whether or not the command should be allowed to revert without the whole transaction failing.
  * If `f` is `0` aka `false` and the command reverts, then the entire transaction will revert and none of the commands will be executed.
  * If `f` is `1` aka `true` and the command reverts, then the transaction will continue, allowing us to achieve partial fills. If using this flag, be careful to include further commands that will remove any funds that could be left unused in the `UniversalRouter` contract.


### `r`[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#r "Direct link to r")
2 unused bytes, reserved for future use. Leaving these 2 bits as `0` will save gas, but any value passed into the contract will be ignored. Later versions of the `UniversalRouter` will likely expand the 5 bits used for `command` to use at least 1 of these bits.
### `command`[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#command "Direct link to command")
A 5 bit unique identifier for the command that should be carried out. The values of these commands can be found within [Commands.sol](https://github.com/Uniswap/universal-router/blob/main/contracts/libraries/Commands.sol), or can be viewed in the table below.
The command types that are not defined do not have an assigned command at this moment in time. Providing one of these identifiers will cause the transaction to revert with `InvalidCommandType`.
## v2 vs v1 Overview[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#v2-vs-v1-overview "Direct link to v2 vs v1 Overview")
**Note:** For details on the previous version, see [Universal Router (Legacy)](https://docs.uniswap.org/universal-router-legacy/overview).
Feature| v1| v2  
---|---|---  
NFT support| ✅ Multiple marketplaces| ❌ Removed  
v4 pool interaction| ❌ Not supported| ✅ `V4_SWAP`, `V4_POSITION_MANAGER_CALL`  
v3/v4 position management| ❌ Not supported| ✅ `V3_POSITION_MANAGER_*` / `V4_*`  
Commands| 0x00–0x3f (dense NFT ops)| 0x00–0x21 (compact core logic)  
Permit2-based transfers| ✅| ✅ Extended with batch & position flows  
Sub-plan execution| ✅ `EXECUTE_SUB_PLAN`| ✅ Still supported  
## Supported Commands (v2)[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#supported-commands-v2 "Direct link to Supported Commands \(v2\)")
Command| Name  
---|---  
`0x00`| `V3_SWAP_EXACT_IN`  
`0x01`| `V3_SWAP_EXACT_OUT`  
`0x02`| `PERMIT2_TRANSFER_FROM`  
`0x03`| `PERMIT2_PERMIT_BATCH`  
`0x04`| `SWEEP`  
`0x05`| `TRANSFER`  
`0x06`| `PAY_PORTION`  
`0x08`| `V2_SWAP_EXACT_IN`  
`0x09`| `V2_SWAP_EXACT_OUT`  
`0x0a`| `PERMIT2_PERMIT`  
`0x0b`| `WRAP_ETH`  
`0x0c`| `UNWRAP_WETH`  
`0x0d`| `PERMIT2_TRANSFER_FROM_BATCH`  
`0x0e`| `BALANCE_CHECK_ERC20`  
`0x10`| `V4_SWAP`  
`0x11`| `V3_POSITION_MANAGER_PERMIT`  
`0x12`| `V3_POSITION_MANAGER_CALL`  
`0x13`| `V4_INITIALIZE_POOL`  
`0x14`| `V4_POSITION_MANAGER_CALL`  
`0x21`| `EXECUTE_SUB_PLAN`  
Commands not listed are placeholders and will revert if called.
## Command Inputs[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#command-inputs "Direct link to Command Inputs")
Each command requires its own input structure. Inputs are encoded using `abi.encode(...)` and placed in `inputs[i]` to match `commands[i]`. For example:
## Swap Commands[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#swap-commands "Direct link to Swap Commands")
### `0x00` – V3_SWAP_EXACT_IN[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x00--v3_swap_exact_in "Direct link to 0x00--v3_swap_exact_in")
**Parameters:**
  * `address recipient`
  * `uint256 amountIn`
  * `uint256 amountOutMin`
  * `bytes path`
  * `bool payerIsUser`


**Calls:** `v3SwapExactInput(...)` in V3SwapModule **Usage:** Ideal for deterministic trades where the input amount is fixed.
### `0x01` – V3_SWAP_EXACT_OUT[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x01--v3_swap_exact_out "Direct link to 0x01--v3_swap_exact_out")
**Parameters:**
  * `address recipient`
  * `uint256 amountOut`
  * `uint256 amountInMax`
  * `bytes path`
  * `bool payerIsUser`


**Calls:** `v3SwapExactOutput(...)` **Usage:** When the user wants to receive a precise amount of output tokens, regardless of price volatility, within a max budget.
### `0x08` – V2_SWAP_EXACT_IN[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x08--v2_swap_exact_in "Direct link to 0x08--v2_swap_exact_in")
**Parameters:**
  * `address recipient`
  * `uint256 amountIn`
  * `uint256 amountOutMin`
  * `address[] path`
  * `bool payerIsUser`


**Calls:** `v2SwapExactInput(...)` in V2SwapModule **Usage:** Simple Uniswap v2-style fixed input swap using token pairs.
### `0x09` – V2_SWAP_EXACT_OUT[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x09--v2_swap_exact_out "Direct link to 0x09--v2_swap_exact_out")
**Parameters:**
  * `address recipient`
  * `uint256 amountOut`
  * `uint256 amountInMax`
  * `address[] path`
  * `bool payerIsUser`


**Calls:** `v2SwapExactOutput(...)` **Usage:** Swaps to get an exact output amount with limited token budget.
## Permit2 Commands[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#permit2-commands "Direct link to Permit2 Commands")
### `0x02` – PERMIT2_TRANSFER_FROM[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x02--permit2_transfer_from "Direct link to 0x02--permit2_transfer_from")
**Parameters:**
  * `address token`
  * `address recipient`
  * `uint160 amount`


**Calls:** `permit2TransferFrom(...)` **Usage:** Transfers a single token using Permit2 allowances. Always pulls from `msg.sender`.
### `0x03` – PERMIT2_PERMIT_BATCH[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x03--permit2_permit_batch "Direct link to 0x03--permit2_permit_batch")
**Parameters:**
  * `PermitBatch permitBatch`
  * `bytes signature`


**Calls:** `PERMIT2.permit(...)` **Usage:** Sets approval for multiple tokens in one signature.
### `0x0a` – PERMIT2_PERMIT[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0a--permit2_permit "Direct link to 0x0a--permit2_permit")
**Parameters:**
  * `PermitSingle permitSingle`
  * `bytes signature`


**Calls:** `PERMIT2.permit(...)` **Usage:** Sets approval for one token, often before `PERMIT2_TRANSFER_FROM`.
### `0x0d` – PERMIT2_TRANSFER_FROM_BATCH[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0d--permit2_transfer_from_batch "Direct link to 0x0d--permit2_transfer_from_batch")
**Parameters:**
  * `AllowanceTransferDetails[] batch`


**Calls:** `permit2TransferFrom(...)` **Usage:** Transfers many tokens in one call from a user to one or more destinations.
## Payment & Balance Commands[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#payment--balance-commands "Direct link to Payment & Balance Commands")
### `0x04` – SWEEP[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x04--sweep "Direct link to 0x04--sweep")
**Parameters:**
  * `address token`
  * `address recipient`
  * `uint256 amountMin`


**Calls:** `Payments.sweep(...)` **Usage:** Clears out all router-held ETH or ERC20 tokens to a destination address.
### `0x05` – TRANSFER[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x05--transfer "Direct link to 0x05--transfer")
**Parameters:**
  * `address token`
  * `address recipient`
  * `uint256 amount`


**Calls:** `Payments.pay(...)` **Usage:** Transfers a specific amount (not full balance) from the router.
### `0x06` – PAY_PORTION[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x06--pay_portion "Direct link to 0x06--pay_portion")
**Parameters:**
  * `address token`
  * `address recipient`
  * `uint256 bips`


**Calls:** `Payments.payPortion(...)` **Usage:** Sends a % of the token balance (e.g., 2500 = 25%).
### `0x0e` – BALANCE_CHECK_ERC20[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0e--balance_check_erc20 "Direct link to 0x0e--balance_check_erc20")
**Parameters:**
  * `address owner`
  * `address token`
  * `uint256 minBalance`


**Calls:** view-only `balanceOf(...)` **Usage:** Ensures required token balance exists; useful for conditional workflows.
## ETH & WETH[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#eth--weth "Direct link to ETH & WETH")
### `0x0b` – WRAP_ETH[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0b--wrap_eth "Direct link to 0x0b--wrap_eth")
**Parameters:**
  * `address recipient`
  * `uint256 amount`


**Calls:** `Payments.wrapETH(...)` → WETH.deposit() **Usage:** Converts ETH held by router into WETH and optionally sends it.
### `0x0c` – UNWRAP_WETH[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0c--unwrap_weth "Direct link to 0x0c--unwrap_weth")
**Parameters:**
  * `address recipient`
  * `uint256 amountMin`


**Calls:** `Payments.unwrapWETH9(...)` **Usage:** Converts all router-held WETH into ETH and sends it.
## v3 & v4 Advanced[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#v3--v4-advanced "Direct link to v3 & v4 Advanced")
## `0x10` – V4_SWAP[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x10--v4_swap "Direct link to 0x10--v4_swap")
### Parameters:[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#parameters "Direct link to Parameters:")
  * **`bytes actions`**Encoded action identifiers specifying the type of swap or payment action. For available action types, see[Uniswap v4 SDK Actions](https://docs.uniswap.org/sdk/v4/reference/enumerations/Actions).
  * **`bytes[] params`**ABI-encoded parameters array, corresponding one-to-one with each action provided in`actions`. Each action type requires its own parameter structure.


### Calls:[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#calls "Direct link to Calls:")
Executes actions via `V4SwapRouter._handleAction(action, params)`:
  * Swap-related actions call `_swapExactInput(...)` or `_swapExactOutput(...)`.
  * Payment-related actions (`settle`, `take`) call internal balance management methods (`_settle(...)`, `_take(...)`).
  * Swap actions ultimately call `_swap(...)`, executing swaps via `PoolManager.swap(...)`.


**Usage:** Executes a swap on Uniswap v4 using the provided parameters.
### Internal Flow:[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#internal-flow "Direct link to Internal Flow:")
```
UniversalRouter.execute(...) receives command `0x10`↓ dispatch (UniversalRouter.sol)V4SwapRouter.\_handleAction(action, params)├── SWAP_EXACT_IN → \_swapExactInput(...)├── SWAP_EXACT_OUT → \_swapExactOutput(...)├── SETTLE / SETTLE_ALL → \_settle(...)├── TAKE / TAKE_ALL / TAKE_PORTION → \_take(...)↓ swap calls route to\_swap(...) → PoolManager.swap(...)
```

### `0x11` – V3_POSITION_MANAGER_PERMIT[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x11--v3_position_manager_permit "Direct link to 0x11--v3_position_manager_permit")
**Parameters:**
  * `address spender`
  * `uint256 tokenId`
  * `uint256 deadline`
  * `uint8 v, bytes32 r, bytes32 s`


**Calls:** NonfungiblePositionManager.permit(...) **Usage:** Grants router permission to operate on a user’s v3 NFT.
### `0x12` – V3_POSITION_MANAGER_CALL[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x12--v3_position_manager_call "Direct link to 0x12--v3_position_manager_call")
**Parameters:**
  * `bytes callData`


**Calls:** Arbitrary call to NonfungiblePositionManager **Usage:** Executes v3 NFT ops like `burn`, `collect`, `decreaseLiquidity`.
### `0x13` – V4_INITIALIZE_POOL[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x13--v4_initialize_pool "Direct link to 0x13--v4_initialize_pool")
**Parameters:**
  * `PoolKey key`
  * `uint160 sqrtPriceX96`


**Calls:** `PoolManager.initialize(...)` **Usage:** Creates new V4 pool with specified fee, tick spacing, etc.
### `0x14` – V4_POSITION_MANAGER_CALL[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x14--v4_position_manager_call "Direct link to 0x14--v4_position_manager_call")
**Parameters:**
  * `bytes callData`


**Calls:** Arbitrary call to v4 PositionManager **Usage:** Used for `modifyLiquidity`, `mint`, `settle`, etc. on a pool.
## Composability[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#composability "Direct link to Composability")
### `0x21` – EXECUTE_SUB_PLAN[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x21--execute_sub_plan "Direct link to 0x21--execute_sub_plan")
**Parameters:**
  * `bytes subCommands`
  * `bytes[] subInputs`


**Calls:** `execute(...)` (reentrantly)
**Usage:** Nested command execution for conditional or fallback logic. Used to group steps or allow selective reverts (via `f` flag).
## Reverting Command Example[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#reverting-command-example "Direct link to Reverting Command Example")
To allow a command to fail without reverting the entire transaction, set the high bit:
```
command =0x80|0x00;// V3_SWAP_EXACT_IN with ALLOW_REVERT
```

Be sure to follow such commands with cleanup logic (e.g., `SWEEP`) to handle unused ETH or tokens.
## References[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#references "Direct link to References")
  * [Uniswap Universal Router GitHub](https://github.com/Uniswap/universal-router)
  * [Latest Commands.sol](https://github.com/Uniswap/universal-router/blob/main/contracts/libraries/Commands.sol)


## Legacy Documentation[​](https://docs.uniswap.org/contracts/universal-router/technical-reference#legacy-documentation "Direct link to Legacy Documentation")
  * [Universal Router (Legacy) Overview](https://docs.uniswap.org/universal-router-legacy/overview)
  * [Universal Router (Legacy) Technical Reference](https://docs.uniswap.org/universal-router-legacy/technical-reference)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/universal-router/02-technical-reference.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/universal-router/overview)[NextOverview](https://docs.uniswap.org/contracts/permit2/overview)
On this page
  * [Functions](https://docs.uniswap.org/contracts/universal-router/technical-reference#functions)
  * [Command Structure](https://docs.uniswap.org/contracts/universal-router/technical-reference#command-structure)
    * [`f`](https://docs.uniswap.org/contracts/universal-router/technical-reference#f)
    * [`r`](https://docs.uniswap.org/contracts/universal-router/technical-reference#r)
    * [`command`](https://docs.uniswap.org/contracts/universal-router/technical-reference#command)
  * [v2 vs v1 Overview](https://docs.uniswap.org/contracts/universal-router/technical-reference#v2-vs-v1-overview)
  * [Supported Commands (v2)](https://docs.uniswap.org/contracts/universal-router/technical-reference#supported-commands-v2)
  * [Command Inputs](https://docs.uniswap.org/contracts/universal-router/technical-reference#command-inputs)
  * [Swap Commands](https://docs.uniswap.org/contracts/universal-router/technical-reference#swap-commands)
    * [`0x00` – V3_SWAP_EXACT_IN](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x00--v3_swap_exact_in)
    * [`0x01` – V3_SWAP_EXACT_OUT](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x01--v3_swap_exact_out)
    * [`0x08` – V2_SWAP_EXACT_IN](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x08--v2_swap_exact_in)
    * [`0x09` – V2_SWAP_EXACT_OUT](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x09--v2_swap_exact_out)
  * [Permit2 Commands](https://docs.uniswap.org/contracts/universal-router/technical-reference#permit2-commands)
    * [`0x02` – PERMIT2_TRANSFER_FROM](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x02--permit2_transfer_from)
    * [`0x03` – PERMIT2_PERMIT_BATCH](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x03--permit2_permit_batch)
    * [`0x0a` – PERMIT2_PERMIT](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0a--permit2_permit)
    * [`0x0d` – PERMIT2_TRANSFER_FROM_BATCH](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0d--permit2_transfer_from_batch)
  * [Payment & Balance Commands](https://docs.uniswap.org/contracts/universal-router/technical-reference#payment--balance-commands)
    * [`0x04` – SWEEP](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x04--sweep)
    * [`0x05` – TRANSFER](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x05--transfer)
    * [`0x06` – PAY_PORTION](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x06--pay_portion)
    * [`0x0e` – BALANCE_CHECK_ERC20](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0e--balance_check_erc20)
  * [ETH & WETH](https://docs.uniswap.org/contracts/universal-router/technical-reference#eth--weth)
    * [`0x0b` – WRAP_ETH](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0b--wrap_eth)
    * [`0x0c` – UNWRAP_WETH](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x0c--unwrap_weth)
  * [v3 & v4 Advanced](https://docs.uniswap.org/contracts/universal-router/technical-reference#v3--v4-advanced)
  * [`0x10` – V4_SWAP](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x10--v4_swap)
    * [Parameters:](https://docs.uniswap.org/contracts/universal-router/technical-reference#parameters)
    * [Calls:](https://docs.uniswap.org/contracts/universal-router/technical-reference#calls)
    * [Internal Flow:](https://docs.uniswap.org/contracts/universal-router/technical-reference#internal-flow)
    * [`0x11` – V3_POSITION_MANAGER_PERMIT](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x11--v3_position_manager_permit)
    * [`0x12` – V3_POSITION_MANAGER_CALL](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x12--v3_position_manager_call)
    * [`0x13` – V4_INITIALIZE_POOL](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x13--v4_initialize_pool)
    * [`0x14` – V4_POSITION_MANAGER_CALL](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x14--v4_position_manager_call)
  * [Composability](https://docs.uniswap.org/contracts/universal-router/technical-reference#composability)
    * [`0x21` – EXECUTE_SUB_PLAN](https://docs.uniswap.org/contracts/universal-router/technical-reference#0x21--execute_sub_plan)
  * [Reverting Command Example](https://docs.uniswap.org/contracts/universal-router/technical-reference#reverting-command-example)
  * [References](https://docs.uniswap.org/contracts/universal-router/technical-reference#references)
  * [Legacy Documentation](https://docs.uniswap.org/contracts/universal-router/technical-reference#legacy-documentation)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/universal-router/02-technical-reference.md)
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
