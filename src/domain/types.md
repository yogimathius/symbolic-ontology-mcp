# Code Quality Guidelines for Symbol Ontology MCP

## Dead Code and Unused Code Handling

### Principles

1. **No `#[allow(dead_code)]` Attributes**

   - Never use `#[allow(dead_code)]` to silence compiler warnings
   - Unused code should either be properly used or removed
   - Exception: Test-only code may be marked with `#[cfg(test)]`

2. **Clean Trait Design**

   - Define minimal traits with only required methods
   - No default implementations that return "NotImplemented" errors
   - All trait methods should be actually used in the codebase

3. **Error Handling**

   - Use proper error constructors from the RMCP library
   - Create specific error types when needed
   - Be explicit about error conversion between layers

4. **Clean Imports**
   - Don't import unused modules or types
   - When re-exporting, only re-export what is actually used

### Implementation Guidance

1. **Repository Pattern**

   - Only define methods that are actually needed and implemented
   - Create separate traits for core vs. extended functionality if needed
   - Use proper abstractions to avoid code duplication

2. **Compiler Warnings**

   - Treat all compiler warnings as errors during development
   - Fix warnings immediately rather than silencing them
   - Run `cargo check` frequently during development

3. **Testing**

   - Write tests for all public APIs
   - Use test-specific modules marked with `#[cfg(test)]`
   - Prefer explicit imports in test modules rather than `use super::*`

4. **Documentation**
   - Document all public APIs
   - Ensure examples in documentation are actually correct and tested
   - Maintain documentation consistency with implementation

By following these guidelines, we ensure a clean, maintainable codebase without unnecessary or dead code.
