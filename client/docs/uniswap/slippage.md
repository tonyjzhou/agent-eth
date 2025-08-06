# Slippage in Uniswap

## What is Slippage?

Slippage is the difference between the expected price of a trade and the actual price at which the trade is executed. It occurs due to price movements between the time you submit a transaction and when it gets included in a block.

## Calculating Slippage

For Uniswap V2, slippage can be calculated as:

```
Slippage = ((Expected Price - Actual Price) / Expected Price) * 100
```

## Slippage Protection

### V2 Router
The Uniswap V2 Router provides slippage protection through minimum amount parameters:

- `amountOutMin`: Minimum amount of output tokens you're willing to accept
- `amountInMax`: Maximum amount of input tokens you're willing to spend

### Common Slippage Settings

- **0.1%**: For stable pairs (USDC/USDT)
- **0.5%**: For major pairs (ETH/USDC)
- **1-3%**: For volatile or low-liquidity pairs

## Example Calculation

If you're swapping 1 ETH for USDC:
- Expected output: 2000 USDC
- With 2% slippage tolerance: amountOutMin = 1960 USDC
- If actual output < 1960 USDC, transaction reverts

## Best Practices

1. **Check liquidity**: Higher liquidity means lower slippage
2. **Monitor market volatility**: Increase slippage during high volatility
3. **Consider gas costs**: Failed transactions still cost gas
4. **Use appropriate deadlines**: Prevent transactions from being mined much later