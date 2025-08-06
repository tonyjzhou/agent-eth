# https://docs.uniswap.org/contracts/v1/reference/interfaces

[Skip to main content](https://docs.uniswap.org/contracts/v1/reference/interfaces#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v1/reference/interfaces)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v1/reference/interfaces)
      * [Quickstart](https://docs.uniswap.org/contracts/v1/reference/interfaces)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v1/reference/interfaces)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v1/reference/interfaces)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v1/reference/interfaces)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [UniswapX](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [Universal Router](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [Permit2](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v1/reference/interfaces)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/reference/interfaces)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/reference/interfaces)
        * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
        * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Iframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
        * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/interfaces)
        * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v1 Protocol
  * Technical Reference
  * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)


On this page
# Factory
## Solidity[​](https://docs.uniswap.org/contracts/v1/reference/interfaces#solidity "Direct link to Solidity")
```
interfaceUniswapFactoryInterface{// Public Variablesaddresspublic exchangeTemplate;uint256public tokenCount;// Create ExchangefunctioncreateExchange(address token)externalreturns(address exchange);// Get Exchange and Token InfofunctiongetExchange(address token)externalviewreturns(address exchange);functiongetToken(address exchange)externalviewreturns(address token);functiongetTokenWithId(uint256 tokenId)externalviewreturns(address token);// Never usefunctioninitializeFactory(address template)external;}
```

## Vyper[​](https://docs.uniswap.org/contracts/v1/reference/interfaces#vyper "Direct link to Vyper")
```
contract UniswapFactoryInterface():# Create ExchangedefcreateExchange(token: address)-> address: modifying# Public VariablesdefexchangeTemplate()-> address: constantdeftokenCount()-> uint256: constant# Get Exchange and Token InfodefgetExchange(token_addr: address)-> address: constantdefgetToken(exchange: address)-> address: constantdefgetTokenWithId(token_id: uint256)-> address: constant# Initialize FactorydefinitializeFactory(template: address): modifying
```

# Exchange
## Solidity[​](https://docs.uniswap.org/contracts/v1/reference/interfaces#solidity-1 "Direct link to Solidity")
```
interfaceUniswapExchangeInterface{// Address of ERC20 token sold on this exchangefunctiontokenAddress()externalviewreturns(address token);// Address of Uniswap FactoryfunctionfactoryAddress()externalviewreturns(address factory);// Provide LiquidityfunctionaddLiquidity(uint256 min_liquidity,uint256 max_tokens,uint256 deadline)externalpayablereturns(uint256);functionremoveLiquidity(uint256 amount,uint256 min_eth,uint256 min_tokens,uint256 deadline)externalreturns(uint256,uint256);// Get PricesfunctiongetEthToTokenInputPrice(uint256 eth_sold)externalviewreturns(uint256 tokens_bought);functiongetEthToTokenOutputPrice(uint256 tokens_bought)externalviewreturns(uint256 eth_sold);functiongetTokenToEthInputPrice(uint256 tokens_sold)externalviewreturns(uint256 eth_bought);functiongetTokenToEthOutputPrice(uint256 eth_bought)externalviewreturns(uint256 tokens_sold);// Trade ETH to ERC20functionethToTokenSwapInput(uint256 min_tokens,uint256 deadline)externalpayablereturns(uint256 tokens_bought);functionethToTokenTransferInput(uint256 min_tokens,uint256 deadline,address recipient)externalpayablereturns(uint256 tokens_bought);functionethToTokenSwapOutput(uint256 tokens_bought,uint256 deadline)externalpayablereturns(uint256 eth_sold);functionethToTokenTransferOutput(uint256 tokens_bought,uint256 deadline,address recipient)externalpayablereturns(uint256 eth_sold);// Trade ERC20 to ETHfunctiontokenToEthSwapInput(uint256 tokens_sold,uint256 min_eth,uint256 deadline)externalreturns(uint256 eth_bought);functiontokenToEthTransferInput(uint256 tokens_sold,uint256 min_eth,uint256 deadline,address recipient)externalreturns(uint256 eth_bought);functiontokenToEthSwapOutput(uint256 eth_bought,uint256 max_tokens,uint256 deadline)externalreturns(uint256 tokens_sold);functiontokenToEthTransferOutput(uint256 eth_bought,uint256 max_tokens,uint256 deadline,address recipient)externalreturns(uint256 tokens_sold);// Trade ERC20 to ERC20functiontokenToTokenSwapInput(uint256 tokens_sold,uint256 min_tokens_bought,uint256 min_eth_bought,uint256 deadline,address token_addr)externalreturns(uint256 tokens_bought);functiontokenToTokenTransferInput(uint256 tokens_sold,uint256 min_tokens_bought,uint256 min_eth_bought,uint256 deadline,address recipient,address token_addr)externalreturns(uint256 tokens_bought);functiontokenToTokenSwapOutput(uint256 tokens_bought,uint256 max_tokens_sold,uint256 max_eth_sold,uint256 deadline,address token_addr)externalreturns(uint256 tokens_sold);functiontokenToTokenTransferOutput(uint256 tokens_bought,uint256 max_tokens_sold,uint256 max_eth_sold,uint256 deadline,address recipient,address token_addr)externalreturns(uint256 tokens_sold);// Trade ERC20 to Custom PoolfunctiontokenToExchangeSwapInput(uint256 tokens_sold,uint256 min_tokens_bought,uint256 min_eth_bought,uint256 deadline,address exchange_addr)externalreturns(uint256 tokens_bought);functiontokenToExchangeTransferInput(uint256 tokens_sold,uint256 min_tokens_bought,uint256 min_eth_bought,uint256 deadline,address recipient,address exchange_addr)externalreturns(uint256 tokens_bought);functiontokenToExchangeSwapOutput(uint256 tokens_bought,uint256 max_tokens_sold,uint256 max_eth_sold,uint256 deadline,address exchange_addr)externalreturns(uint256 tokens_sold);functiontokenToExchangeTransferOutput(uint256 tokens_bought,uint256 max_tokens_sold,uint256 max_eth_sold,uint256 deadline,address recipient,address exchange_addr)externalreturns(uint256 tokens_sold);// ERC20 compatibility for liquidity tokensbytes32public name;bytes32public symbol;uint256public decimals;functiontransfer(address _to,uint256 _value)externalreturns(bool);functiontransferFrom(address _from,address _to,uint256 value)externalreturns(bool);functionapprove(address _spender,uint256 _value)externalreturns(bool);functionallowance(address _owner,address _spender)externalviewreturns(uint256);functionbalanceOf(address _owner)externalviewreturns(uint256);functiontotalSupply()externalviewreturns(uint256);// Never usefunctionsetup(address token_addr)external;}
```

