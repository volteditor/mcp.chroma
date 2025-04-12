# 🧠 mcp.chroma

A ChromaDB MCP server for vector embeddings, collections, and document management.

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![MCP](https://img.shields.io/badge/MCP-Protocol-blue?style=for-the-badge)](https://modelcontextprotocol.io/)
[![ChromaDB](https://img.shields.io/badge/ChromaDB-Vector_Database-purple?style=for-the-badge)](https://www.trychroma.com/)

## 📋 Overview

This MCP server provides a interface for working with [ChromaDB](https://www.trychroma.com/), a vector database for embeddings. It enables operations on collections and documents through a set of tools accessible via the MCP (Model-Controller-Protocol) interface.

## ✨ Features

- 📊 Collection management (create, list, modify, delete)
- 📄 Document operations (add, query, get, update, delete)
- 🧠 Thought processing for session management
- 🔌 Multiple client types (HTTP, Cloud, Persistent, Ephemeral)

## 🚀 Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/yourusername/mcp-chroma.git
cd mcp-chroma
cargo build --release
```

## 🛠️ Usage

### Setting Up Environment

Create a `.chroma_env` file in your project directory with the configuration parameters:

```
CHROMA_CLIENT_TYPE=ephemeral
CHROMA_HOST=localhost
CHROMA_PORT=8000
```

### Running the Server

```bash
# Run with default configuration
./mcp-chroma

# Run with specific client type
./mcp-chroma --client-type http --host localhost --port 8000

# Run with persistent storage
./mcp-chroma --client-type persistent --data-dir ./chroma_data
```

### Available Client Types

1. **Ephemeral**: In-memory client (default)
2. **Persistent**: Local storage client with persistence
3. **HTTP**: Remote client via HTTP
4. **Cloud**: Managed cloud client

## ⚙️ Configuration Options

| Option | Environment Variable | Description | Default |
|--------|---------------------|-------------|---------|
| `--client-type` | `CHROMA_CLIENT_TYPE` | Type of client (ephemeral, persistent, http, cloud) | ephemeral |
| `--data-dir` | `CHROMA_DATA_DIR` | Directory for persistent storage | None |
| `--host` | `CHROMA_HOST` | Host for HTTP client | None |
| `--port` | `CHROMA_PORT` | Port for HTTP client | None |
| `--ssl` | `CHROMA_SSL` | Use SSL for HTTP client | true |
| `--tenant` | `CHROMA_TENANT` | Tenant for cloud client | None |
| `--database` | `CHROMA_DATABASE` | Database for cloud client | None |
| `--api-key` | `CHROMA_API_KEY` | API key for cloud client | None |
| `--dotenv-path` | `CHROMA_DOTENV_PATH` | Path to .env file | .chroma_env |

## 🧰 Tools

### Collection Tools

- `chroma_list_collections`: List all collections
- `chroma_create_collection`: Create a new collection
- `chroma_peek_collection`: Preview documents in a collection
- `chroma_get_collection_info`: Get metadata about a collection
- `chroma_get_collection_count`: Count documents in a collection
- `chroma_modify_collection`: Update collection properties
- `chroma_delete_collection`: Delete a collection

### Document Tools

- `chroma_add_documents`: Add documents to a collection
- `chroma_query_documents`: Search for similar documents
- `chroma_get_documents`: Retrieve documents from a collection
- `chroma_update_documents`: Update existing documents
- `chroma_delete_documents`: Delete documents from a collection

### Thought Processing

- `process_thought`: Process thoughts in an ongoing session

## 📝 Examples

### Creating a Collection

```json
{
  "collection_name": "my_documents",
  "metadata": {
    "description": "A collection of example documents"
  }
}
```

### Querying Documents

```json
{
  "collection_name": "my_documents",
  "query_texts": ["What are the benefits of vector databases?"],
  "n_results": 3
}
```

## 🔧 Integration with Claude

You can use MCP-Chroma with Claude by setting up a configuration like:

```json
{
  "mcpServers": {
    "chroma": {
      "command": "mcp-chroma",
      "args": [
        "--client-type",
        "http",
        "--host",
        "localhost",
        "--port",
        "8000"
      ],
      "env": {
        "CHROMA_API_KEY": "<YOUR_API_KEY>"
      }
    }
  }
}
```

## 🖥️ Integration with Cursor

To use MCP-Chroma with Cursor, add the following to your `.vscode/mcp.json` file:

```json
{
  "mcp": {
    "inputs": [
      {
        "type": "promptString",
        "id": "chroma_api_key",
        "description": "ChromaDB API Key",
        "password": true
      }
    ],
    "servers": {
      "chroma": {
        "command": "mcp-chroma",
        "args": [
          "--client-type",
          "http",
          "--host",
          "localhost",
          "--port",
          "8000"
        ],
        "env": {
          "CHROMA_API_KEY": "${input:chroma_api_key}"
        }
      }
    }
  }
}
```

## 📄 License

[MIT License](LICENSE)
