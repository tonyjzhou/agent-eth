# https://docs.uniswap.org/contracts/v1/reference/exchange

[Skip to main content](https://docs.uniswap.org/contracts/v1/reference/exchange#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v1/reference/exchange)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v1/reference/exchange)
      * [Quickstart](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v1/reference/exchange)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [UniswapX](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [Universal Router](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [Permit2](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v1/reference/exchange)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/reference/exchange)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
        * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Iframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
        * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v1 Protocol
  * Technical Reference
  * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)


On this page
# setup
Parameter| Description  
---|---  
token_addr| Ethereum address of an ERC20 Token  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract "Direct link to Smart Contract")
```
# Can only be called by factory contract during createExchange()setup(token_addr: address):
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3 "Direct link to Web3")
```
// Can only be called by factory contract during createExchange()exchangeContract.methods.setup((token:String)).send()
```

# addLiquidity
Parameter| Type| Description  
---|---|---  
msg.value| uint256| Amount of ETH added  
min_liquidity| uint256| Minimum minted liquidity  
max_tokens| uint256| Maximum ERC20 tokens added  
deadline| uint256| Transaction deadline  
Returns|   
---|---  
uint256| Amount of liquidity tokens minted  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-1 "Direct link to Smart Contract")
```
@payableaddLiquidity(  min_liquidity: uint256,  max_tokens: uint256,  deadline: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-1 "Direct link to Web3")
```
exchangeContract.methods.addLiquidity(min_liquidity, max_tokens, deadline).send({value: ethValue })
```

# removeLiquidity
Parameter| Type| Description  
---|---|---  
amount| uint256| Amount of liquidity burned  
min_eth| uint256| Minimum ETH removed  
min_tokens| uint256| Minimum ERC20 tokens removed  
deadline| uint256| Transaction deadline  
Returns|   
---|---  
uint256| Amount of ETH removed  
uint256| Amount of ERC20 tokens removed.  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-2 "Direct link to Smart Contract")
```
removeLiquidity(  amount: uint256;  min_eth: uint256,  min_tokens: uint256,  deadline: uint256):(uint256, uint256)
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-2 "Direct link to Web3")
```
exchangeContract.methods.removeLiquidity(amount, min_eth, min_tokens, deadline).send()
```

# default
Parameter| Type| Description  
---|---|---  
msg.value| uint256| Amount of ETH sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-3 "Direct link to Smart Contract")
```
# Default function in Vyper replaces the "fallback" function in Solidity@payable__default__():
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-3 "Direct link to Web3")
```
web3.eth.sendTransaction({value: ethAmount })
```

# ethToTokenSwapInput
Parameter| Type| Description  
---|---|---  
msg.value| uint256| Amount of ETH sold  
min_tokens| uint256| Minimum ERC20 tokens bought  
deadline| uint256| Transaction deadline  
Returns|   
---|---  
uint256| Amount of ERC20 tokens bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-4 "Direct link to Smart Contract")
```
@payableethToTokenSwapInput(  min_tokens: uint256,  deadline: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-4 "Direct link to Web3")
```
exchangeContract.methods.ethToTokenSwapInput(min_liquidity, max_tokens, deadline).send({value: ethValue })
```

# ethToTokenTransferInput
Parameter| Type| Description  
---|---|---  
msg.value| uint256| Amount of ETH sold  
min_tokens| uint256| Minimum ERC20 tokens bought  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives ERC20 tokens  
Returns|   
---|---  
uint256| Amount of ERC20 tokens bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-5 "Direct link to Smart Contract")
```
@payableethToTokenTransferInput(  min_tokens: uint256,  deadline: uint256,  recipient: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-5 "Direct link to Web3")
```
exchangeContract.methods.ethToTokenTransferInput(min_liquidity, max_tokens, deadline, recipient).send({value: ethValue })
```

