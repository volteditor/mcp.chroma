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
use serde::{Deserialize, Serialize};
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
    
    async fn call_tool_method<T, R, F, Fut>(&self, args: Value, f: F) -> Result<Value, anyhow::Error> 
    where
        T: for<'de> Deserialize<'de>,
        R: Serialize,
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = Result<R>>,
    {
        let args = serde_json::from_value(args)?;
        let result = f(args).await?;
        serde_json::to_value(result).map_err(Into::into)
    }
    
    async fn dispatch_method(&self, name: &str, args: Value) -> Result<Value, anyhow::Error> {
        match name {
            "chroma_list_collections" => {
                self.call_tool_method(args, tools::chroma_list_collections).await
            }
            "chroma_create_collection" => {
                self.call_tool_method(args, tools::chroma_create_collection).await
            }
            "chroma_peek_collection" => {
                self.call_tool_method(args, tools::chroma_peek_collection).await
            }
            "chroma_get_collection_info" => {
                self.call_tool_method(args, tools::chroma_get_collection_info).await
            }
            "chroma_get_collection_count" => {
                self.call_tool_method(args, tools::chroma_get_collection_count).await
            }
            "chroma_modify_collection" => {
                self.call_tool_method(args, tools::chroma_modify_collection).await
            }
            "chroma_delete_collection" => {
                self.call_tool_method(args, tools::chroma_delete_collection).await
            }
            "chroma_add_documents" => {
                self.call_tool_method(args, tools::chroma_add_documents).await
            }
            "chroma_query_documents" => {
                self.call_tool_method(args, tools::chroma_query_documents).await
            }
            "chroma_get_documents" => {
                self.call_tool_method(args, tools::chroma_get_documents).await
            }
            "chroma_update_documents" => {
                self.call_tool_method(args, tools::chroma_update_documents).await
            }
            "chroma_delete_documents" => {
                self.call_tool_method(args, tools::chroma_delete_documents).await
            }
            "process_thought" => {
                self.call_tool_method(args, tools::process_thought).await
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

    let mut config = Config::parse();

    if Path::new(&config.dotenv_path).exists() {
        tracing::debug!("Loading environment from {}", config.dotenv_path.display());
        dotenv::from_path(&config.dotenv_path)?;
        config = Config::parse();
    } else {
        tracing::warn!("Environment file {} not found, using defaults", config.dotenv_path.display());
    }
    
    config.validate()?;
    client::initialize_client()?;
    run_server(ByteTransport::new(stdin(), stdout()), config).await
}
