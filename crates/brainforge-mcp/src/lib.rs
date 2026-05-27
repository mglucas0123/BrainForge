mod server;

use brainforge_core::KitPaths;

pub async fn run_mcp_server(paths: KitPaths) -> anyhow::Result<()> {
    server::run_server(paths).await
}

/// Blocking entry for the CLI (`brainforge mcp`).
pub fn run_mcp_server_blocking(paths: KitPaths) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(run_mcp_server(paths))
}
