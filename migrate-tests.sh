#!/bin/bash

# Create necessary test directories if they don't exist
mkdir -p ontology-core/tests/domain
mkdir -p ontology-core/tests/db/repository
mkdir -p ontology-api-server/tests/api
mkdir -p symbol-mcp-client/tests/mcp

# Migrate common fixtures
cp -r tests/common ontology-core/tests/

# Migrate domain tests
cp tests/domain/symbol_tests.rs ontology-core/tests/domain/
cp tests/domain/symbolset_tests.rs ontology-core/tests/domain/
cp tests/domain/ontology_tests.rs ontology-core/tests/domain/

# Migrate db tests
cp tests/db/repository/symbol_repository_tests.rs ontology-core/tests/db/repository/
cp tests/db/repository/symbol_set_repository_tests.rs ontology-core/tests/db/repository/
cp tests/db/repository/factory_tests.rs ontology-core/tests/db/repository/

# Migrate MCP service tests
cp tests/mcp/service_tests.rs symbol-mcp-client/tests/mcp/

# Create lib.rs files for each test directory
cat > ontology-core/tests/lib.rs << EOF
pub mod common;
pub mod domain;
pub mod db;
EOF

cat > ontology-core/tests/db/mod.rs << EOF
pub mod repository;
EOF

cat > ontology-core/tests/domain/mod.rs << EOF
pub mod symbol_tests;
pub mod symbolset_tests;
pub mod ontology_tests;
EOF

cat > ontology-api-server/tests/lib.rs << EOF
pub mod api;
EOF

cat > symbol-mcp-client/tests/lib.rs << EOF
pub mod mcp;
EOF

cat > symbol-mcp-client/tests/mcp/mod.rs << EOF
pub mod service_tests;
EOF

echo "Test migration completed. You'll need to update the imports in each test file to use the new crate structure." 