# https://docs.uniswap.org/contracts/v1/guides/pool-liquidity

[Skip to main content](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#__docusaurus_skipToContent_fallback)
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
  * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)


On this page
# Formalized Model
Uniswap liquidity pools are autonomous and use the Constant Product Market Maker (`x * y = k`). This model was formalized and the smart contract implementation passed a lightweight formal verification.
  * [Formalized Specification](https://github.com/runtimeverification/verified-smart-contracts/blob/uniswap/uniswap/x-y-k.pdf)
  * [Lightweight Verification](https://github.com/runtimeverification/verified-smart-contracts/tree/uniswap/uniswap/results)


## Create Exchange[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#create-exchange "Direct link to Create Exchange")
The `createExchange` function is used to deploy exchange contracts for ERC20 tokens that do not yet have one.
```
factory.methods.createExchange(tokenAddress).send()
```

Once an exchange is created the address can be retrieved with [`getExchange`](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap/#get-exchange-address).
## Exchange Reserves[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#exchange-reserves "Direct link to Exchange Reserves")
Each exchange contract holds a liquidity reserve of ETH and its associated ERC20 token.
### ETH Reserve[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#eth-reserve "Direct link to ETH Reserve")
The ETH reserve associated with an ERC20 token exchange is the ETH balance of the exchange smart contract.
```
const ethReserve = web3.eth.getBalance(exchangeAddress)
```

### ERC20 Reserve[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#erc20-reserve "Direct link to ERC20 Reserve")
The ERC20 reserve associated with an ERC20 token exchange is the ERC20 balance of the exchange smart contract.
```
const tokenReserve = tokenContract.methods.balanceOf(exchangeAddress)
```

## Add Liquidity[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#add-liquidity "Direct link to Add Liquidity")
Anyone who wants can join a Uniswap liquidity pool by calling the `addLiquidity` function.
```
exchange.methods.addLiquidity(min_liquidity, max_tokens, deadline).send({value: ethAmount })
```

Adding liquidity requires depositing an equivalent **value** of ETH and ERC20 tokens into the ERC20 token's associated exchange contract.
The first liquidity provider to join a pool sets the initial exchange rate by depositing what they believe to be an equivalent value of ETH and ERC20 tokens. If this ratio is off, arbitrage traders will bring the prices to equilibrium at the expense of the initial liquidity provider.
All future liquidity providers deposit ETH and ERC20's using the exchange rate at the moment of their deposit. If the exchange rate is bad there is a profitable arbitrage opportunity that will correct the price.
### Parameters[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#parameters "Direct link to Parameters")
The `ethAmount` sent to `addLiquidity` is the exact amount of ETH that will be deposited into the liquidity reserves. It should be 50% of the total value a liquidity provider wishes to deposit into the reserves.
Since liquidity providers must deposit at the current exchange rate, the Uniswap smart contracts use `ethAmount` to determine the amount of ERC20 tokens that must be deposited. This token amount is the remaining 50% of total value a liquidity provider wishes to deposit. Since exchange rate can change between when a transaction is signed and when it is executed on Ethereum, `max_tokens` is used to bound the amount this rate can fluctuate. For the first liquidity provider, `max_tokens` is the exact amount of tokens deposited.
Liquidity tokens are minted to track the relative proportion of total reserves that each liquidity provider has contributed. `min_liquidity` is used in combination with `max_tokens` and `ethAmount` to bound the rate at which liquidity tokens are minted. For the first liquidity provider, `min_liquidity` does not do anything and can be set to 0.
Transaction `deadline` is used to set a time after which a transaction can no longer be executed. This limits the "free option" problem, where Ethereum miners can hold signed transactions and execute them based off market movements.
## Remove Liquidity[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#remove-liquidity "Direct link to Remove Liquidity")
Liquidity providers use the `removeLiquidity` function to withdraw their portion of the reserves.
```
exchange.methods.removeLiquidity(amount, min_eth, min_tokens, deadline).send()
```

Liquidity is withdrawn at the same ratio as the reserves at the time of withdrawal. If the exchange rate is bad there is a profitable arbitrage opportunity that will correct the price.
### Parameters[​](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#parameters-1 "Direct link to Parameters")
`amount` specifies the number of liquidity tokens that will be burned. Dividing this amount by the total liquidity token supply gives the percentage of both the ETH and ER20 reserves the provider is withdrawing.
Since exchange rate can change between when a transaction is signed and when it is executed on Ethereum, `min_eth` and `min_tokens` are used to bound the amount this rate can fluctuate.
Same as in `addLiquidity`, `deadline` is used to set a time after which a transaction can no longer be executed.
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/02-pool-liquidity.md)
Was this helpful?
[PreviousConnect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)[NextTrade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
On this page
  * [Create Exchange](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#create-exchange)
  * [Exchange Reserves](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#exchange-reserves)
    * [ETH Reserve](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#eth-reserve)
    * [ERC20 Reserve](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#erc20-reserve)
  * [Add Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#add-liquidity)
    * [Parameters](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#parameters)
  * [Remove Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#remove-liquidity)
    * [Parameters](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity#parameters-1)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/02-pool-liquidity.md)
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
