# https://docs.uniswap.org/contracts/v1/reference/factory

[Skip to main content](https://docs.uniswap.org/contracts/v1/reference/factory#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v1/reference/factory)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v1/reference/factory)
      * [Quickstart](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v1/reference/factory)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [UniswapX](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [Universal Router](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [Permit2](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v1/reference/factory)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/reference/factory)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
        * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Iframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
        * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v1 Protocol
  * Technical Reference
  * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)


On this page
# initializeFactory
Parameter| Description  
---|---  
template| Ethereum address of exchange template  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract "Direct link to Smart Contract")
```
initializeFactory(template: address)
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/factory#web3 "Direct link to Web3")
```
factoryContract.methods.initializeFactory(template).send()
```

# createExchange
Parameter| Type| Description  
---|---|---  
token| address| Ethereum address of an ERC20 token  
Returns|   
---|---  
address| Ethereum address of a Uniswap exchange  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-1 "Direct link to Smart Contract")
```
createExchange(token: address): address
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/factory#web3-1 "Direct link to Web3")
```
factoryContract.methods.createExchange(token).send()
```

# getExchange
Parameter| Type| Description  
---|---|---  
token| address| Ethereum address of an ERC20 token  
Returns|   
---|---  
address| Ethereum address of a Uniswap exchange  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-2 "Direct link to Smart Contract")
```
@constantgetExchange(token: address): address
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/factory#web3-2 "Direct link to Web3")
```
factoryContract.methods.getExchange(token).call()
```

# getToken
Parameter| Type| Description  
---|---|---  
exchange| address| Ethereum address of a Uniswap exchange  
Returns|   
---|---  
address| Ethereum address of an ERC20 token  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-3 "Direct link to Smart Contract")
```
@constantgetToken(exchange: address): address
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/factory#web3-3 "Direct link to Web3")
```
factoryContract.methods.getToken(exchange).call()
```

# getTokenWithId
Parameter| Type| Description  
---|---|---  
token_id| uint256| Uniswap ID for an ERC20 token  
Returns|   
---|---  
address| Ethereum address of an ERC20 token  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-4 "Direct link to Smart Contract")
```
@constantgetTokenWithId(token_id: uint256): address
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/factory#web3-4 "Direct link to Web3")
```
factoryContract.methods.getTokenWithId(token_id).call()
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/reference/01-factory.md)
Was this helpful?
[PreviousToken Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)[NextExchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
On this page
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/factory#web3)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-1)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/factory#web3-1)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-2)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/factory#web3-2)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-3)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/factory#web3-3)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/factory#smart-contract-4)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/factory#web3-4)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/reference/01-factory.md)
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
