# https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer

[Skip to main content](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#__docusaurus_skipToContent_fallback)
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
    * [Smart Wallet](https://docs.uniswap.org/contracts/smart-wallet/overview)
    * [UniswapX](https://docs.uniswap.org/contracts/uniswapx/overview)
    * [Universal Router](https://docs.uniswap.org/contracts/universal-router/overview)
    * [Permit2](https://docs.uniswap.org/contracts/permit2/overview)
      * [Overview](https://docs.uniswap.org/contracts/permit2/overview)
      * [Technical Reference](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [SignatureTransfer](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)
        * [AllowanceTransfer](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * Permit2
  * Technical Reference
  * [AllowanceTransfer](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer)


On this page
# AllowanceTransfer
> [**Source Code**](https://github.com/Uniswap/permit2/blob/main/src/AllowanceTransfer.sol)
## Overview[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#overview "Direct link to Overview")
The main entry points on this contract are:
  * `approve`
    * Use approve when you do not want to set token permissions through signature validation.
  * `permit`
    * Use permit when you do want to set token permissions through signature validation.
  * `transferFrom`
    * Use transferFrom when you want to transfer a token and have the necessary permissions to do so.


## Functions[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#functions "Direct link to Functions")
### `approve`[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#approve "Direct link to approve")
**Function Signature**
```
functionapprove(address token,address spender,uint160 amount,uint48 expiration)external
```

**Parameters**
  * token - the token address to approve
  * spender - the spender address to approve
  * amount - the approved amount of the token, `type(uint160).max` is treated as an unlimited allowance
  * expiration - the timestamp at which the approval is no longer valid, passing in `0` will expire the permissions at `block.timestamp`


### Single `permit`[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#single-permit "Direct link to single-permit")
**Function Signature**
```
functionpermit(address owner, PermitSingle memory permitSingle,bytescalldata signature)external;
```

**Parameters**
  * owner - the address of the token’s owner
  * permitSingle - constructed with the following:


```
structPermitSingle{// the permit data for a single token allowance    PermitDetails details;// address permissioned on the allowed tokensaddress spender;// deadline on the permit signatureuint256 sigDeadline;}structPermitDetails{// ERC20 token addressaddress token;// the maximum amount allowed to spenduint160 amount;// timestamp at which a spender's token allowances become invaliduint48 expiration;// an incrementing value indexed per owner,token,and spender for each signatureuint48 nonce;}
```

  * signature - the signature over the permit data. Supports EOA signatures, compact signatures defined by [EIP-2098](https://eips.ethereum.org/EIPS/eip-2098), and contract signatures defined by [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)


### Batched `permit`[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#batched-permit "Direct link to batched-permit")
**Function Signature**
```
functionpermit(address owner, PermitBatch memory permitBatch,bytescalldata signature)external;
```

**Parameters**
  * owner - the address of the token’s owner
  * permitBatch - constructed with the following:


```
structPermitBatch{// the permit data for multiple token allowances    PermitDetails[] details;// address permissioned on the allowed tokensaddress spender;// deadline on the permit signatureuint256 sigDeadline;}structPermitDetails{// ERC20 token addressaddress token;// the maximum amount allowed to spenduint160 amount;// timestamp at which a spender's token allowances become invaliduint48 expiration;// an incrementing value indexed per owner,token,and spender for each signatureuint48 nonce;}
```

  * signature - the signature over the permit data. Supports EOA signatures, compact signatures defined by [EIP-2098](https://eips.ethereum.org/EIPS/eip-2098), and contract signatures defined by [EIP-1271](https://eips.ethereum.org/EIPS/eip-1271)


### Single `transferFrom`[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#single-transferfrom "Direct link to single-transferfrom")
**Function Signature**
```
functiontransferFrom(addressfrom,address to,uint160 amount,address token)external;
```

**Parameters**
  * from - the address to transfer the token from
  * to - the address of the recipient
  * amount - the amount of the token to transfer, the maximum amount is `type(uint160).max`
  * token - the address of the token to be transferred


### Batched `transferFrom`[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#batched-transferfrom "Direct link to batched-transferfrom")
**Function Signature**
```
functiontransferFrom(AllowanceTransferDetails[]calldata transferDetails)external;
```

**Parameters**
  * transferDetails - constructed with the following


```
structAllowanceTransferDetails{// the owner of the tokenaddressfrom;// the recipient of the tokenaddress to;// the amount of the tokenuint160 amount;// the token to be transferredaddress token;}
```

## Nonce Schema[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#nonce-schema "Direct link to Nonce Schema")
The nonces used to protect against replay attacks of signatures are similar to standard incrementing nonces. However, we pack nonces with an allowed amount, and an allowed duration. This means that nonces are incremented _per owner_ , _per token_ , and _per spender._ Which further implies that you could sign two different permits at the same time with the same nonces and they _won’t_ cancel each other out so long as the token or spender differ.
The mapping nonces are packed in is defined as follows:
```
mapping(address=>mapping(address=>mapping(address=> PackedAllowance)))public allowance;
```

and indexed as follows:
```
PackedAllowance allowanceInformation = allowance[ownerAddress][tokenAddress][spenderAddress];uint48 nonce = allowanceInformation.nonce;
```

## Security Considerations[​](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#security-considerations "Direct link to Security Considerations")
Similar to the security considerations outlined in SignatureTransfer, integrating contracts need to perform valid safety checks on the caller and pass in correct addresses for the `from` argument in any transfer calls.
All amounts on the `AllowanceTransfer` contract are of type `uint160` so make sure integrating contracts safely downcast if they have to. See how Permit2Lib downcasts safely.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/permit2/reference/allowance-transfer.md)
Was this helpful?
[PreviousSignatureTransfer](https://docs.uniswap.org/contracts/permit2/reference/signature-transfer)[NextOverview](https://docs.uniswap.org/contracts/v2/overview)
On this page
  * [Overview](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#overview)
  * [Functions](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#functions)
    * [`approve`](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#approve)
    * [Single `permit`](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#single-permit)
    * [Batched `permit`](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#batched-permit)
    * [Single `transferFrom`](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#single-transferfrom)
    * [Batched `transferFrom`](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#batched-transferfrom)
  * [Nonce Schema](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#nonce-schema)
  * [Security Considerations](https://docs.uniswap.org/contracts/permit2/reference/allowance-transfer#security-considerations)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/permit2/reference/allowance-transfer.md)
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
