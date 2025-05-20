#!/bin/bash
# Rename dream-mcp-client to symbol-mcp-client

set -e  # Exit on error

echo "Renaming dream-mcp-client to symbol-mcp-client..."

# 1. Create new directory if it doesn't exist
if [ ! -d "symbol-mcp-client" ]; then
    mkdir -p symbol-mcp-client
    echo "Created symbol-mcp-client directory"
fi

# 2. Copy all files from dream-mcp-client to symbol-mcp-client
cp -r dream-mcp-client/* symbol-mcp-client/
echo "Copied files from dream-mcp-client to symbol-mcp-client"

# 3. Update references in Cargo.toml files
find . -name "Cargo.toml" -type f -exec sed -i '' 's/dream-mcp-client/symbol-mcp-client/g' {} \;
find . -name "Cargo.toml" -type f -exec sed -i '' 's/dream-mcp/symbol-mcp/g' {} \;
find . -name "Cargo.toml" -type f -exec sed -i '' 's/"dream"/"symbol"/g' {} \;
echo "Updated references in Cargo.toml files"

# 4. Update main workspace Cargo.toml
sed -i '' 's/dream-ontology-mcp/symbol-ontology-mcp/g' Cargo.toml
echo "Updated main workspace Cargo.toml"

# 5. Update references in source files
find . -name "*.rs" -type f -exec sed -i '' 's/dream-mcp-client/symbol-mcp-client/g' {} \;
find . -name "*.rs" -type f -exec sed -i '' 's/dream_mcp_client/symbol_mcp_client/g' {} \;
find . -name "*.rs" -type f -exec sed -i '' 's/dream-mcp/symbol-mcp/g' {} \;
find . -name "*.rs" -type f -exec sed -i '' 's/dream_mcp/symbol_mcp/g' {} \;
find . -name "*.rs" -type f -exec sed -i '' 's/DreamMcp/SymbolMcp/g' {} \;
find . -name "*.rs" -type f -exec sed -i '' 's/DREAM_MCP/SYMBOL_MCP/g' {} \;
echo "Updated references in source files"

# 6. Update environment variable references in symbol-mcp-client
find symbol-mcp-client -type f -exec sed -i '' 's/DREAM_MCP_API_KEY/SYMBOL_MCP_API_KEY/g' {} \;
find symbol-mcp-client -type f -exec sed -i '' 's/DREAM_MCP_API_URL/SYMBOL_MCP_API_URL/g' {} \;
find symbol-mcp-client -type f -exec sed -i '' 's/Dream Ontology/Symbol Ontology/g' {} \;
find symbol-mcp-client -type f -exec sed -i '' 's/dreamontology/symbolontology/g' {} \;
echo "Updated environment variable references"

# 7. Update README files
find . -name "*.md" -type f -exec sed -i '' 's/Dream Ontology/Symbol Ontology/g' {} \;
find . -name "*.md" -type f -exec sed -i '' 's/dream-mcp-client/symbol-mcp-client/g' {} \;
find . -name "*.md" -type f -exec sed -i '' 's/dream-mcp/symbol-mcp/g' {} \;
find . -name "*.md" -type f -exec sed -i '' 's/dream symbol/symbol/g' {} \;
echo "Updated README files"

# 8. Update specific struct names in seeder
sed -i '' 's/DreamDictionaryRow/SymbolDictionaryRow/g' src/bin/ontology_seeder.rs
sed -i '' 's/DictionaryOfDreamsRow/DictionaryOfSymbolsRow/g' src/bin/ontology_seeder.rs
sed -i '' 's/DreamsInterpretationsRow/SymbolsInterpretationsRow/g' src/bin/ontology_seeder.rs
sed -i '' 's/CleanedDreamInterpretationsRow/CleanedSymbolInterpretationsRow/g' src/bin/ontology_seeder.rs
sed -i '' 's/SampleDreamSymbolRow/SampleSymbolRow/g' src/bin/ontology_seeder.rs
sed -i '' 's/"dream"/"symbol"/g' src/bin/ontology_seeder.rs
echo "Updated struct names in seeder"

# 9. Update database references
find . -name "*.rs" -type f -exec sed -i '' 's/dream_ontology/symbol_ontology/g' {} \;
echo "Updated database references"

# Do not delete the original directory yet
echo "Completed renaming process. The original dream-mcp-client directory is still present."
echo "Please review the changes and then run 'rm -rf dream-mcp-client' to remove the original directory." 