# ethToTokenSwapOutput
Parameter| Type| Description  
---|---|---  
msg.value| uint256| Maximum ETH sold  
tokens_bought| uint256| Amount of ERC20 tokens bought  
deadline| uint256| Transaction deadline  
Returns|   
---|---  
uint256| Amount of ETH sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-6 "Direct link to Smart Contract")
```
@payableethToTokenSwapOutput(  tokens_bought: uint256,  deadline: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-6 "Direct link to Web3")
```
exchangeContract.methods.ethToTokenSwapOutput(tokens_bought, deadline).send({value: ethValue })
```

# ethToTokenTransferOutput
Parameter| Type| Description  
---|---|---  
msg.value| uint256| Maximum ETH sold  
tokens_bought| uint256| Amount of ERC20 tokens bought  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives ERC20 tokens  
Returns|   
---|---  
uint256| Amount of ETH sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-7 "Direct link to Smart Contract")
```
@payableethToTokenTransferOutput(  tokens_bought: uint256,  deadline: uint256,  recipient: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-7 "Direct link to Web3")
```
exchangeContract.methods.ethToTokenTransferOutput(tokens_bought, deadline,(recipient:String)).send({value: ethValue })
```

# tokenToEthSwapInput
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of ERC20 tokens sold  
min_eth| uint256| Minimum ETH bought  
deadline| uint256| Transaction deadline  
Returns|   
---|---  
uint256| Amount of ETH bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-8 "Direct link to Smart Contract")
```
tokenToEthSwapInput(  tokens_sold: uint256,  min_eth: uint256,  deadline: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-8 "Direct link to Web3")
```
exchangeContract.methods.tokenToEthSwapInput(tokens_sold, min_eth, deadline).send()
```

# tokenToEthTransferInput
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of ERC20 tokens sold  
min_eth| uint256| Minimum ETH bought  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives ETH  
Returns|   
---|---  
uint256| Amount of ETH bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-9 "Direct link to Smart Contract")
```
tokenToEthTransferInput(  tokens_sold: uint256,  min_eth: uint256,  deadline: uint256,  recipient: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-9 "Direct link to Web3")
```
exchangeContract.methods.tokenToEthTransferInput(tokens_sold, min_eth, deadline, recipient).send()
```

# tokenToEthSwapOutput
Parameter| Type| Description  
---|---|---  
eth_bought| uint256| Amount of ETH bought  
max_tokens| uint256| Maximum ERC20 tokens sold  
deadline| uint256| Transaction deadline  
Returns|   
---|---  
uint256| Amount of ERC20 tokens sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-10 "Direct link to Smart Contract")
```
tokenToEthSwapOutput(  eth_bought: uint256,  max_tokens: uint256,  deadline: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-10 "Direct link to Web3")
```
exchangeContract.methods.tokenToEthSwapOutput(eth_bought, max_tokens,(deadline:Integer)).send()
```

# tokenToEthTransferOutput
Parameter| Type| Description  
---|---|---  
eth_bought| uint256| Amount of ETH bought  
max_tokens| uint256| Maximum ERC20 tokens sold  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives ETH  
Returns|   
---|---  
uint256| Amount of ERC20 tokens sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-11 "Direct link to Smart Contract")
```
tokenToEthTransferOutput(  eth_bought: uint256,  max_tokens: uint256,  deadline: uint256,  recipient: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-11 "Direct link to Web3")
```
exchangeContract.methods.tokenToEthTransferOutput(eth_bought, max_tokens,(deadline:Integer),(recipient:String)).send()
```

# tokenToTokenSwapInput
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of input ERC20 tokens sold  
min_tokens_bought| uint256| Minimum output ERC20 tokens bought  
min_eth_bought| uint256| Minimum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
token_addr| address| Address of output ERC20 token  
Returns|   
---|---  
uint256| Amount of output ERC20 tokens bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-12 "Direct link to Smart Contract")
```
tokenToTokenSwapInput(  tokens_sold: uint256,  min_tokens_bought: uint256,  min_eth_bought: uint256,  deadline: uint256,  token_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-12 "Direct link to Web3")
```
exchangeContract.methods.tokenToTokenSwapInput(tokens_sold, min_tokens_bought, min_eth_bought, deadline, token_addr).send()
```

# tokenToTokenTransferInput
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of input ERC20 tokens sold  
min_tokens_bought| uint256| Minimum output ERC20 tokens bought  
min_eth_bought| uint256| Minimum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives output ERC20 tokens  
token_addr| address| Address of output ERC20 token  
Returns|   
---|---  
uint256| Amount of output ERC20 tokens bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-13 "Direct link to Smart Contract")
```
tokenToTokenTransferInput(  tokens_sold: uint256,  min_tokens_bought: uint256,  min_eth_bought: uint256,  deadline: uint256,  recipient: address  token_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-13 "Direct link to Web3")
```
exchangeContract.methods.tokenToTokenTransferInput(tokens_sold, min_tokens_bought, min_eth_bought, deadline, recipient, token_addr).send()
```

