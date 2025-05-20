#!/bin/bash
# Script to check for any remaining dream references in the codebase

echo "Checking for 'dream' references in the source code..."

# Check source code files
echo "Source code checks:"
grep -r --include="*.rs" -i "dream" . || echo "No 'dream' references found in Rust code"

# Check Cargo.toml files
echo -e "\nCargo.toml checks:"
grep -r --include="Cargo.toml" -i "dream" . || echo "No 'dream' references found in Cargo.toml files"

# Check markdown files
echo -e "\nDocumentation checks:"
grep -r --include="*.md" -i "dream" . || echo "No 'dream' references found in documentation"

# Check for the dream-mcp-client directory
echo -e "\nDirectory checks:"
if [ -d "dream-mcp-client" ]; then
    echo "WARNING: dream-mcp-client directory still exists"
else
    echo "dream-mcp-client directory not found (good)"
fi

echo -e "\nRefactoring check complete. If any 'dream' references were found, please update them." 