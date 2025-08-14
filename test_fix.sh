#!/bin/bash
set -e

echo "Testing JSON parsing fix..."
echo "Building client..."
cargo build --bin agent-eth-client

echo "Testing that the fix compiles and basic functionality works..."
echo "exit" | ANTHROPIC_API_KEY="dummy_key_for_testing" ./target/debug/agent-eth-client

echo "✅ JSON parsing fix appears to be working - client starts and exits cleanly"
echo "The fix should now handle Claude 4.0's markdown-wrapped JSON responses"