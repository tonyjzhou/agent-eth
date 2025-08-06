# https://docs.uniswap.org/contracts/permit2/reference/signature-transfer

[Skip to main content](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
      * [Quickstart](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
      * [Technical Reference](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
    * [v3 Protocol](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
    * [Smart Wallet](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
    * [UniswapX](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
    * [Universal Router](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
      * [Overview](https://docs.uniswap.org/contracts/permit2/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [SignatureTransfer](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [AllowanceTransfer](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer)
    * [v2 Protocol](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
    * [v1 Protocol](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Permit2
  * Technical Reference
  * [SignatureTransfer](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)


On this page
# SignatureTransfer
> [**Source Code**](https://github.com/Uniswap/permit2/blob/main/src/SignatureTransfer.sol)
## Overview[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#overview "Direct link to Overview")
The main entry points on this contract are:
  * `permitTransferFrom`
    * Use permitTransferFrom when you want to transfer a token from an owner through signature validation.
  * `permitWitnessTransferFrom`
    * Use permitWitnessTransferFrom when you want to transfer a token from an owner through signature validation, but you would also like to validate other data. Any other data you wish to be validated can be passed through with the `witness` param.


Each of these functions is overloaded with a batched version that allows users to transfer multiple tokens with 1 transaction.
## Functions[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#functions "Direct link to Functions")
### Single `permitTransferFrom`[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#single-permittransferfrom "Direct link to single-permittransferfrom")
Use the `permitTransferFrom` to transfer just one token.
**Function signature**
```
functionpermitTransferFrom(    PermitTransferFrom memory permit,    SignatureTransferDetails calldata transferDetails,address owner,bytescalldata signature)external
```

**Parameters**
  * permit - Construct `PermitTransferFrom` struct with the following:


```
structPermitTransferFrom{    TokenPermissions permitted;// a unique value for every token owner's signature to prevent signature replaysuint256 nonce;// deadline on the permit signatureuint256 deadline;}structTokenPermissions{// ERC20 token addressaddress token;// the maximum amount that can be spentuint256 amount;}
```

  * transferDetails - information about recipient and amount


```
structSignatureTransferDetails{// recipient addressaddress to;// spender requested amountuint256 requestedAmount;}
```

  * owner - the signer of the permit message and owner of the tokens
  * signature - the signature over the permit data. Supports EOA signatures, compact signatures defined by [EIP-2098](https://eips.ethereum.org/EIPS/eip-2098), and contract signatures defined by [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)


### Batched `permitTransferFrom`[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#batched-permittransferfrom "Direct link to batched-permittransferfrom")
Use `permitTransferFrom` with the batched parameters when you want to transfer multiple tokens from an owner.
**Function Signature**
```
functionpermitTransferFrom(    PermitBatchTransferFrom memory permit,    SignatureTransferDetails[]calldata transferDetails,address owner,bytescalldata signature)external
```

**Parameters**
  * permit - Construct `PermitBatchTransferFrom` with the following:


```
structPermitBatchTransferFrom{// the tokens and corresponding amounts permitted for a transfer    TokenPermissions[] permitted;// a unique value for every token owner's signature to prevent signature replaysuint256 nonce;// deadline on the permit signatureuint256 deadline;}structTokenPermissions{// ERC20 token addressaddress token;// the maximum amount that can be spentuint256 amount;}
```

  * transferDetails - parameterized by the spender with information about the token transfer. 
    * The length of the `SignatureTransferDetails` array must equal the length of the `TokenPermissions` array passed in with `PermitBatchTransferFrom` struct. The token to be transferred specified in the `TokenPermissions` array should match the index of the `SignatureTransferDetails` array.
    * Note that if a spender is permitted to a token but does not need to transfer that token, they can specify that the `requestedAmount` is 0 so that the transfer is skipped.
  * owner - the signer of the permit message and owner of the tokens


```
structSignatureTransferDetails{// recipient addressaddress to;// spender requested amountuint256 requestedAmount;}
```

  * signature - the signature over the permit data. Supports EOA signatures, compact signatures defined by [EIP-2098](https://eips.ethereum.org/EIPS/eip-2098), and contract signatures defined by [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)


### Single `permitWitnessTransferFrom`[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#single-permitwitnesstransferfrom "Direct link to single-permitwitnesstransferfrom")
**Function Signature**
```
functionpermitWitnessTransferFrom(    PermitTransferFrom memory permit,    SignatureTransferDetails calldata transferDetails,address owner,bytes32 witness,stringcalldata witnessTypeString,bytescalldata signature)external
```

**Parameters**
  * permit - constructed with the same type as defined above in the single permitTransferFrom case
  * transferDetails constructed with same type as defined above in the single permitTransferFrom case
  * owner - the signer of the permit message and owner of the tokens
  * witness - arbitrary data passed through that was signed by the user. Is used to reconstruct the signature. Pass through this data if you want the permit signature recovery also to validate other data.
  * witnessTypeString - a string that defines the typed data that the witness was hashed from. It must also include the `TokenPermissions` struct and comply with [EIP-712](https://eips.ethereum.org/EIPS/eip-712) struct ordering. See an example below.
  * signature - the signature over the permit data. Supports EOA signatures, compact signatures defined by [EIP-2098](https://eips.ethereum.org/EIPS/eip-2098), and contract signatures defined by [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)


### Batch `permitWitnessTransferFrom`[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#batch-permitwitnesstransferfrom "Direct link to batch-permitwitnesstransferfrom")
**Function Signature**
```
functionpermitWitnessTransferFrom(    PermitBatchTransferFrom memory permit,    SignatureTransferDetails[]calldata transferDetails,address owner,bytes32 witness,stringcalldata witnessTypeString,bytescalldata signature)external
```

**Parameters**
  * permit - constructed with the same type in the batched case of `permitTransferFrom`
  * transferDetails - constructed with the same type in the batched case of `permitTransferFrom`
  * owner - the signer of the permit message and owner of the tokens
  * witness - arbitrary data passed through that was signed by the user. Is used to reconstruct the signature. Pass through this data if you want the permit signature recovery to also validate other data.
  * witnessTypeString - a string that defines the typed data that the witness was hashed from. It must also include the `TokenPermissions` struct and comply with [EIP-712](https://eips.ethereum.org/EIPS/eip-712) struct ordering. See an example below.
  * signature - the signature over the permit data. Supports EOA signatures, compact signatures defined by [EIP-2098](https://eips.ethereum.org/EIPS/eip-2098), and contract signatures defined by [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)


**Example`permitWitnessTransferFrom` parameters**
If an integrating contract would also like the signer to verify information about a trade, an integrating contract may ask the signer to also sign an `ExampleTrade` object that we define below:
```
structExampleTrade{address exampleTokenAddress;uint256 exampleMinimumAmountOut;}
```

Following EIP-712, the typehash for the data would be defined by:
```
bytes32 _EXAMPLE_TRADE_TYPEHASH =keccak256('ExampleTrade(address exampleTokenAddress,uint256 exampleMinimumAmountOut)');
```

The `witness` that should be passed along with the permit message should be:
```
bytes32 witness =keccak256(      abi.encode(_EXAMPLE_TRADE_TYPEHASH, exampleTrade.exampleTokenAddress, exampleTrade.exampleMinimumAmountOut));
```

And the `witnessTypeString` to be passed in should be:
```
stringconstant witnessTypeString ="ExampleTrade witness)ExampleTrade(address exampleTokenAddress,uint256 exampleMinimumAmountOut)TokenPermissions(address token,uint256 amount)"
```

It’s important to note that when hashing multiple typed structs, the ordering of the structs in the type string matters. Referencing EIP-721:
> If the struct type references other struct types (and these in turn reference even more struct types), then the set of referenced struct types is collected, sorted by name and appended to the encoding. An example encoding is `Transaction(Person from,Person to,Asset tx)Asset(address token,uint256 amount)Person(address wallet,string name)`
## Nonce Schema[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#nonce-schema "Direct link to Nonce Schema")
Instead of using incrementing nonces, we introduce non-monotonic, or unordered nonces with a `nonceBitmap`.
The `nonceBitmap` maps an owner's address to a uint248 value, which we will call `wordPos` which is the index of the desired bitmap. There are 2248 possible indices thus 2248 possible bitmaps where each bitmap holds 256 bits. A bit must be flipped on to prevent replays of users’ signatures. Bits that are dirtied may not be used again.
```
// nonceBitmap[ownerAddress][wordPosition] retrieves a uint256 bitmapmapping(address=>mapping(uint248=>uint256))public nonceBitmap;
```

Users will sign a `uint256 nonce` value where the first 248 bits correspond to the word position of the bitmap to dirty and the last 8 bits correspond to the actual bit position being flipped on.
```
uint248 wordPos =uint248(nonce >>8);uint8 bitPos =uint8(nonce);
```

```
uint256 bitmap = nonceBitmap[wordPos][bitPos]
```

## Security Considerations[​](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#security-considerations "Direct link to Security Considerations")
An integrating contract must check that tokens are released by a triggering call from the signer, or that the signer meant for their signature to be released by someone else.
💡 Consider this scenario:
A signer called Bob signs a permit to transfer 100 USDC with a router contract as the permissioned spender. The router contract never checks who the caller is but spends any permit messages on the Permit2 contract. An attacker Eve can steal Bob’s signature, pass it through to the router with herself as the recipient, and transfer Bob’s tokens to herself.
Universal Router protects against this by checking that the `msg.sender` from inside the routing contract is the supposed spender by passing `msg.sender` in as the `owner` param in any permit calls and by passing in `msg.sender` as the `from` param in any transfer calls.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/permit2/reference/signature-transfer.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/permit2/overview)[NextAllowanceTransfer](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer)
On this page
  * [Overview](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#overview)
  * [Functions](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#functions)
    * [Single `permitTransferFrom`](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#single-permittransferfrom)
    * [Batched `permitTransferFrom`](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#batched-permittransferfrom)
    * [Single `permitWitnessTransferFrom`](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#single-permitwitnesstransferfrom)
    * [Batch `permitWitnessTransferFrom`](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#batch-permitwitnesstransferfrom)
  * [Nonce Schema](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#nonce-schema)
  * [Security Considerations](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer#security-considerations)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/permit2/reference/signature-transfer.md)
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
