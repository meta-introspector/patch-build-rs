#!/bin/bash
# Miner Node Coordination for Free Tier API Aggregation
# Coordinates multiple nodes to collect "free air" from API providers

echo "🔗 Starting Solana Miner Node Coordination"

# List of free tier API providers
API_PROVIDERS=(
    "api.mainnet-beta.solana.com"
    "solana-api.projectserum.com" 
    "api.solana.fm"
    "rpc.ankr.com/solana"
)

# Node coordination directory
COORD_DIR="/tmp/solana_coordination"
mkdir -p "$COORD_DIR"

# Function to sample blocks from provider
sample_blocks() {
    local provider=$1
    local node_id=$2
    
    echo "📡 Node $node_id sampling from $provider"
    
    # Use free tier limits efficiently
    curl -s -X POST \
        -H "Content-Type: application/json" \
        -d '{"jsonrpc":"2.0","id":1,"method":"getRecentBlockhash"}' \
        "https://$provider" > "$COORD_DIR/blocks_$node_id.json"
    
    # Extract and lift integer codes
    grep -o '[0-9]\+' "$COORD_DIR/blocks_$node_id.json" | head -20 > "$COORD_DIR/ints_$node_id.txt"
    
    echo "✅ Node $node_id collected $(wc -l < "$COORD_DIR/ints_$node_id.txt") integers"
}

# Coordinate multiple nodes
for i in {1..4}; do
    provider=${API_PROVIDERS[$((i-1))]}
    sample_blocks "$provider" "$i" &
done

# Wait for all nodes to complete
wait

echo "🔄 Aggregating collected data..."

# Combine all integer codes
cat "$COORD_DIR"/ints_*.txt | sort -n | uniq > "$COORD_DIR/aggregated_ints.txt"

# Generate Rust macro from aggregated data
echo "macro_rules! mined_blockchain_data {" > "$COORD_DIR/generated_macro.rs"
echo "    () => {" >> "$COORD_DIR/generated_macro.rs"
echo "        vec![$(paste -sd, "$COORD_DIR/aggregated_ints.txt")]" >> "$COORD_DIR/generated_macro.rs"
echo "    };" >> "$COORD_DIR/generated_macro.rs"
echo "}" >> "$COORD_DIR/generated_macro.rs"

echo "🎯 Generated macro with $(wc -l < "$COORD_DIR/aggregated_ints.txt") blockchain integers"
echo "📁 Saved to: $COORD_DIR/generated_macro.rs"

# Cleanup
rm -f "$COORD_DIR"/blocks_*.json "$COORD_DIR"/ints_*.txt

echo "✨ Miner coordination complete - free air collected!"