# tokenToTokenSwapOutput
Parameter| Type| Description  
---|---|---  
tokens_bought| uint256| Amount of output ERC20 tokens bought  
max_tokens_sold| uint256| Maximum input ERC20 tokens bought  
max_eth_sold| uint256| Maximum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
token_addr| address| Address of output ERC20 token  
Returns|   
---|---  
uint256| Amount of input ERC20 tokens sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-14 "Direct link to Smart Contract")
```
tokenToTokenSwapOutput(  tokens_bought: uint256,  max_tokens_sold: uint256,  max_eth_sold: uint256,  deadline: uint256,  token_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-14 "Direct link to Web3")
```
exchangeContract.methods.tokenToTokenSwapOutput(tokens_bought, max_tokens_sold, max_eth_sold, deadline, token_addr).send()
```

# tokenToTokenTransferOutput
Parameter| Type| Description  
---|---|---  
tokens_bought| uint256| Amount of output ERC20 tokens bought  
max_tokens_sold| uint256| Maximum input ERC20 tokens bought  
max_eth_sold| uint256| Maximum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives output ERC20 tokens  
token_addr| address| Address of output ERC20 token  
Returns|   
---|---  
uint256| Amount of input ERC20 tokens sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-15 "Direct link to Smart Contract")
```
tokenToTokenTransferOutput(  tokens_bought: uint256,  max_tokens_sold: uint256,  max_eth_sold: uint256,  deadline: uint256,  recipient: address,  token_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-15 "Direct link to Web3")
```
exchangeContract.methods.tokenToTokenTransferOutput(tokens_bought, max_tokens_sold, max_eth_sold, deadline, recipient, token_addr).send()
```

# tokenToExchangeSwapInput
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of input ERC20 tokens sold  
min_tokens_bought| uint256| Minimum output ERC20 tokens bought  
min_eth_bought| uint256| Minimum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
exchange_addr| address| Address of output ERC20 token exchange  
Returns|   
---|---  
uint256| Amount of output ERC20 tokens bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-16 "Direct link to Smart Contract")
```
tokenToTokenSwapInput(  tokens_sold: uint256,  min_tokens_bought: uint256,  min_eth_bought: uint256,  deadline: uint256,  exchange_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-16 "Direct link to Web3")
```
exchangeContract.methods.tokenToTokenSwapInput(tokens_sold, min_tokens_bought, min_eth_bought, deadline, exchange_addr).send()
```

# tokenToExchangeTransferInput
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of input ERC20 tokens sold  
min_tokens_bought| uint256| Minimum output ERC20 tokens bought  
min_eth_bought| uint256| Minimum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives output ERC20 tokens  
exchange_addr| address| Address of output ERC20 token exchange  
Returns|   
---|---  
uint256| Amount of output ERC20 tokens bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-17 "Direct link to Smart Contract")
```
tokenToExchangeTransferInput(  tokens_sold: uint256,  min_tokens_bought: uint256,  min_eth_bought: uint256,  deadline: uint256,  recipient: address  exchange_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-17 "Direct link to Web3")
```
exchangeContract.methods.tokenToExchangeTransferInput(tokens_sold, min_tokens_bought, min_eth_bought, deadline, recipient, exchange_addr).send()
```

