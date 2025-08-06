# https://docs.uniswap.org/universal-router-legacy/technical-reference

[Skip to main content](https://docs.uniswap.org/universal-router-legacy/technical-reference#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Universal Router (Legacy)](https://docs.uniswap.org/universal-router-legacy/technical-reference)
    * [Overview](https://docs.uniswap.org/universal-router-legacy/overview)
    * [Technical Reference](https://docs.uniswap.org/universal-router-legacy/technical-reference)


  * [Home](https://docs.uniswap.org/)
  * Universal Router (Legacy)
  * [Technical Reference](https://docs.uniswap.org/universal-router-legacy/technical-reference)


On this page
# Technical Reference
## Functions[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#functions "Direct link to Functions")
Transactions to the `UniversalRouter` all go through the `UniversalRouter.execute` functions:
  * `execute(bytes calldata commands, bytes[] calldata inputs, uint256 deadline)`
  * `execute(bytes calldata commands, bytes[] calldata inputs)`


The first of these functions adds the functionality to allow transactions to have a transaction deadline. If the `block.timestamp` is after the `deadline` provided the transaction will revert. After that check, the 2 functions otherwise execute identically.
The `execute` functions work like a simplified VM - they take in a list of commands, and a list of inputs for the commands and execute them in the order specified.
## Command Structure[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#command-structure "Direct link to Command Structure")
The first parameter for the function (`bytes calldata commands`) is a list of commands for the contract to execute, in the order they should be executed. Each command is encoded in 1 byte, containing the following split of 8 bits:
0| 1 2| 3 4 5 6 7  
---|---|---  
f| r| command  
### `f`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#f "Direct link to f")
A single bit flag, that signals whether or not the command should be allowed to revert without the whole transaction failing.
  * If `f` is `0` aka `false` and the command reverts, then the entire transaction will revert and none of the commands will be executed.
  * If `f` is `1` aka `true` and the command reverts, then the transaction will continue, allowing us to achieve partial fills. If using this flag, be careful to include further commands that will remove any funds that could be left unused in the `UniversalRouter` contract.


### `r`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#r "Direct link to r")
2 unused bytes, reserved for future use. Leaving these 2 bits as `0` will save gas, but any value passed into the contract will be ignored. Later versions of the `UniversalRouter` will likely expand the 5 bits used for `command` to use at least 1 of these bits.
### `command`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#command "Direct link to command")
A 5 bit unique identifier for the command that should be carried out. The values of these commands can be found within [Commands.sol](https://github.com/Uniswap/universal-router/blob/main/contracts/libraries/Commands.sol), or can be viewed in the table below.
The command types that are not defined do not have an assigned command at this moment in time. Providing one of these identifiers will cause the transaction to revert with `InvalidCommandType`.
A complete list of commands can be found in the table below:
Command| Value  
---|---  
`0x00`| [`V3_SWAP_EXACT_IN`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v3_swap_exact_in)  
`0x01`| [`V3_SWAP_EXACT_OUT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v3_swap_exact_out)  
`0x02`| [`PERMIT2_TRANSFER_FROM`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_transfer_from)  
`0x03`| [`PERMIT2_PERMIT_BATCH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_permit_batch)  
`0x04`| [`SWEEP`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep)  
`0x05`| [`TRANSFER`](https://docs.uniswap.org/universal-router-legacy/technical-reference#transfer)  
`0x06`| [`PAY_PORTION`](https://docs.uniswap.org/universal-router-legacy/technical-reference#pay_portion)  
`0x07`|   
`0x08`| [`V2_SWAP_EXACT_IN`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v2_swap_exact_in)  
`0x09`| [`V2_SWAP_EXACT_OUT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v2_swap_exact_out)  
`0x0a`| [`PERMIT2_PERMIT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_permit)  
`0x0b`| [`WRAP_ETH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#wrap_eth)  
`0x0c`| [`UNWRAP_WETH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#unwrap_weth)  
`0x0d`| [`PERMIT2_TRANSFER_FROM_BATCH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_transfer_from_batch)  
`0x0e`|   
`0x0f`|   
`0x10`| [`SEAPORT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#seaport)  
`0x11`| [`LOOKS_RARE_721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#looks_rare_721)  
`0x12`| [`NFTX`](https://docs.uniswap.org/universal-router-legacy/technical-reference#nftx)  
`0x13`| [`CRYPTOPUNKS`](https://docs.uniswap.org/universal-router-legacy/technical-reference#cryptopunks)  
`0x14`| [`LOOKS_RARE_1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#looks_rare_1155)  
`0x15`| [`OWNER_CHECK_721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#owner_check_721)  
`0x16`| [`OWNER_CHECK_1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#owner_check_1155)  
`0x17`| [`SWEEP_ERC721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep_erc721)  
`0x18`| [`X2Y2_721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#x2y2_721)  
`0x19`| [`SUDOSWAP`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sudoswap)  
`0x1a`| [`NFT20`](https://docs.uniswap.org/universal-router-legacy/technical-reference#nft20)  
`0x1b`| [`X2Y2_1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#x2y2_1155)  
`0x1c`| [`FOUNDATION`](https://docs.uniswap.org/universal-router-legacy/technical-reference#foundation)  
`0x1d`| [`SWEEP_ERC1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep_erc1155)  
`0x1e`|   
`0x1f`|   
## Command Inputs[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#command-inputs "Direct link to Command Inputs")
The second parameter for the function is an array of bytes strings. Each element in the array is the abi-encoded input that will be used for the respective command.
`commands[i]` is the command that will use `inputs[i]` as its encoded input parameters.
The router uses the command type to know how to decode the encoded input parameters - depending on the command chosen, the required inputs is different.
The input parameters required for each command are outlined below:
### `V3_SWAP_EXACT_IN`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#v3_swap_exact_in "Direct link to v3_swap_exact_in")
  * `address` The recipient of the output of the trade
  * `uint256` The amount of input tokens for the trade
  * `uint256` The minimum amount of output tokens the user wants
  * `bytes` The UniswapV3 encoded path to trade along
  * `bool` A flag for whether the input tokens should come from the `msg.sender` (through Permit2) or whether the funds are already in the `UniversalRouter`


### `V3_SWAP_EXACT_OUT`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#v3_swap_exact_out "Direct link to v3_swap_exact_out")
  * `address` The recipient of the output of the trade
  * `uint256` The amount of output tokens to receive
  * `uint256` The maximum number of input tokens that should be spent
  * `bytes` The UniswapV3 encoded path to trade along
  * `bool` A flag for whether the input tokens should come from the `msg.sender` (through Permit2) or whether the funds are already in the `UniversalRouter`


### `PERMIT2_TRANSFER_FROM`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_transfer_from "Direct link to permit2_transfer_from")
  * `address` The token to fetch from Permit2
  * `address` The recipient of the tokens fetched
  * `uint256` The amount of token to fetch


The individual that the tokens are fetched from is always the `msg.sender` of the transaction
### `PERMIT2_PERMIT_BATCH`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_permit_batch "Direct link to permit2_permit_batch")
  * `IAllowanceTransfer.PermitBatch` A `PermitBatch` struct outlining all of the Permit2 permits to execute.
  * `bytes` The signature to provide to Permit2


The individual that signed the permits must be the `msg.sender` of the transaction
### `SWEEP`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep "Direct link to sweep")
  * `address` The ERC20 token to sweep (or Constants.ETH for ETH)
  * `address` The recipient of the sweep
  * `uint256` The minimum required tokens to receive from the sweep


### `TRANSFER`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#transfer "Direct link to transfer")
  * `address` The ERC20 token to transfer (or Constants.ETH for ETH)
  * `address` The recipient of the transfer
  * `uint256` The amount to transfer


### `PAY_PORTION`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#pay_portion "Direct link to pay_portion")
  * `address` The ERC20 token to transfer (or Constants.ETH for ETH)
  * `address` The recipient of the transfer
  * `uint256` In basis points, the percentage of the contract’s balance to transfer


### `V2_SWAP_EXACT_IN`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#v2_swap_exact_in "Direct link to v2_swap_exact_in")
  * `address` The recipient of the output of the trade
  * `uint256` The amount of input tokens for the trade
  * `uint256` The minimum amount of output tokens the user wants
  * `address[]` The UniswapV2 token path to trade along
  * `bool` A flag for whether the input tokens should come from the `msg.sender` (through Permit2) or whether the funds are already in the `UniversalRouter`


### `V2_SWAP_EXACT_OUT`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#v2_swap_exact_out "Direct link to v2_swap_exact_out")
  * `address` The recipient of the output of the trade
  * `uint256` The amount of output tokens to receive
  * `uint256` The maximum number of input tokens that should be spent
  * `address[]` The UniswapV2 token path to trade along
  * `bool` A flag for whether the input tokens should come from the `msg.sender` (through Permit2) or whether the funds are already in the `UniversalRouter`


### `PERMIT2_PERMIT`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_permit "Direct link to permit2_permit")
  * `IAllowanceTransfer.PermitSingle` A `PermitSingle` struct outlining the Permit2 permit to execute
  * `bytes` The signature to provide to Permit2


The individual that signed the permit must be the `msg.sender` of the transaction
### `WRAP_ETH`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#wrap_eth "Direct link to wrap_eth")
  * `address` The recipient of the WETH
  * `uint256` The amount of ETH to wrap


### `UNWRAP_WETH`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#unwrap_weth "Direct link to unwrap_weth")
  * `address` The recipient of the ETH
  * `uint256` The minimum required ETH to receive from the unwrapping


### `PERMIT2_TRANSFER_FROM_BATCH`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_transfer_from_batch "Direct link to permit2_transfer_from_batch")
  * `IAllowanceTransfer.AllowanceTransferDetails[]` An array of `AllowanceTransferDetails` structs that each describe a Permit2 transfer to perform


### `SEAPORT`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#seaport "Direct link to seaport")
  * `uint256` The ETH value to forward to the Seaport contract
  * `bytes` The calldata to use to call the Seaport contract


### `LOOKS_RARE_721`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#looks_rare_721 "Direct link to looks_rare_721")
  * `uint256` The ETH value to forward to the LooksRare contract
  * `bytes` The calldata to use to call the LooksRare contract
  * `address` The recipient of the ERC721
  * `address` The ERC721 token address
  * `uint256` The ID of the ERC721


### `NFTX`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#nftx "Direct link to nftx")
  * `uint256` The ETH value to forward to the NFTX contract
  * `bytes` The calldata to use to call the NFTX contract


### `CRYPTOPUNKS`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#cryptopunks "Direct link to cryptopunks")
  * `uint256` The PunkID to purchase
  * `address` The recipient for the cryptopunk
  * `uint256` The ETH value to forward to the Cryptopunks contract


### `LOOKS_RARE_1155`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#looks_rare_1155 "Direct link to looks_rare_1155")
  * `uint256` The ETH value to forward to the LooksRare contract
  * `bytes` The calldata to use to call the LooksRare contract
  * `address` The recipient of the ERC1155
  * `address` The ERC1155 token address
  * `uint256` The ID of the ERC1155
  * `uint256` The amount of the ERC1155 to transfer


### `OWNER_CHECK_721`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#owner_check_721 "Direct link to owner_check_721")
  * `address` The required owner of the ERC721
  * `address` The ERC721 token address
  * `uint256` The ID of the ERC721


### `OWNER_CHECK_1155`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#owner_check_1155 "Direct link to owner_check_1155")
  * `address` The required owner of the ERC1155
  * `address` The ERC721 token address
  * `uint256` The ID of the ERC1155
  * `uint256` The minimum required amount of the ERC1155


### `SWEEP_ERC721`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep_erc721 "Direct link to sweep_erc721")
  * `address` The ERC721 token address to transfer
  * `address` The recipient of the transfer
  * `uint256` The token ID to transfer


### `X2Y2_721`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#x2y2_721 "Direct link to x2y2_721")
  * `uint256` The ETH value to forward to the X2Y2 contract
  * `bytes` The calldata to use to call the X2Y2 contract
  * `address` The recipient of the ERC721
  * `address` The ERC721 token address
  * `uint256` The ID of the ERC721


### `SUDOSWAP`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#sudoswap "Direct link to sudoswap")
  * `uint256` The ETH value to forward to the Sudoswap contract
  * `bytes` The calldata to use to call the Sudoswap contract


### `NFT20`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#nft20 "Direct link to nft20")
  * `uint256` The ETH value to forward to the NFT20 contract
  * `bytes` The calldata to use to call the NFT20 contract


### `X2Y2_1155`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#x2y2_1155 "Direct link to x2y2_1155")
  * `uint256` The ETH value to forward to the X2Y2 contract
  * `bytes` The calldata to use to call the X2Y2 contract
  * `address` The recipient of the ERC1155
  * `address` The ERC1155 token address
  * `uint256` The ID of the ERC1155
  * `uint256` The amount of the ERC1155 to transfer


### `FOUNDATION`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#foundation "Direct link to foundation")
  * `uint256` The ETH value to forward to the Foundation contract
  * `bytes` The calldata to use to call the Foundation contract
  * `address` The recipient of the ERC721
  * `address` The ERC721 token address
  * `uint256` The ID of the ERC721


### `SWEEP_ERC1155`[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep_erc1155 "Direct link to sweep_erc1155")
  * `address` The ERC1155 token address to sweep
  * `address` The recipient of the sweep
  * `uint256` The token ID to sweep
  * `uint256` The minimum required tokens to receive from the sweep


## Example: Reverting Commands[​](https://docs.uniswap.org/universal-router-legacy/technical-reference#example-reverting-commands "Direct link to Example: Reverting Commands")
For a Sudoswap command, that should be _allowed to revert_ , the following 8 bit command should be provided:
```
command = 0x80 (10000000) && 0x19 (00011001) = 0x99 (10011001)
```

Take care when working with reverting commands - ensure you have appended commands to deal with funds that could remain in the contract after either outcomes. For example, if the Sudoswap command reverts, a following `SWEEP` can be added to ensure that any ETH that was not spent does not get left in the router.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/universal-router-legacy/02-technical-reference.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/universal-router-legacy/overview)
On this page
  * [Functions](https://docs.uniswap.org/universal-router-legacy/technical-reference#functions)
  * [Command Structure](https://docs.uniswap.org/universal-router-legacy/technical-reference#command-structure)
    * [`f`](https://docs.uniswap.org/universal-router-legacy/technical-reference#f)
    * [`r`](https://docs.uniswap.org/universal-router-legacy/technical-reference#r)
    * [`command`](https://docs.uniswap.org/universal-router-legacy/technical-reference#command)
  * [Command Inputs](https://docs.uniswap.org/universal-router-legacy/technical-reference#command-inputs)
    * [`V3_SWAP_EXACT_IN`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v3_swap_exact_in)
    * [`V3_SWAP_EXACT_OUT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v3_swap_exact_out)
    * [`PERMIT2_TRANSFER_FROM`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_transfer_from)
    * [`PERMIT2_PERMIT_BATCH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_permit_batch)
    * [`SWEEP`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep)
    * [`TRANSFER`](https://docs.uniswap.org/universal-router-legacy/technical-reference#transfer)
    * [`PAY_PORTION`](https://docs.uniswap.org/universal-router-legacy/technical-reference#pay_portion)
    * [`V2_SWAP_EXACT_IN`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v2_swap_exact_in)
    * [`V2_SWAP_EXACT_OUT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#v2_swap_exact_out)
    * [`PERMIT2_PERMIT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_permit)
    * [`WRAP_ETH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#wrap_eth)
    * [`UNWRAP_WETH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#unwrap_weth)
    * [`PERMIT2_TRANSFER_FROM_BATCH`](https://docs.uniswap.org/universal-router-legacy/technical-reference#permit2_transfer_from_batch)
    * [`SEAPORT`](https://docs.uniswap.org/universal-router-legacy/technical-reference#seaport)
    * [`LOOKS_RARE_721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#looks_rare_721)
    * [`NFTX`](https://docs.uniswap.org/universal-router-legacy/technical-reference#nftx)
    * [`CRYPTOPUNKS`](https://docs.uniswap.org/universal-router-legacy/technical-reference#cryptopunks)
    * [`LOOKS_RARE_1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#looks_rare_1155)
    * [`OWNER_CHECK_721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#owner_check_721)
    * [`OWNER_CHECK_1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#owner_check_1155)
    * [`SWEEP_ERC721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep_erc721)
    * [`X2Y2_721`](https://docs.uniswap.org/universal-router-legacy/technical-reference#x2y2_721)
    * [`SUDOSWAP`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sudoswap)
    * [`NFT20`](https://docs.uniswap.org/universal-router-legacy/technical-reference#nft20)
    * [`X2Y2_1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#x2y2_1155)
    * [`FOUNDATION`](https://docs.uniswap.org/universal-router-legacy/technical-reference#foundation)
    * [`SWEEP_ERC1155`](https://docs.uniswap.org/universal-router-legacy/technical-reference#sweep_erc1155)
  * [Example: Reverting Commands](https://docs.uniswap.org/universal-router-legacy/technical-reference#example-reverting-commands)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/universal-router-legacy/02-technical-reference.md)
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
