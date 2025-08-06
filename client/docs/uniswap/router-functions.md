# Uniswap Router Functions

## V2 Router Key Functions

### exactInput vs exactOutput

#### swapExactETHForTokens
- **Purpose**: Swap exact amount of ETH for tokens
- **Input**: Exact ETH amount
- **Output**: Variable token amount (minimum guaranteed)
- **Parameters**:
  - `amountOutMin`: Minimum tokens to receive
  - `path`: Array of token addresses (ETH -> Token)
  - `to`: Recipient address
  - `deadline`: Transaction deadline

#### swapTokensForExactETH  
- **Purpose**: Swap tokens for exact amount of ETH
- **Input**: Variable token amount (maximum specified)
- **Output**: Exact ETH amount
- **Parameters**:
  - `amountOut`: Exact ETH amount to receive
  - `amountInMax`: Maximum tokens to spend
  - `path`: Array of token addresses (Token -> ETH)
  - `to`: Recipient address
  - `deadline`: Transaction deadline

#### swapExactTokensForTokens
- **Purpose**: Swap exact amount of one token for another
- **Input**: Exact token amount
- **Output**: Variable token amount (minimum guaranteed)

#### swapTokensForExactTokens
- **Purpose**: Swap tokens for exact amount of another token
- **Input**: Variable token amount (maximum specified) 
- **Output**: Exact token amount

## Key Differences

| Function Type | Input Amount | Output Amount | Use Case |
|---------------|--------------|---------------|----------|
| exactInput | Fixed | Variable (min) | "Spend exactly X, get at least Y" |
| exactOutput | Variable (max) | Fixed | "Get exactly Y, spend at most X" |

## Router Address

- **V2 Router**: `0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D`
- **V3 SwapRouter**: `0xE592427A0AEce92De3Edee1F18E0157C05861564`

## Example Usage

```solidity
// Swap exactly 1 ETH for USDC (minimum 1900 USDC)
uniswapV2Router.swapExactETHForTokens{value: 1 ether}(
    1900 * 10**6, // amountOutMin (USDC has 6 decimals)
    path, // [WETH, USDC]
    msg.sender, // recipient
    block.timestamp + 300 // 5 minute deadline
);
```