# tokenToExchangeSwapOutput
Parameter| Type| Description  
---|---|---  
tokens_bought| uint256| Amount of output ERC20 tokens bought  
max_tokens_sold| uint256| Maximum input ERC20 tokens bought  
max_eth_sold| uint256| Maximum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
exchange_addr| address| Address of output ERC20 token exchange  
Returns|   
---|---  
uint256| Amount of input ERC20 tokens sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-18 "Direct link to Smart Contract")
```
tokenToExchangeSwapOutput(  tokens_bought: uint256,  max_tokens_sold: uint256,  max_eth_sold: uint256,  deadline: uint256,  exchange_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-18 "Direct link to Web3")
```
exchangeContract.methods.tokenToExchangeSwapOutput(tokens_bought, max_tokens_sold, max_eth_sold, deadline, exchange_addr).send()
```

# tokenToExchangeTransferOutput
Parameter| Type| Description  
---|---|---  
tokens_bought| uint256| Amount of output ERC20 tokens bought  
max_tokens_sold| uint256| Maximum input ERC20 tokens bought  
max_eth_sold| uint256| Maximum ETH bought as intermediary  
deadline| uint256| Transaction deadline  
recipient| address| Address that receives output ERC20 tokens  
exchange_addr| address| Address of output ERC20 token exchange  
Returns|   
---|---  
uint256| Amount of input ERC20 tokens sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-19 "Direct link to Smart Contract")
```
tokenToExchangeTransferOutput(  tokens_bought: uint256,  max_tokens_sold: uint256,  max_eth_sold: uint256,  deadline: uint256,  recipient: address,  exchange_addr: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-19 "Direct link to Web3")
```
exchangeContract.methods.tokenToExchangeTransferOutput(tokens_bought, max_tokens_sold, max_eth_sold, deadline, recipient, exchange_addr).send()
```

# getEthToTokenInputPrice
Parameter| Type| Description  
---|---|---  
eth_sold| uint256| Amount of ETH sold  
Returns|   
---|---  
uint256| Amount of ERC20 tokens that can be bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-20 "Direct link to Smart Contract")
```
@constantgetEthToTokenInputPrice(eth_sold: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-20 "Direct link to Web3")
```
exchangeContract.methods.getEthToTokenInputPrice(eth_sold).call()
```

# getEthToTokenOutputPrice
Parameter| Type| Description  
---|---|---  
tokens_bought| uint256| Amount of ERC20 tokens bought  
Returns|   
---|---  
uint256| Amount of ETH that must be sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-21 "Direct link to Smart Contract")
```
@constantgetEthToTokenOutputPrice(tokens_bought: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-21 "Direct link to Web3")
```
exchangeContract.methods.getEthToTokenOutputPrice(tokens_bought).call()
```

# getTokenToEthInputPrice
Parameter| Type| Description  
---|---|---  
tokens_sold| uint256| Amount of ERC20 tokens sold  
Returns|   
---|---  
uint256| Amount of ETH that can be bought  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-22 "Direct link to Smart Contract")
```
@constantgetTokenToEthInputPrice(tokens_sold: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-22 "Direct link to Web3")
```
exchangeContract.methods.getTokenToEthInputPrice(tokens_sold).call()
```

# getTokenToEthOutputPrice
Parameter| Type| Description  
---|---|---  
eth_bought| uint256| Amount of ETH bought  
Returns|   
---|---  
uint256| Amount of ERC20 tokens that must be sold  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-23 "Direct link to Smart Contract")
```
@constantgetTokenToEthOutputPrice(eth_bought: uint256): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-23 "Direct link to Web3")
```
exchangeContract.methods.getTokenToEthOutputPrice(eth_bought).call()
```

# tokenAddress
Returns|   
---|---  
address| Address of ERC20 token sold on exchange  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-24 "Direct link to Smart Contract")
```
@constanttokenAddress(): address
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-24 "Direct link to Web3")
```
exchangeContract.methods.tokenAddress().call()
```

# factoryAddress
Returns|   
---|---  
address| Address of factory that created exchange  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-25 "Direct link to Smart Contract")
```
@constantfactoryAddress(): address
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-25 "Direct link to Web3")
```
exchangeContract.methods.factoryAddress().call()
```

