# https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap

[Skip to main content](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#__docusaurus_skipToContent_fallback)
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
    * [v2 Protocol](https://docs.uniswap.org/contracts/v2/overview)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/overview)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
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
  * Guides
  * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)


On this page
The Uniswap smart contracts exist on the Ethereum blockchain. Use [ethers.js](https://github.com/ethers-io/ethers.js/) or [web3.js](https://github.com/ethereum/web3.js) to connect your website to Ethereum. Users will need a web3-enabled browser. On desktop this means using the [MetaMask](https://metamask.io/) extension or something similar. On mobile, web3-compatible browsers include [Trust Wallet](https://trustwallet.com/) and [Coinbase Wallet](https://wallet.coinbase.com/). See [ethereum.org](https://ethereum.org/use/#_3-what-is-a-wallet-and-which-one-should-i-use) to learn more.
# Factory Contract
The Uniswap [factory contract](https://github.com/Uniswap/uniswap-v1/blob/master/contracts/uniswap_factory.vy) can be used to create exchange contracts for any ERC20 token that does not already have one. It also functions as a registry of ERC20 tokens that have been added to the system, and the exchange with which they are associated.
The factory contract can be instantiated using the factory address and ABI:
## [Factory Address](https://etherscan.io/address/0xc0a47dFe034B400B47bDaD5FecDa2621de6c4d95)[​](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#factory-address "Direct link to factory-address")
```
// mainnetconst factory ='0xc0a47dFe034B400B47bDaD5FecDa2621de6c4d95'// testnetsconst ropsten ='0x9c83dCE8CA20E9aAF9D3efc003b2ea62aBC08351'const rinkeby ='0xf5D915570BC477f9B8D6C0E980aA81757A3AaC36'const kovan ='0xD3E51Ef092B2845f10401a0159B2B96e8B6c3D30'const görli ='0x6Ce570d02D73d4c384b46135E87f8C592A8c86dA'
```

### Factory Interface[​](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#factory-interface "Direct link to Factory Interface")
Creating the factory interface in web3 requires the **factory address** and the **factory ABI** :
```
const factoryABI =[{name:'NewExchange',inputs:[{type:'address',name:'token',indexed:true},{type:'address',name:'exchange',indexed:true},],anonymous:false,type:'event',},{name:'initializeFactory',outputs:[],inputs:[{type:'address',name:'template'}],constant:false,payable:false,type:'function',gas:35725,},{name:'createExchange',outputs:[{type:'address',name:'out'}],inputs:[{type:'address',name:'token'}],constant:false,payable:false,type:'function',gas:187911,},{name:'getExchange',outputs:[{type:'address',name:'out'}],inputs:[{type:'address',name:'token'}],constant:true,payable:false,type:'function',gas:715,},{name:'getToken',outputs:[{type:'address',name:'out'}],inputs:[{type:'address',name:'exchange'}],constant:true,payable:false,type:'function',gas:745,},{name:'getTokenWithId',outputs:[{type:'address',name:'out'}],inputs:[{type:'uint256',name:'token_id'}],constant:true,payable:false,type:'function',gas:736,},{name:'exchangeTemplate',outputs:[{type:'address',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:633,},{name:'tokenCount',outputs:[{type:'uint256',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:663,},]
```

```
const factoryContract =newweb3.eth.Contract(factoryABI, factoryAddress)
```

# Exchange Contracts
## Get Exchange Address[​](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#get-exchange-address "Direct link to Get Exchange Address")
There is a separate exchange contract for every ERC20 token. The `getExchange` method in the factory contract can be used to find the Ethereum address associated with an ERC20 token address.
```
const exchangeAddress = factoryContract.methods.getExchange(tokenAddress)
```

If the return value is `0x0000000000000000000000000000000000000000` the token does not yet have an exchange.
## Exchange Interface[​](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#exchange-interface "Direct link to Exchange Interface")
Creating an exchange interface in web3 requires the **exchange address** and the **exchange ABI** :
```
const exchangeABI =[{name:'TokenPurchase',inputs:[{type:'address',name:'buyer',indexed:true},{type:'uint256',name:'eth_sold',indexed:true},{type:'uint256',name:'tokens_bought',indexed:true},],anonymous:false,type:'event',},{name:'EthPurchase',inputs:[{type:'address',name:'buyer',indexed:true},{type:'uint256',name:'tokens_sold',indexed:true},{type:'uint256',name:'eth_bought',indexed:true},],anonymous:false,type:'event',},{name:'AddLiquidity',inputs:[{type:'address',name:'provider',indexed:true},{type:'uint256',name:'eth_amount',indexed:true},{type:'uint256',name:'token_amount',indexed:true},],anonymous:false,type:'event',},{name:'RemoveLiquidity',inputs:[{type:'address',name:'provider',indexed:true},{type:'uint256',name:'eth_amount',indexed:true},{type:'uint256',name:'token_amount',indexed:true},],anonymous:false,type:'event',},{name:'Transfer',inputs:[{type:'address',name:'_from',indexed:true},{type:'address',name:'_to',indexed:true},{type:'uint256',name:'_value',indexed:false},],anonymous:false,type:'event',},{name:'Approval',inputs:[{type:'address',name:'_owner',indexed:true},{type:'address',name:'_spender',indexed:true},{type:'uint256',name:'_value',indexed:false},],anonymous:false,type:'event',},{name:'setup',outputs:[],inputs:[{type:'address',name:'token_addr'}],constant:false,payable:false,type:'function',gas:175875,},{name:'addLiquidity',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'min_liquidity'},{type:'uint256',name:'max_tokens'},{type:'uint256',name:'deadline'},],constant:false,payable:true,type:'function',gas:82605,},{name:'removeLiquidity',outputs:[{type:'uint256',name:'out'},{type:'uint256',name:'out'},],inputs:[{type:'uint256',name:'amount'},{type:'uint256',name:'min_eth'},{type:'uint256',name:'min_tokens'},{type:'uint256',name:'deadline'},],constant:false,payable:false,type:'function',gas:116814,},{name:'__default__',outputs:[],inputs:[],constant:false,payable:true,type:'function',},{name:'ethToTokenSwapInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'min_tokens'},{type:'uint256',name:'deadline'},],constant:false,payable:true,type:'function',gas:12757,},{name:'ethToTokenTransferInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'min_tokens'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},],constant:false,payable:true,type:'function',gas:12965,},{name:'ethToTokenSwapOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'},{type:'uint256',name:'deadline'},],constant:false,payable:true,type:'function',gas:50455,},{name:'ethToTokenTransferOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},],constant:false,payable:true,type:'function',gas:50663,},{name:'tokenToEthSwapInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'},{type:'uint256',name:'min_eth'},{type:'uint256',name:'deadline'},],constant:false,payable:false,type:'function',gas:47503,},{name:'tokenToEthTransferInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'},{type:'uint256',name:'min_eth'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},],constant:false,payable:false,type:'function',gas:47712,},{name:'tokenToEthSwapOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'eth_bought'},{type:'uint256',name:'max_tokens'},{type:'uint256',name:'deadline'},],constant:false,payable:false,type:'function',gas:50175,},{name:'tokenToEthTransferOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'eth_bought'},{type:'uint256',name:'max_tokens'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},],constant:false,payable:false,type:'function',gas:50384,},{name:'tokenToTokenSwapInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'},{type:'uint256',name:'min_tokens_bought'},{type:'uint256',name:'min_eth_bought'},{type:'uint256',name:'deadline'},{type:'address',name:'token_addr'},],constant:false,payable:false,type:'function',gas:51007,},{name:'tokenToTokenTransferInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'},{type:'uint256',name:'min_tokens_bought'},{type:'uint256',name:'min_eth_bought'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},{type:'address',name:'token_addr'},],constant:false,payable:false,type:'function',gas:51098,},{name:'tokenToTokenSwapOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'},{type:'uint256',name:'max_tokens_sold'},{type:'uint256',name:'max_eth_sold'},{type:'uint256',name:'deadline'},{type:'address',name:'token_addr'},],constant:false,payable:false,type:'function',gas:54928,},{name:'tokenToTokenTransferOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'},{type:'uint256',name:'max_tokens_sold'},{type:'uint256',name:'max_eth_sold'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},{type:'address',name:'token_addr'},],constant:false,payable:false,type:'function',gas:55019,},{name:'tokenToExchangeSwapInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'},{type:'uint256',name:'min_tokens_bought'},{type:'uint256',name:'min_eth_bought'},{type:'uint256',name:'deadline'},{type:'address',name:'exchange_addr'},],constant:false,payable:false,type:'function',gas:49342,},{name:'tokenToExchangeTransferInput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'},{type:'uint256',name:'min_tokens_bought'},{type:'uint256',name:'min_eth_bought'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},{type:'address',name:'exchange_addr'},],constant:false,payable:false,type:'function',gas:49532,},{name:'tokenToExchangeSwapOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'},{type:'uint256',name:'max_tokens_sold'},{type:'uint256',name:'max_eth_sold'},{type:'uint256',name:'deadline'},{type:'address',name:'exchange_addr'},],constant:false,payable:false,type:'function',gas:53233,},{name:'tokenToExchangeTransferOutput',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'},{type:'uint256',name:'max_tokens_sold'},{type:'uint256',name:'max_eth_sold'},{type:'uint256',name:'deadline'},{type:'address',name:'recipient'},{type:'address',name:'exchange_addr'},],constant:false,payable:false,type:'function',gas:53423,},{name:'getEthToTokenInputPrice',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'eth_sold'}],constant:true,payable:false,type:'function',gas:5542,},{name:'getEthToTokenOutputPrice',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_bought'}],constant:true,payable:false,type:'function',gas:6872,},{name:'getTokenToEthInputPrice',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'tokens_sold'}],constant:true,payable:false,type:'function',gas:5637,},{name:'getTokenToEthOutputPrice',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'uint256',name:'eth_bought'}],constant:true,payable:false,type:'function',gas:6897,},{name:'tokenAddress',outputs:[{type:'address',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:1413,},{name:'factoryAddress',outputs:[{type:'address',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:1443,},{name:'balanceOf',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'address',name:'_owner'}],constant:true,payable:false,type:'function',gas:1645,},{name:'transfer',outputs:[{type:'bool',name:'out'}],inputs:[{type:'address',name:'_to'},{type:'uint256',name:'_value'},],constant:false,payable:false,type:'function',gas:75034,},{name:'transferFrom',outputs:[{type:'bool',name:'out'}],inputs:[{type:'address',name:'_from'},{type:'address',name:'_to'},{type:'uint256',name:'_value'},],constant:false,payable:false,type:'function',gas:110907,},{name:'approve',outputs:[{type:'bool',name:'out'}],inputs:[{type:'address',name:'_spender'},{type:'uint256',name:'_value'},],constant:false,payable:false,type:'function',gas:38769,},{name:'allowance',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'address',name:'_owner'},{type:'address',name:'_spender'},],constant:true,payable:false,type:'function',gas:1925,},{name:'name',outputs:[{type:'bytes32',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:1623,},{name:'symbol',outputs:[{type:'bytes32',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:1653,},{name:'decimals',outputs:[{type:'uint256',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:1683,},{name:'totalSupply',outputs:[{type:'uint256',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:1713,},]
```

```
const exchangeContract =newweb3.eth.Contract(exchangeABI, exchangeAddress)
```

# Token Contracts
Some Uniswap interactions require making calls directly to ERC20 token contracts rather than the exchanges with which they are associated.
## Get Token Address[​](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#get-token-address "Direct link to Get Token Address")
The `getToken` method in the factory contract can be used to find the ERC20 token address associated with an exchange contract. There is no barrier of entry for adding an ERC20 token to Uniswap or checks on the validity of the token contracts. Frontend interfaces should maintain a list of valid ERC20 tokens that users can safely trade or allow users to paste in arbitrary addresses.
```
const tokenAddress = factoryContract.methods.getToken(exchangeAddress)
```

If the return value is `0x0000000000000000000000000000000000000000` the input address is not a Uniswap exchange.
## Token Interface[​](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#token-interface "Direct link to Token Interface")
Creating a token interface in web3 requires the **token address** and the **token ABI** :
```
const tokenABI =[{name:'Transfer',inputs:[{type:'address',name:'_from',indexed:true},{type:'address',name:'_to',indexed:true},{type:'uint256',name:'_value',indexed:false},],anonymous:false,type:'event',},{name:'Approval',inputs:[{type:'address',name:'_owner',indexed:true},{type:'address',name:'_spender',indexed:true},{type:'uint256',name:'_value',indexed:false},],anonymous:false,type:'event',},{name:'__init__',outputs:[],inputs:[{type:'bytes32',name:'_name'},{type:'bytes32',name:'_symbol'},{type:'uint256',name:'_decimals'},{type:'uint256',name:'_supply'},],constant:false,payable:false,type:'constructor',},{name:'deposit',outputs:[],inputs:[],constant:false,payable:true,type:'function',gas:74279,},{name:'withdraw',outputs:[{type:'bool',name:'out'}],inputs:[{type:'uint256',name:'_value'}],constant:false,payable:false,type:'function',gas:108706,},{name:'totalSupply',outputs:[{type:'uint256',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:543,},{name:'balanceOf',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'address',name:'_owner'}],constant:true,payable:false,type:'function',gas:745,},{name:'transfer',outputs:[{type:'bool',name:'out'}],inputs:[{type:'address',name:'_to'},{type:'uint256',name:'_value'},],constant:false,payable:false,type:'function',gas:74698,},{name:'transferFrom',outputs:[{type:'bool',name:'out'}],inputs:[{type:'address',name:'_from'},{type:'address',name:'_to'},{type:'uint256',name:'_value'},],constant:false,payable:false,type:'function',gas:110600,},{name:'approve',outputs:[{type:'bool',name:'out'}],inputs:[{type:'address',name:'_spender'},{type:'uint256',name:'_value'},],constant:false,payable:false,type:'function',gas:37888,},{name:'allowance',outputs:[{type:'uint256',name:'out'}],inputs:[{type:'address',name:'_owner'},{type:'address',name:'_spender'},],constant:true,payable:false,type:'function',gas:1025,},{name:'name',outputs:[{type:'bytes32',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:723,},{name:'symbol',outputs:[{type:'bytes32',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:753,},{name:'decimals',outputs:[{type:'uint256',name:'out'}],inputs:[],constant:true,payable:false,type:'function',gas:783,},]
```

```
const tokenContract =newweb3.eth.Contract(tokenABI, tokenAddress)
```

[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/01-connect-to-uniswap.md)
Was this helpful?
[PreviousOverview](https://docs.uniswap.org/contracts/v1/overview)[NextPool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
On this page
  * [Factory Address](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#factory-address)
    * [Factory Interface](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#factory-interface)
  * [Get Exchange Address](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#get-exchange-address)
  * [Exchange Interface](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#exchange-interface)
  * [Get Token Address](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#get-token-address)
  * [Token Interface](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap#token-interface)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/01-connect-to-uniswap.md)
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
