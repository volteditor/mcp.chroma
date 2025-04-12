use crate::client::get_client;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use mcp_spec::tool::Tool;


#[derive(Debug, Serialize, Deserialize)]
pub struct ListCollectionsRequest {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn chroma_list_collections(request: ListCollectionsRequest) -> Result<Vec<String>> {
    let client = get_client();
    client.list_collections(request.limit, request.offset)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCollectionRequest {
    pub collection_name: String,
    pub embedding_function_name: Option<String>,
    pub metadata: Option<Value>,
    pub space: Option<String>,
    pub ef_construction: Option<i32>,
    pub ef_search: Option<i32>,
    pub max_neighbors: Option<i32>,
    pub num_threads: Option<i32>,
    pub batch_size: Option<i32>,
    pub sync_threshold: Option<i32>,
    pub resize_factor: Option<f32>,
}

pub async fn chroma_create_collection(request: CreateCollectionRequest) -> Result<String> {
    let client = get_client();
    client.create_collection(&request.collection_name, request.metadata)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeekCollectionRequest {
    pub collection_name: String,
    pub limit: usize,
}

pub async fn chroma_peek_collection(request: PeekCollectionRequest) -> Result<Value> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    collection.peek(request.limit)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCollectionInfoRequest {
    pub collection_name: String,
}

pub async fn chroma_get_collection_info(request: GetCollectionInfoRequest) -> Result<Value> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    let count = collection.count()?;
    let sample_documents = collection.peek(3)?;
    
    Ok(serde_json::json!({
        "name": request.collection_name,
        "count": count,
        "sample_documents": sample_documents
    }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetCollectionCountRequest {
    pub collection_name: String,
}

pub async fn chroma_get_collection_count(request: GetCollectionCountRequest) -> Result<usize> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    collection.count()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyCollectionRequest {
    pub collection_name: String,
    pub new_name: Option<String>,
    pub new_metadata: Option<Value>,
    pub ef_search: Option<i32>,
    pub num_threads: Option<i32>,
    pub batch_size: Option<i32>,
    pub sync_threshold: Option<i32>,
    pub resize_factor: Option<f32>,
}

pub async fn chroma_modify_collection(request: ModifyCollectionRequest) -> Result<String> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    collection.modify(request.new_name.clone(), request.new_metadata.clone())?;
    
    let mut modified_aspects = Vec::new();
    if request.new_name.is_some() { modified_aspects.push("name"); }
    if request.new_metadata.is_some() { modified_aspects.push("metadata"); }
    if request.ef_search.is_some() || request.num_threads.is_some() || 
       request.batch_size.is_some() || request.sync_threshold.is_some() || 
       request.resize_factor.is_some() { modified_aspects.push("hnsw"); }
    
    Ok(format!("Successfully modified collection {}: updated {}", 
               request.collection_name, 
               modified_aspects.join(" and ")))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCollectionRequest {
    pub collection_name: String,
}

pub async fn chroma_delete_collection(request: DeleteCollectionRequest) -> Result<String> {
    let client = get_client();
    client.delete_collection(&request.collection_name)?;
    Ok(format!("Successfully deleted collection {}", request.collection_name))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AddDocumentsRequest {
    pub collection_name: String,
    pub documents: Vec<String>,
    pub metadatas: Option<Vec<Value>>,
    pub ids: Option<Vec<String>>,
}

pub async fn chroma_add_documents(request: AddDocumentsRequest) -> Result<String> {
    if request.documents.is_empty() {
        return Err(anyhow!("The 'documents' list cannot be empty."));
    }
    
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    
    let ids = match request.ids {
        Some(ids) => ids,
        None => (0..request.documents.len()).map(|i| i.to_string()).collect(),
    };
    
    let documents_len = request.documents.len();
    collection.add(request.documents.clone(), request.metadatas.clone(), ids)?;
    
    Ok(format!("Successfully added {} documents to collection {}", 
               documents_len, 
               request.collection_name))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDocumentsRequest {
    pub collection_name: String,
    pub query_texts: Vec<String>,
    pub n_results: Option<usize>,
    pub where_filter: Option<Value>,
    pub where_document: Option<Value>,
    pub include: Option<Vec<String>>,
}

pub async fn chroma_query_documents(request: QueryDocumentsRequest) -> Result<Value> {
    if request.query_texts.is_empty() {
        return Err(anyhow!("The 'query_texts' list cannot be empty."));
    }
    
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    
    let n_results = request.n_results.unwrap_or(5);
    let include = request.include.unwrap_or_else(|| vec!["documents".to_string(), "metadatas".to_string(), "distances".to_string()]);
    
    collection.query(request.query_texts, n_results, request.where_filter, request.where_document, include)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDocumentsRequest {
    pub collection_name: String,
    pub ids: Option<Vec<String>>,
    pub where_filter: Option<Value>,
    pub where_document: Option<Value>,
    pub include: Option<Vec<String>>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

pub async fn chroma_get_documents(request: GetDocumentsRequest) -> Result<Value> {
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    
    let include = request.include.unwrap_or_else(|| vec!["documents".to_string(), "metadatas".to_string()]);
    
    collection.get(request.ids, request.where_filter, request.where_document, include, request.limit, request.offset)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDocumentsRequest {
    pub collection_name: String,
    pub ids: Vec<String>,
    pub embeddings: Option<Vec<Vec<f32>>>,
    pub metadatas: Option<Vec<Value>>,
    pub documents: Option<Vec<String>>,
}

pub async fn chroma_update_documents(request: UpdateDocumentsRequest) -> Result<String> {
    if request.ids.is_empty() {
        return Err(anyhow!("The 'ids' list cannot be empty."));
    }
    
    if request.embeddings.is_none() && request.metadatas.is_none() && request.documents.is_none() {
        return Err(anyhow!("At least one of 'embeddings', 'metadatas', or 'documents' must be provided for update."));
    }
    
    let check_length = |name: &str, len: usize| {
        if len != request.ids.len() {
            return Err(anyhow!("Length of '{}' list must match length of 'ids' list.", name));
        }
        Ok(())
    };
    
    if let Some(ref embeddings) = request.embeddings {
        check_length("embeddings", embeddings.len())?;
    }
    
    if let Some(ref metadatas) = request.metadatas {
        check_length("metadatas", metadatas.len())?;
    }
    
    if let Some(ref documents) = request.documents {
        check_length("documents", documents.len())?;
    }
    
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    
    collection.update(request.ids.clone(), request.embeddings, request.metadatas, request.documents)?;
    
    Ok(format!(
        "Successfully updated {} documents in collection '{}'",
        request.ids.len(),
        request.collection_name
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteDocumentsRequest {
    pub collection_name: String,
    pub ids: Vec<String>,
}

pub async fn chroma_delete_documents(request: DeleteDocumentsRequest) -> Result<String> {
    if request.ids.is_empty() {
        return Err(anyhow!("The 'ids' list cannot be empty."));
    }
    
    let client = get_client();
    let collection = client.get_collection(&request.collection_name)?;
    
    collection.delete(request.ids.clone())?;
    
    Ok(format!(
        "Successfully deleted {} documents from collection '{}'",
        request.ids.len(),
        request.collection_name
    ))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtData {
    pub session_id: String,
    pub thought: String,
    pub thought_number: usize,
    pub total_thoughts: usize,
    pub next_thought_needed: bool,
    pub is_revision: Option<bool>,
    pub revises_thought: Option<usize>,
    pub branch_from_thought: Option<usize>,
    pub branch_id: Option<String>,
    pub needs_more_thoughts: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThoughtResponse {
    pub session_id: String,
    pub thought_number: usize,
    pub total_thoughts: usize,
    pub next_thought_needed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

fn validate_thought_data(input_data: &ThoughtData) -> Result<()> {
    if input_data.session_id.is_empty() {
        return Err(anyhow!("Invalid sessionId: must be provided"));
    }
    if input_data.thought.is_empty() {
        return Err(anyhow!("Invalid thought: must be a string"));
    }
    if input_data.thought_number == 0 {
        return Err(anyhow!("Invalid thoughtNumber: must be a number greater than 0"));
    }
    if input_data.total_thoughts == 0 {
        return Err(anyhow!("Invalid totalThoughts: must be a number greater than 0"));
    }
    
    Ok(())
}

pub async fn process_thought(input_data: ThoughtData) -> Result<ThoughtResponse> {
    match validate_thought_data(&input_data) {
        Ok(_) => {
            let total_thoughts = std::cmp::max(input_data.thought_number, input_data.total_thoughts);
            
            Ok(ThoughtResponse {
                session_id: input_data.session_id,
                thought_number: input_data.thought_number,
                total_thoughts,
                next_thought_needed: input_data.next_thought_needed,
                error: None,
                status: None,
            })
        }
        Err(e) => {
            Ok(ThoughtResponse {
                session_id: input_data.session_id,
                thought_number: input_data.thought_number,
                total_thoughts: input_data.total_thoughts,
                next_thought_needed: input_data.next_thought_needed,
                error: Some(e.to_string()),
                status: Some("failed".to_string()),
            })
        }
    }
}

pub fn get_tool_definitions() -> Vec<Tool> {
    let mut tools = Vec::new();
    
    let add_tool = |tools: &mut Vec<Tool>, name: &str, description: &str, schema: Value| {
        tools.push(Tool {
            name: name.to_string(),
            description: description.to_string(),
            input_schema: schema,
        });
    };
    
    add_tool(
        &mut tools,
        "chroma_list_collections",
        "Lists all collections in the ChromaDB instance",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "properties": {
                "limit": {"type": "integer", "description": "Maximum number of collections to return"},
                "offset": {"type": "integer", "description": "Offset for pagination"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_create_collection",
        "Creates a new collection in ChromaDB",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to create"},
                "metadata": {"type": "object", "description": "Optional metadata for the collection"},
                "embedding_function_name": {"type": "string", "description": "Name of the embedding function to use"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_peek_collection",
        "Shows a sample of documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "limit"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to peek"},
                "limit": {"type": "integer", "description": "Number of documents to return"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_get_collection_info",
        "Gets metadata about a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_get_collection_count",
        "Counts the number of documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_modify_collection",
        "Modifies collection properties",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to modify"},
                "new_name": {"type": "string", "description": "New name for the collection"},
                "new_metadata": {"type": "object", "description": "New metadata for the collection"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_delete_collection",
        "Deletes a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection to delete"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_add_documents",
        "Adds documents to a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "documents"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "documents": {"type": "array", "items": {"type": "string"}, "description": "List of documents to add"},
                "metadatas": {"type": "array", "items": {"type": "object"}, "description": "List of metadata objects for documents"},
                "ids": {"type": "array", "items": {"type": "string"}, "description": "List of IDs for documents"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_query_documents",
        "Searches for similar documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "query_texts"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "query_texts": {"type": "array", "items": {"type": "string"}, "description": "List of query texts"},
                "n_results": {"type": "integer", "description": "Number of results to return per query"},
                "where_filter": {"type": "object", "description": "Filter by metadata"},
                "where_document": {"type": "object", "description": "Filter by document content"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_get_documents",
        "Retrieves documents from a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "ids": {"type": "array", "items": {"type": "string"}, "description": "List of document IDs to retrieve"},
                "where_filter": {"type": "object", "description": "Filter by metadata"},
                "where_document": {"type": "object", "description": "Filter by document content"},
                "limit": {"type": "integer", "description": "Maximum number of documents to return"},
                "offset": {"type": "integer", "description": "Offset for pagination"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_update_documents",
        "Updates documents in a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "ids"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "ids": {"type": "array", "items": {"type": "string"}, "description": "List of document IDs to update"},
                "documents": {"type": "array", "items": {"type": "string"}, "description": "List of document contents"},
                "metadatas": {"type": "array", "items": {"type": "object"}, "description": "List of metadata objects"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "chroma_delete_documents",
        "Deletes documents from a collection",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["collection_name", "ids"],
            "properties": {
                "collection_name": {"type": "string", "description": "Name of the collection"},
                "ids": {"type": "array", "items": {"type": "string"}, "description": "List of document IDs to delete"}
            }
        })).unwrap()
    );
    
    add_tool(
        &mut tools,
        "process_thought",
        "Processes a thought in an ongoing session",
        serde_json::to_value(serde_json::json!({
            "type": "object", 
            "required": ["session_id", "thought", "thought_number", "total_thoughts", "next_thought_needed"],
            "properties": {
                "session_id": {"type": "string", "description": "Session identifier"},
                "thought": {"type": "string", "description": "Content of the current thought"},
                "thought_number": {"type": "integer", "description": "Number of this thought in the sequence"},
                "total_thoughts": {"type": "integer", "description": "Total expected thoughts"},
                "next_thought_needed": {"type": "boolean", "description": "Whether another thought is needed"}
            }
        })).unwrap()
    );
    
    tools
}
