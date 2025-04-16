# 🚧 **Backend Architecture Differentiation**

Your overall architecture includes two distinct backend services:

1. **Dream Interpretation Backend**  
   (Axum API + AI Integration + User Database)
2. **Symbolic Ontology Database (MCP Server)**  
   (Ontology management + MCP Protocol)

Below is a precise breakdown differentiating these two clearly:

---

## 🌙 **1. Dream Interpretation Backend (Primary Backend)**

**Purpose:**  
Serves as the main user-facing backend, handling dream inputs and generating symbolic interpretations using an integrated LLM.

**Key Responsibilities:**

- **User Interaction and Data Storage**

  - Receives user-submitted dreams.
  - Stores dreams and generated interpretations in a relational database (Postgres via SQLx).

- **AI Integration**

  - Constructs dynamic prompts based on dreams and retrieved symbolic data.
  - Calls external LLM providers (e.g., DeepSeek via OpenRouter).

- **Interpretation Aggregation**

  - Queries the symbolic ontology database (via MCP) to gather structured symbolic meanings for grounding the LLM-generated interpretation.
  - Integrates this symbolic context into the prompt, ensuring grounded, structured interpretations.

- **Output Management**
  - Stores final interpretations alongside the original dream inputs.
  - Serves interpretations and associated symbolic references back to the frontend (Leptos).

**Typical API Endpoint Example:**

```http
POST /interpret
{
  "dream": "I dreamt of being chased by wolves through the forest."
}
```

---

## 📚 **2. Symbolic Ontology Database (MCP Server)**

**Purpose:**  
Acts as a structured symbolic knowledge repository, exposing well-defined symbolic and archetypal data via the standardized Model Context Protocol (MCP).

**Key Responsibilities:**

- **Ontology Storage and Management**

  - Hosts structured, symbolic data (archetypes, symbols, mythological references, Jungian archetypes).
  - Uses structured formats (OWL/RDF or JSON) for representing symbolic relations.

- **Symbolic Data Retrieval (via MCP)**

  - Implements MCP endpoints to allow querying by other services (notably the interpretation backend).
  - Provides structured JSON-RPC responses containing symbolic meanings, archetypal relationships, or context-specific interpretations.

- **Ontology Extensibility and Governance**
  - Supports adding, updating, and refining symbolic meanings or archetypes over time.
  - Manages ontology versioning and metadata (origin of symbols, cultural contexts, etc.).

**Typical MCP Query Example (JSON-RPC):**

```json
{
  "method": "get_symbol_meanings",
  "params": {
    "symbols": ["wolves", "forest", "chased"]
  }
}
```

**Typical MCP Response:**

```json
{
  "result": {
    "wolves": {
      "meanings": ["threat", "instinct", "group dynamics"],
      "archetypes": ["Shadow", "Wildness"]
    },
    "forest": {
      "meanings": ["the unconscious", "mystery", "personal growth"],
      "archetypes": ["Threshold", "Unconscious Realm"]
    },
    "chased": {
      "meanings": ["anxiety", "avoidance", "inner conflict"],
      "archetypes": ["Shadow Confrontation"]
    }
  }
}
```

---

## 📌 **Key Differentiation (Summary Table)**

| Component             | Dream Interpretation Backend | Symbolic Ontology Database (MCP Server) |
| --------------------- | ---------------------------- | --------------------------------------- |
| **Core Function**     | Interpretation & User data   | Structured symbolic ontology            |
| **Data Handled**      | Dreams, interpretations      | Symbols, archetypes, relationships      |
| **Protocol**          | HTTP API (Axum REST/JSON)    | MCP (JSON-RPC via rmcp SDK)             |
| **Data Storage**      | Relational DB (Postgres)     | Graph DB or JSON-based                  |
| **Data Scope**        | User-specific, session-based | General, universally applicable         |
| **Interactions**      | User frontend & LLM          | Backend services via MCP                |
| **Integration Point** | Calls MCP for symbolic data  | Provides symbolic data to callers       |
| **Development Focus** | Prompt engineering & UX      | Ontology governance & symbolic depth    |

---

## 📖 **Iterating Clearly with Cursor**

When working in Cursor, you could frame your prompts like this:

> "Implement the Axum endpoint (`POST /interpret`) that receives dream inputs, fetches symbolic data via MCP from the symbolic ontology server, and calls the LLM to return structured interpretations."

or,

> "Create an MCP server endpoint using `rmcp` that handles JSON-RPC requests to return symbolic meanings and archetypal associations for given symbols."

This ensures clarity, helping Cursor (or similar tools) focus precisely on the relevant backend component.
