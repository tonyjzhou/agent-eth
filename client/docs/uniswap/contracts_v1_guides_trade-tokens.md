# https://docs.uniswap.org/contracts/v1/guides/trade-tokens

[Skip to main content](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#__docusaurus_skipToContent_fallback)
[Uniswap Docs](https://docs.uniswap.org/)
Search`⌘``K`
[Submit Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdjSkZam8KiatL9XACRVxCHjDJjaPGbls77PCXDKFn4JwykXg/viewform)
  * [Concepts](https://docs.uniswap.org/concepts/overview)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
    * [v4 Protocol](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
      * [Overview](https://docs.uniswap.org/contracts/v4/overview)
      * [Deployments](https://docs.uniswap.org/contracts/v4/deployments)
      * [Concepts](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
      * [Quickstart](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Create Pool](https://docs.uniswap.org/contracts/v4/quickstart/create-pool)
        * [Manage Liquidity](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Swap](https://docs.uniswap.org/contracts/v4/quickstart/swap)
        * [Hooks](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Subscriber](https://docs.uniswap.org/contracts/v4/quickstart/subscriber)
      * [Guides](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [v3 Protocol](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [Smart Wallet](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [UniswapX](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [Universal Router](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [Permit2](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [v2 Protocol](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
    * [v1 Protocol](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
      * [Overview](https://docs.uniswap.org/contracts/v1/overview)
      * [Guides](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Connect to Uniswap](https://docs.uniswap.org/contracts/v1/guides/connect-to-uniswap)
        * [Pool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)
        * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Custom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
        * [Iframe Integration](https://docs.uniswap.org/contracts/v1/guides/iframe-integration)
        * [Token Listing](https://docs.uniswap.org/contracts/v1/guides/token-listing)
      * [Technical Reference](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)
        * [Factory](https://docs.uniswap.org/contracts/v1/reference/factory)
        * [Exchange](https://docs.uniswap.org/contracts/v1/reference/exchange)
        * [Interfaces](https://docs.uniswap.org/contracts/v1/reference/interfaces)
  * [SDKs](https://docs.uniswap.org/sdk/v4/overview)
  * [APIs](https://docs.uniswap.org/api/subgraph/overview)


  * [Home](https://docs.uniswap.org/)
  * [Contracts](https://docs.uniswap.org/contracts/v4/overview)
  * v1 Protocol
  * Guides
  * [Trade Tokens](https://docs.uniswap.org/contracts/v1/guides/trade-tokens)


On this page
# Trade Tokens
In Uniswap, there is a separate exchange contract for each ERC20 token. These exchanges hold reserves of both ETH and their associated ERC20. Instead of waiting to be matched in an order-book, users can make trades against the reserves at any time. Reserves are pooled between a decentralized network of liquidity providers who collect fees on every trade.
Pricing is automatic, based on the `x * y = k` market making formula which automatically adjusts prices based off the relative sizes of the two reserves and the size of the incoming trade. Since all tokens share ETH as a common pair, it is used as an intermediary asset for direct trading between any ERC20 ⇄ ERC20 pair.
## ETH ⇄ ERC20 Calculations[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#eth--erc20-calculations "Direct link to ETH ⇄ ERC20 Calculations")
The variables needed to determine price when trading between ETH and ERC20 tokens is:
  * ETH reserve size of the ERC20 exchange
  * ERC20 reserve size of the ERC20 exchange
  * Amount sold (input) or amount bought (output)


### Amount Bought (sell order)[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-bought-sell-order "Direct link to Amount Bought \(sell order\)")
For sell orders (exact input), the amount bought (output) is calculated:
```
// Sell ETH for ERC20const inputAmount = userInputEthValueconst inputReserve = web3.eth.getBalance(exchangeAddress)const outputReserve = tokenContract.methods.balanceOf(exchangeAddress).call()// Sell ERC20 for ETHconst inputAmount = userInputTokenValueconst inputReserve = tokenContract.methods.balanceOf(exchangeAddress).call()const outputReserve = web3.eth.getBalance(exchangeAddress)// Output amount boughtconst numerator = inputAmount * outputReserve *997const denominator = inputReserve *1000+ inputAmount *997const outputAmount = numerator / denominator
```

### Amount Sold (buy order)[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-sold-buy-order "Direct link to Amount Sold \(buy order\)")
For buy orders (exact output), the cost (input) is calculated:
```
// Buy ERC20 with ETHconst outputAmount = userInputTokenValueconst inputReserve = web3.eth.getBalance(exchangeAddress)const outputReserve = tokenContract.methods.balanceOf(exchangeAddress).call()// Buy ETH with ERC20const outputAmount = userInputEthValueconst inputReserve = tokenContract.methods.balanceOf(exchangeAddress).call()const outputReserve = web3.eth.getBalance(exchangeAddress)// Costconst numerator = outputAmount * inputReserve *1000const denominator =(outputReserve - outputAmount)*997const inputAmount = numerator / denominator +1
```

### Liquidity Provider Fee[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#liquidity-provider-fee "Direct link to Liquidity Provider Fee")
There is a 0.3% liquidity provider fee built into the price formula. This can be calculated:
```
fee = inputAmount *0.003
```

### Exchange Rate[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#exchange-rate "Direct link to Exchange Rate")
The exchange rate is simply the output amount divided by the input amount.
```
const rate = outputAmount / inputAmount
```

## ERC20 ⇄ ERC20 Calculations[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#erc20--erc20-calculations "Direct link to ERC20 ⇄ ERC20 Calculations")
The variables needed to determine price when trading between two ERC20 tokens is:
  * ETH reserve size of the input ERC20 exchange
  * ERC20 reserve size of the input ERC20 exchange
  * ETH reserve size of the output ERC20 exchange
  * ERC20 reserve size of the output ERC20 exchange
  * Amount sold (input) or amount bought (output)


### Amount Bought (sell order)[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-bought-sell-order-1 "Direct link to Amount Bought \(sell order\)")
For sell orders (exact input), the amount bought (output) is calculated:
```
// TokenA (ERC20) to ETH conversionconst inputAmountA = userInputTokenAValueconst inputReserveA = tokenContractA.methods.balanceOf(exchangeAddressA).call()const outputReserveA = web3.eth.getBalance(exchangeAddressA)const numeratorA = inputAmountA * outputReserveA *997const denominatorA = inputReserveA *1000+ inputAmountA *997const outputAmountA = numeratorA / denominatorA// ETH to TokenB conversionconst inputAmountB = outputAmountAconst inputReserveB = web3.eth.getBalance(exchangeAddressB)const outputReserveB = tokenContract.methods.balanceOf(exchangeAddressB).call()const numeratorB = inputAmountB * outputReserveB *997const denominatorB = inputReserveB *1000+ inputAmountB *997const outputAmountB = numeratorB / denominatorB
```

### Amount Sold (buy order)[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-sold-buy-order-1 "Direct link to Amount Sold \(buy order\)")
For buy orders (exact output), the cost (input) is calculated:
```
// Buy TokenB with ETHconst outputAmountB = userInputTokenBValueconst inputReserveB = web3.eth.getBalance(exchangeAddressB)const outputReserveB = tokenContractB.methods.balanceOf(exchangeAddressB).call()// Costconst numeratorB = outputAmountB * inputReserveB *1000const denominatorB =(outputReserveB - outputAmountB)*997const inputAmountB = numeratorB / denominatorB +1// Buy ETH with TokenAconst outputAmountA = userInputEthValueconst inputReserveA = tokenContractA.methods.balanceOf(exchangeAddressA).call()const outputReserveA = web3.eth.getBalance(exchangeAddressA)// Costconst numeratorA = outputAmountA * inputReserveA *1000const denominatorA =(outputReserveA - outputAmountA)*997const inputAmountA = numeratorA / denominatorA +1
```

### Liquidity Provider Fee[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#liquidity-provider-fee-1 "Direct link to Liquidity Provider Fee")
There is a 0.30% liquidity provider fee to swap from TokenA to ETH on the input exchange. There is another 0.3% liquidity provider fee to swap the remaining ETH to TokenB.
```
const exchangeAFee = inputAmountA *0.003const exchangeBFee = inputAmountB *0.003
```

Since users only inputs Token A, it can be represented to them as:
```
const combinedFee = inputAmountA *0.00591
```

### Exchange Rate[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#exchange-rate-1 "Direct link to Exchange Rate")
The exchange rate is simply the output amount divided by the input amount.
```
const rate = outputAmountB / inputAmountA
```

## Deadlines[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#deadlines "Direct link to Deadlines")
Many Uniswap functions include a transaction `deadline` that sets a time after which a transaction can no longer be executed. This limits miners holding signed transactions for extended durations and executing them based off market movements. It also reduces uncertainty around transactions that take a long time to execute due to issues with gas price.
Deadlines are calculated by adding the desired amount of time (in seconds) to the latest Ethereum block timestamp.
```
web3.eth.getBlock('latest',(error, block)=>{ deadline = block.timestamp+300// transaction expires in 300 seconds (5 minutes)})
```

## Recipients[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#recipients "Direct link to Recipients")
Uniswap allows traders to swap tokens and transfer the output to a new `recipient` address. This allows for a type of payment where the payer sends one token and the payee receives another.
## ETH ⇄ ERC20 Trades[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#eth--erc20-trades "Direct link to ETH ⇄ ERC20 Trades")
Coming soon...
## ERC20 ⇄ ERC20 Trades[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#erc20--erc20-trades "Direct link to ERC20 ⇄ ERC20 Trades")
Coming soon...
## Custom Pools[​](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#custom-pools "Direct link to Custom Pools")
Coming soon...
[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/03-trade-tokens.md)
Was this helpful?
[PreviousPool Liquidity](https://docs.uniswap.org/contracts/v1/guides/pool-liquidity)[NextCustom Linking](https://docs.uniswap.org/contracts/v1/guides/custom-linking)
On this page
  * [ETH ⇄ ERC20 Calculations](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#eth--erc20-calculations)
    * [Amount Bought (sell order)](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-bought-sell-order)
    * [Amount Sold (buy order)](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-sold-buy-order)
    * [Liquidity Provider Fee](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#liquidity-provider-fee)
    * [Exchange Rate](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#exchange-rate)
  * [ERC20 ⇄ ERC20 Calculations](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#erc20--erc20-calculations)
    * [Amount Bought (sell order)](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-bought-sell-order-1)
    * [Amount Sold (buy order)](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#amount-sold-buy-order-1)
    * [Liquidity Provider Fee](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#liquidity-provider-fee-1)
    * [Exchange Rate](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#exchange-rate-1)
  * [Deadlines](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#deadlines)
  * [Recipients](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#recipients)
  * [ETH ⇄ ERC20 Trades](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#eth--erc20-trades)
  * [ERC20 ⇄ ERC20 Trades](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#erc20--erc20-trades)
  * [Custom Pools](https://docs.uniswap.org/contracts/v1/guides/trade-tokens#custom-pools)


[Edit this page](https://github.com/uniswap/uniswap-docs/tree/main/docs/contracts/v1/guides/03-trade-tokens.md)
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
