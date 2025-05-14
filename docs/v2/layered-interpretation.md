### 🧩 Phase 1: Extend Symbol Model with Layered Interpretation

> ✅ **Prompt:**

> We are upgrading the `Symbol` model to support **layered symbolic interpretation** in v2 of the Symbol Ontology MCP server.
>
> ### Step 1: Create a new struct:
>
> ```rust
> pub struct InterpretationLayer {
>     pub universal: Option<String>,
>     pub cultural: Option<String>,
>     pub personal: Option<String>,
> }
> ```
>
> ### Step 2: In the `Symbol` model:
>
> - Refactor the `interpretations` field to store a `HashMap<String, InterpretationLayer>`, where the key is the interpretation context (e.g., "dreams", "mythology").
> - Update the struct and derive traits accordingly.
> - Ensure deserialization from existing seed data fails gracefully or ignores missing layers for now.
>
> ### Step 3: Add default layer fallback logic:
>
> - If only a flat string is provided for an interpretation, treat it as the `universal` field.
> - Preserve compatibility with older datasets that use `HashMap<String, String>`.
>
> Use this phase to define, migrate, and verify the model changes without touching the MCP API or database migrations yet.

---

### 🔌 Phase 2: Update Repositories and Seed Data Handling

> ✅ **Prompt:**

> Now that `Symbol` uses `InterpretationLayer`, update the **repository layer** to:
>
> 1. Properly parse `HashMap<String, InterpretationLayer>` in both memory and Postgres implementations.
> 2. Ensure compatibility with old `String` values where present — auto-convert to `InterpretationLayer { universal: Some(str), .. }` at load-time.
> 3. Validate and document how seed data in JSON and CSV can optionally include layered interpretations.
> 4. Ensure symbol lookup, search, and category filtering still function as expected.
>
> Maintain backward compatibility where needed — do not break old loading logic, just enhance when `InterpretationLayer` is present.

---

### 🌐 Phase 3: Add LLM-Oriented MCP Endpoint for Layered Context

> ✅ **Prompt:**

> Implement a **new MCP API endpoint** for LLM prompt usage:
>
> ```
> GET /api/symbols/:id/prompt-context
> ```
>
> ### Requirements:
>
> - Given a symbol ID or name, return a JSON object with:
>
>   - The name, category, and description
>   - Each interpretation layer (`universal`, `cultural`, `personal`) from all contexts
>   - Related symbols and their categories
>
> - Format should be prompt-friendly: minimal nesting, clean keys, and optional compact version (query param `?compact=true`)
> - If `InterpretationLayer` is missing for a context, gracefully fall back to `universal`
>
> Ensure all logic is modular (e.g., `fn build_prompt_context(symbol: &Symbol) -> PromptContextResponse`) for reuse and testing.

---

### 📘 Phase 4 (Optional): Add Tests and Docs

> ✅ **Prompt:**

> Write tests for:
>
> - Deserialization of `InterpretationLayer`
> - Symbol repository fallback behavior
> - The new `/prompt-context` endpoint (check both rich and compact formats)
>
> Also update:
>
> - The MCP API Docs with examples of the new endpoint
> - The internal README explaining `InterpretationLayer` and future plans
