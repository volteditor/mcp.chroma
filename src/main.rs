mod client;
mod config;
mod tools;

use anyhow::Result;
use clap::Parser;
use config::Config;
use mcp_server::{router::Router, Server, router::RouterService, ByteTransport};
use mcp_spec::{
    content::Content,
    handler::{PromptError, ResourceError, ToolError},
    prompt::Prompt,
    protocol::ServerCapabilities,
    resource::Resource,
    tool::Tool,
};
use serde_json::Value;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use tokio::io::{stdin, stdout};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct ChromaRouter {}

impl ChromaRouter {
    fn new(_config: Config) -> Self {
        Self {}
    }
    
    async fn dispatch_method(&self, name: &str, args: Value) -> Result<Value, anyhow::Error> {
        match name {
            "chroma_list_collections" => {
                let args: tools::ListCollectionsRequest = serde_json::from_value(args)?;
                let result = tools::chroma_list_collections(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_create_collection" => {
                let args: tools::CreateCollectionRequest = serde_json::from_value(args)?;
                let result = tools::chroma_create_collection(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_peek_collection" => {
                let args: tools::PeekCollectionRequest = serde_json::from_value(args)?;
                let result = tools::chroma_peek_collection(args).await?;
                Ok(result)
            }
            "chroma_get_collection_info" => {
                let args: tools::GetCollectionInfoRequest = serde_json::from_value(args)?;
                let result = tools::chroma_get_collection_info(args).await?;
                Ok(result)
            }
            "chroma_get_collection_count" => {
                let args: tools::GetCollectionCountRequest = serde_json::from_value(args)?;
                let result = tools::chroma_get_collection_count(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_modify_collection" => {
                let args: tools::ModifyCollectionRequest = serde_json::from_value(args)?;
                let result = tools::chroma_modify_collection(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_delete_collection" => {
                let args: tools::DeleteCollectionRequest = serde_json::from_value(args)?;
                let result = tools::chroma_delete_collection(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_add_documents" => {
                let args: tools::AddDocumentsRequest = serde_json::from_value(args)?;
                let result = tools::chroma_add_documents(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_query_documents" => {
                let args: tools::QueryDocumentsRequest = serde_json::from_value(args)?;
                let result = tools::chroma_query_documents(args).await?;
                Ok(result)
            }
            "chroma_get_documents" => {
                let args: tools::GetDocumentsRequest = serde_json::from_value(args)?;
                let result = tools::chroma_get_documents(args).await?;
                Ok(result)
            }
            "chroma_update_documents" => {
                let args: tools::UpdateDocumentsRequest = serde_json::from_value(args)?;
                let result = tools::chroma_update_documents(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "chroma_delete_documents" => {
                let args: tools::DeleteDocumentsRequest = serde_json::from_value(args)?;
                let result = tools::chroma_delete_documents(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            "process_thought" => {
                let args: tools::ThoughtData = serde_json::from_value(args)?;
                let result = tools::process_thought(args).await?;
                serde_json::to_value(result).map_err(Into::into)
            }
            _ => Err(anyhow::anyhow!("Method not found: {}", name)),
        }
    }
}

impl Router for ChromaRouter {
    fn name(&self) -> String {
        "mcp-chroma".to_string()
    }

    fn instructions(&self) -> String {
        "ChromaDB MCP Server provides tools to work with vector embeddings, collections, and documents.".to_string()
    }

    fn capabilities(&self) -> ServerCapabilities {
        mcp_server::router::CapabilitiesBuilder::new()
            .with_tools(true)
            .build()
    }

    fn list_tools(&self) -> Vec<Tool> {
        tools::get_tool_definitions()
    }

    fn call_tool(
        &self,
        tool_name: &str,
        arguments: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Content>, ToolError>> + Send + 'static>> {
        let tool_name = tool_name.to_string();
        
        Box::pin(async move {
            let router = ChromaRouter::new(Config::parse());
            match router.dispatch_method(&tool_name, arguments).await {
                Ok(value) => {
                    let json_str = serde_json::to_string_pretty(&value)
                        .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
                    Ok(vec![Content::text(json_str)])
                }
                Err(err) => Err(ToolError::ExecutionError(err.to_string())),
            }
        })
    }

    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }

    fn read_resource(
        &self,
        _uri: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, ResourceError>> + Send + 'static>> {
        Box::pin(async { Err(ResourceError::NotFound("Resource not found".to_string())) })
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        vec![]
    }

    fn get_prompt(
        &self,
        _prompt_name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<String, PromptError>> + Send + 'static>> {
        Box::pin(async { Err(PromptError::NotFound("Prompt not found".to_string())) })
    }
}

async fn run_server(transport: ByteTransport<tokio::io::Stdin, tokio::io::Stdout>, config: Config) -> Result<()> {
    let router = ChromaRouter::new(config);
    let router_service = RouterService(router);
    let server = Server::new(router_service);
    
    tracing::info!("Starting MCP server with transport");
    server.run(transport).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .init();

    let config = Config::parse();

    if Path::new(&config.dotenv_path).exists() {
        tracing::debug!("Loading environment from {}", config.dotenv_path.display());
        dotenv::from_path(&config.dotenv_path)?;
        let config = Config::parse();
        config.validate()?;
        client::initialize_client()?;
        run_server(ByteTransport::new(stdin(), stdout()), config).await?;
    } else {
        tracing::warn!("Environment file {} not found, using defaults", config.dotenv_path.display());
        config.validate()?;
        client::initialize_client()?;
        run_server(ByteTransport::new(stdin(), stdout()), config).await?;
    }
    
    Ok(())
}