## Vyper[​](https://docs.uniswap.org/contracts/v1/reference/interfaces#vyper-1 "Direct link to Vyper")
```
contract UniswapExchangeInterface():# Public VariablesdeftokenAddress()-> address: constantdeffactoryAddress()-> address: constant# Providing LiquiditydefaddLiquidity(min_liquidity: uint256, max_tokens: uint256, deadline: timestamp)-> uint256: modifyingdefremoveLiquidity(amount: uint256, min_eth: uint256(wei), min_tokens: uint256, deadline: timestamp)->(uint256(wei), uint256): modifying# TradingdefethToTokenSwapInput(min_tokens: uint256, deadline: timestamp)-> uint256: modifyingdefethToTokenTransferInput(min_tokens: uint256, deadline: timestamp, recipient: address)-> uint256: modifyingdefethToTokenSwapOutput(tokens_bought: uint256, deadline: timestamp)-> uint256(wei): modifyingdefethToTokenTransferOutput(tokens_bought: uint256, deadline: timestamp, recipient: address)-> uint256(wei): modifyingdeftokenToEthSwapInput(tokens_sold: uint256, min_eth: uint256(wei), deadline: timestamp)-> uint256(wei): modifyingdeftokenToEthTransferInput(tokens_sold: uint256, min_eth: uint256(wei), deadline: timestamp, recipient: address)-> uint256(wei): modifyingdeftokenToEthSwapOutput(eth_bought: uint256(wei), max_tokens: uint256, deadline: timestamp)-> uint256: modifyingdeftokenToEthTransferOutput(eth_bought: uint256(wei), max_tokens: uint256, deadline: timestamp, recipient: address)-> uint256: modifyingdeftokenToTokenSwapInput(tokens_sold: uint256, min_tokens_bought: uint256, min_eth_bought: uint256(wei), deadline: timestamp, token_addr: address)-> uint256: modifyingdeftokenToTokenTransferInput(tokens_sold: uint256, min_tokens_bought: uint256, min_eth_bought: uint256(wei), deadline: timestamp, recipient: address, token_addr: address)-> uint256: modifyingdeftokenToTokenSwapOutput(tokens_bought: uint256, max_tokens_sold: uint256, max_eth_sold: uint256(wei), deadline: timestamp, token_addr: address)-> uint256: modifyingdeftokenToTokenTransferOutput(tokens_bought: uint256, max_tokens_sold: uint256, max_eth_sold: uint256(wei), deadline: timestamp, recipient: address, token_addr: address)-> uint256: modifyingdeftokenToExchangeSwapInput(tokens_sold: uint256, min_tokens_bought: uint256, min_eth_bought: uint256(wei), deadline: timestamp, exchange_addr: address)-> uint256: modifyingdeftokenToExchangeTransferInput(tokens_sold: uint256, min_tokens_bought: uint256, min_eth_bought: uint256(wei), deadline: timestamp, recipient: address, exchange_addr: address)-> uint256: modifyingdeftokenToExchangeSwapOutput(tokens_bought: uint256, max_tokens_sold: uint256, max_eth_sold: uint256(wei), deadline: timestamp, exchange_addr: address)-> uint256: modifyingdeftokenToExchangeTransferOutput(tokens_bought: uint256, max_tokens_sold: uint256, max_eth_sold: uint256(wei), deadline: timestamp, recipient: address, exchange_addr: address)-> uint256: modifying# Get PricedefgetEthToTokenInputPrice(eth_sold: uint256(wei))-> uint256: constantdefgetEthToTokenOutputPrice(tokens_bought: uint256)-> uint256(wei): constantdefgetTokenToEthInputPrice(tokens_sold: uint256)-> uint256(wei): constantdefgetTokenToEthOutputPrice(eth_bought: uint256(wei))-> uint256: constant# Pool Token ERC20 CompatibilitydefbalanceOf()-> address: constantdefallowance(_owner : address, _spender : address)-> uint256: constantdeftransfer(_to : address, _value : uint256)->bool: modifyingdeftransferFrom(_from : address, _to : address, _value : uint256)->bool: modifyingdefapprove(_spender : address, _value : uint256)->bool: modifying# Setupdefsetup(token_addr: address): modifying
```

