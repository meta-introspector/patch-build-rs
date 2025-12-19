#!/usr/bin/env bash
# Audit Fix Tool Runner
# Usage: ./scripts/run_audit_fix.sh [command] [args...]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Check if binary exists, if not build it
AUDIT_FIX="$PROJECT_ROOT/target/debug/audit-fix"

if [ ! -f "$AUDIT_FIX" ]; then
    echo "🔨 Building audit-fix tool..."
    cd "$PROJECT_ROOT"
    cargo build -p introspector_core --bin audit-fix
fi

# Run the tool
"$AUDIT_FIX" "$@"
