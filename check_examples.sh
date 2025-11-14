#!/bin/bash

# Array of new example directories
examples=(
    "04-networking/06-websocket"
    # "04-networking/07-grpc"  # Skipped: Requires protoc (Protocol Buffers compiler)
    "04-networking/08-quic"
    "04-networking/09-graphql"
    "05-os/03-process-management"
    "05-os/04-signals"
    "05-os/05-filesystem-watching"
    "05-os/06-memory-mapping"
    "05-os/07-permissions"
    "06-libraries/08-rayon"
    "06-libraries/09-tokio-advanced"
    "06-libraries/10-tracing"
)

failed=()

for dir in "${examples[@]}"; do
    echo "Checking $dir..."
    if cd "$dir" 2>/dev/null; then
        if ! cargo check 2>&1 | tail -5 | grep -q "Finished"; then
            failed+=("$dir")
            echo "✗ $dir has errors"
        else
            echo "✓ $dir passed"
        fi
        cd - > /dev/null
    else
        echo "✗ $dir not found"
        failed+=("$dir (not found)")
    fi
done

echo ""
echo "Summary:"
if [ ${#failed[@]} -eq 0 ]; then
    echo "All examples passed!"
else
    echo "Failed examples:"
    printf '%s\n' "${failed[@]}"
fi