# ERC20 Token
## Solidity[​](https://docs.uniswap.org/contracts/v1/reference/interfaces#solidity-2 "Direct link to Solidity")
```
interfaceERC20Interface{functiontotalSupply()publicviewreturns(uint);functionbalanceOf(address tokenOwner)publicviewreturns(uint balance);functionallowance(address tokenOwner,address spender)publicviewreturns(uint remaining);functiontransfer(address to,uint tokens)publicreturns(bool success);functionapprove(address spender,uint tokens)publicreturns(bool success);functiontransferFrom(addressfrom,address to,uint tokens)publicreturns(bool success);// optionalfunctionname()externalviewreturns(string);functionsymbol()externalviewreturns(string);functiondecimals()externalviewreturns(string);eventTransfer(addressindexedfrom,addressindexed to,uint tokens);eventApproval(addressindexed tokenOwner,addressindexed spender,uint tokens);}
```

## Vyper[​](https://docs.uniswap.org/contracts/v1/reference/interfaces#vyper-2 "Direct link to Vyper")
```
contract ERC20Interface():deftotalSupply()-> uint256: constantdefbalanceOf(_owner: address)-> uint256: constantdefallowance(_owner : address, _spender : address)-> uint256: constantdeftransfer(_to : address, _value : uint256)->bool: modifyingdefapprove(_spender : address, _value : uint256)->bool: modifyingdeftransferFrom(_from : address, _to : address, _value : uint256)->bool: modifying# optionaldefname()-> bytes32: constantdefsymbol()-> bytes32: constantdefdecimals()-> uint256: constant
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/reference/03-interfaces.md)
Was this helpful?
[PreviousExchange](https://docs.uniswap.org/contracts/v1/reference/exchange)[NextOverview](https://docs.uniswap.org/sdk/v4/overview)
On this page
  * [Solidity](https://docs.uniswap.org/contracts/v1/reference/interfaces#solidity)
  * [Vyper](https://docs.uniswap.org/contracts/v1/reference/interfaces#vyper)
  * [Solidity](https://docs.uniswap.org/contracts/v1/reference/interfaces#solidity-1)
  * [Vyper](https://docs.uniswap.org/contracts/v1/reference/interfaces#vyper-1)
  * [Solidity](https://docs.uniswap.org/contracts/v1/reference/interfaces#solidity-2)
  * [Vyper](https://docs.uniswap.org/contracts/v1/reference/interfaces#vyper-2)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/reference/03-interfaces.md)
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