# name
Returns|   
---|---  
bytes32| Name of liquidity token  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-26 "Direct link to Smart Contract")
```
# all exchange contracts have the same name@constantname(): bytes32 // Uniswap V1
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-26 "Direct link to Web3")
```
exchangeContract.methods.tokenAddress().call()
```

# symbol
Returns|   
---|---  
bytes32| Symbol of liquidity token  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-27 "Direct link to Smart Contract")
```
# all exchange contracts have the same symbol@constantsymbol(): bytes32 // UNI-V1
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-27 "Direct link to Web3")
```
exchangeContract.methods.tokenAddress().call()
```

# decimals
Returns|   
---|---  
uint256| Decimals of liquidity token  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-28 "Direct link to Smart Contract")
```
# all exchange contracts have the same decimals@constantdecimals(): uint256 //18
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-28 "Direct link to Web3")
```
exchangeContract.methods.decimals().call()
```

# balanceOf
Parameter| Type| Description  
---|---|---  
_owner| address| Ethereum address  
Returns|   
---|---  
uint256| Liquidity token balance of address  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-29 "Direct link to Smart Contract")
```
@constantbalanceOf(_owner: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-29 "Direct link to Web3")
```
exchangeContract.methods.balanceOf(_owner).call()
```

# transfer
Parameter| Type| Description  
---|---|---  
_to| address| Recipient address  
_value| uint256| Amount transferred  
Returns|   
---|---  
bool| True if successful. Reverts or false on failure  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-30 "Direct link to Smart Contract")
```
transfer(  _to: address,  _value : uint256):bool
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-30 "Direct link to Web3")
```
exchangeContract.methods.transfer(_to, _value).send()
```

# transferFrom
Parameter| Type| Description  
---|---|---  
_from| address| Sender address  
_to| address| Recipient address  
_value| uint256| Amount transferred  
Returns|   
---|---  
bool| True if successful. Reverts or false on failure  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-31 "Direct link to Smart Contract")
```
transferFrom(  _from: address,  _to: address,  _value : uint256):bool
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-31 "Direct link to Web3")
```
exchangeContract.methods.transferFrom(_from, _to, _value).send()
```

# approve
Parameter| Type| Description  
---|---|---  
_spender| address| Address of approved spender  
_value| uint256| Spender allowance  
Returns|   
---|---  
bool| True if successful. Reverts or false on failure  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-32 "Direct link to Smart Contract")
```
approve(  _spender: address,  _value: uint256):bool
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-32 "Direct link to Web3")
```
exchangeContract.methods.approve(_spender, _value).send()
```

# allowance
Parameter| Type| Description  
---|---|---  
_owner| address| Address of liquidity token owner  
_spender| uint256| Address of approved spender  
Returns|   
---|---  
uint256| Spender allowance  
## Smart Contract[​](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-33 "Direct link to Smart Contract")
```
allowance(  _owner: address,  _spender: address): uint256
```

## Web3[​](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-33 "Direct link to Web3")
```
exchangeContract.methods.allowance(_owner, _spender).call()
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/reference/02-exchange.md)
Was this helpful?
[PreviousFactory](https://docs.uniswap.org/contracts/v1/reference/factory)[NextInterfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
On this page
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-1)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-1)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-2)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-2)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-3)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-3)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-4)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-4)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-5)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-5)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-6)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-6)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-7)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-7)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-8)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-8)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-9)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-9)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-10)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-10)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-11)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-11)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-12)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-12)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-13)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-13)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-14)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-14)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-15)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-15)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-16)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-16)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-17)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-17)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-18)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-18)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-19)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-19)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-20)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-20)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-21)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-21)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-22)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-22)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-23)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-23)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-24)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-24)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-25)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-25)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-26)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-26)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-27)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-27)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-28)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-28)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-29)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-29)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-30)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-30)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-31)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-31)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-32)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-32)
  * [Smart Contract](https://docs.uniswap.org/contracts/v1/reference/exchange#smart-contract-33)
  * [Web3](https://docs.uniswap.org/contracts/v1/reference/exchange#web3-33)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/reference/02-exchange.md)
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
