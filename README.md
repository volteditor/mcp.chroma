# MCP-Chroma

A ChromaDB MCP server for vector embeddings, collections, and document management.

## Overview

MCP-Chroma provides a server interface for working with [ChromaDB](https://www.trychroma.com/), a vector database for embeddings. It enables operations on collections and documents through a set of tools accessible via the MCP (Model-Controller-Protocol) interface.

## Features

- Collection management (create, list, modify, delete)
- Document operations (add, query, get, update, delete)
- Thought processing for session management
- Multiple client types (HTTP, Cloud, Persistent, Ephemeral)

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/yourusername/mcp-chroma.git
cd mcp-chroma
cargo build --release
```

## Usage

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

### Configuration Options

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

## Tools

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

## Example: Creating a Collection

```json
{
  "collection_name": "my_documents",
  "metadata": {
    "description": "A collection of example documents"
  }
}
```

## Example: Querying Documents

```json
{
  "collection_name": "my_documents",
  "query_texts": ["What are the benefits of vector databases?"],
  "n_results": 3
}
```

## License

[MIT License](LICENSE)

# ChromaDB Tools Testing

This project provides a Docker Compose setup for running and testing ChromaDB integration tools using Cargo's test framework.

## Overview

The setup includes:
- A ChromaDB server with basic authentication
- A test container that runs Cargo integration tests against all ChromaDB tools

## Files

- `docker-compose.yml` - Defines the services: ChromaDB and test container
- `Dockerfile.test` - Builds the test container with Cargo test support
- `tests/integration_tests.rs` - Integration tests for all ChromaDB tools

## Getting Started

### Prerequisites

- Docker
- Docker Compose

### Running the Tests

1. Start the services:

```bash
docker-compose up
```

This will:
- Start the ChromaDB server
- Wait for it to be healthy
- Run the Cargo integration tests against all ChromaDB tools

To run only the ChromaDB server without tests:

```bash
docker-compose up chroma
```

### Test Output

Test results are stored in the `test-results` directory, which is mounted as a volume from the test container.

## Integration Tests

The integration tests in `tests/integration_tests.rs` cover all ChromaDB tools:

1. `test_list_collections` - Tests listing collections
2. `test_create_and_delete_collection` - Tests creating and deleting collections
3. `test_collection_operations` - Tests modifying collections and getting collection info
4. `test_document_operations` - Tests document operations:
   - Adding documents
   - Querying documents
   - Getting documents
   - Filtering documents
   - Updating documents
   - Deleting documents

## Authentication

The ChromaDB server is configured with basic authentication:
- Username: `admin`
- Password: `admin`

These credentials are used by the test container to authenticate with the ChromaDB server.

## Running Tests Locally

You can also run the tests locally if you have a ChromaDB server running:

```bash
# Set environment variables for the ChromaDB server
export CHROMA_CLIENT_TYPE=http
export CHROMA_HOST=localhost
export CHROMA_PORT=8000
export CHROMA_USERNAME=admin
export CHROMA_PASSWORD=admin

# Run the tests
cargo test --test integration_tests -- --nocapture
```

## Data Persistence

ChromaDB data is stored in a Docker volume named `chroma-data`. This volume persists across container restarts.
