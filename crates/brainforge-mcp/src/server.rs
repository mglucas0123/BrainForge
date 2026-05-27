use std::sync::Arc;
use brainforge_core::{
    KitPaths, MemoryFile, MemoryTarget, compress_memory, discover_transcripts_dir,
    extract_text_lines, list_sessions, read_memory, run_doctor, scan_entries,
    search_transcripts, sync_memory_to_cursor,
};
use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars,
    tool, tool_handler, tool_router,
    ServerHandler, ServiceExt,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct RoutineParams {
    /// Active caveman level (lite, full, ultra)
    pub level: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct ReadParams {
    /// Which memory file to read ('context' or 'user')
    pub which: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct WriteParams {
    /// Which memory file to write ('context' or 'user')
    pub which: String,
    /// New § entries to add or replace
    pub entries: Vec<String>,
    /// Replace the entire list of entries (default: false)
    pub replace: Option<bool>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct RecallParams {
    /// list | last | search
    pub mode: String,
    /// For search mode
    pub query: Option<String>,
    /// Max lines (last) or hits (search)
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct CompressParams {
    /// Which memory file to compress ('context' or 'user')
    pub which: String,
    /// Allow fewer entries after duplicate merge (default: false)
    pub allow_merge: Option<bool>,
}

#[derive(Clone)]
pub struct BrainForgeMcpServer {
    paths: Arc<KitPaths>,
    #[allow(dead_code)] // used by #[tool_handler] via Self::tool_router()
    tool_router: ToolRouter<Self>,
}

fn mcp_error(msg: impl Into<std::borrow::Cow<'static, str>>) -> ErrorData {
    ErrorData::new(ErrorCode::INTERNAL_ERROR, msg, None)
}

#[tool_router]
impl BrainForgeMcpServer {
    pub fn new(paths: KitPaths) -> Self {
        Self {
            paths: Arc::new(paths),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Returns the canonical BrainForge routine (Portuguese) including caveman guidelines and skill list")]
    async fn brainforge_routine(
        &self,
        Parameters(RoutineParams { level }): Parameters<RoutineParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let level_str = level.unwrap_or_else(|| "full".to_string());
        
        let core_dir = self.paths.core();
        
        let routine_text = std::fs::read_to_string(core_dir.join("BRAINFORGE.md")).unwrap_or_default();
        let caveman_text = std::fs::read_to_string(core_dir.join("caveman.md")).unwrap_or_default();
        let cavemem_text = std::fs::read_to_string(core_dir.join("cavemem.md")).unwrap_or_default();
        
        let mut response = String::new();
        response.push_str("## Canonical BrainForge Routine\n\n");
        if !routine_text.is_empty() {
            response.push_str(&routine_text);
        } else {
            response.push_str("Fallback: BrainForge ON. Default routine active.");
        }
        
        response.push_str("\n\n## Caveman Output Rules (pt-BR)\n\n");
        if !caveman_text.is_empty() {
            response.push_str(&caveman_text);
        }
        
        response.push_str("\n\n## Cavemem Memory Rules\n\n");
        if !cavemem_text.is_empty() {
            response.push_str(&cavemem_text);
        }
        
        response.push_str(&format!("\n\nActive Caveman Level: **{}**", level_str));
        
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Reads a memory file (context or user) and returns its entries and capacity statistics")]
    async fn memory_read(
        &self,
        Parameters(ReadParams { which }): Parameters<ReadParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let target = match which.to_lowercase().as_str() {
            "context" => MemoryTarget::ContextMd,
            "user" => MemoryTarget::UserMd,
            _ => return Err(mcp_error("Invalid memory target. Must be 'context' or 'user'")),
        };
        
        let (mem, stats) = match read_memory(&self.paths, target) {
            Ok(res) => res,
            Err(e) => return Err(mcp_error(format!("Failed to read memory: {e}"))),
        };
        
        let mut out = String::new();
        out.push_str(&format!("### {} ({})\n", stats.target.label(), stats.target.filename()));
        out.push_str(&format!("Capacity: {}/{} chars ({}%)\n", stats.char_used, stats.char_limit, stats.capacity_pct));
        out.push_str(&format!("Entries count: {}\n\n", stats.entry_count));
        
        out.push_str("#### Entries:\n");
        for (i, entry) in mem.entries.iter().enumerate() {
            out.push_str(&format!("{}. §{}§\n", i + 1, entry));
        }
        
        if stats.exceeds_threshold {
            out.push_str("\n⚠ **Threshold exceeded! Consolidation / compression recommended.**\n");
        }
        
        Ok(CallToolResult::success(vec![Content::text(out)]))
    }

    #[tool(description = "Writes new entries to the memory files (context or user), automatically validating anchor preservation")]
    async fn memory_write(
        &self,
        Parameters(WriteParams {
            which,
            entries,
            replace,
        }): Parameters<WriteParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let target = match which.to_lowercase().as_str() {
            "context" => MemoryTarget::ContextMd,
            "user" => MemoryTarget::UserMd,
            _ => return Err(mcp_error("Invalid memory target. Must be 'context' or 'user'")),
        };
        
        let replace_bool = replace.unwrap_or(false);
        
        let filepath = target.path(&self.paths);
        let original_text = match std::fs::read_to_string(&filepath) {
            Ok(txt) => txt,
            Err(e) => return Err(mcp_error(format!("Failed to read memory: {e}"))),
        };
        
        let original_mem = match MemoryFile::parse(&original_text, target.char_limit()) {
            Ok(mem) => mem,
            Err(e) => return Err(mcp_error(format!("Failed to parse memory: {e}"))),
        };
        
        let mut updated_mem = original_mem.clone();
        if replace_bool {
            updated_mem.entries = entries;
        } else {
            updated_mem.entries.extend(entries);
        }
        
        updated_mem.recompute_capacity();

        let secret_hits = scan_entries(&updated_mem.entries);
        if !secret_hits.is_empty() {
            let detail = secret_hits
                .iter()
                .map(|h| format!("{}: {}", h.kind, h.line_hint))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(mcp_error(format!("Secret scan blocked write: {detail}")));
        }
        
        let issues = updated_mem.validate(&original_mem, true);
        if !issues.is_empty() {
            let issues_str = issues.join("; ");
            return Err(mcp_error(format!("Validation failed: {}", issues_str)));
        }
        
        let rendered = updated_mem.render();
        if let Err(e) = std::fs::write(&filepath, &rendered) {
            return Err(mcp_error(format!("Failed to write memory: {e}")));
        }
        
        let _ = sync_memory_to_cursor(&self.paths);
        
        let response = format!(
            "Successfully written memory to {}. New capacity: {}/{} chars ({}%)",
            target.filename(),
            updated_mem.entry_chars(),
            updated_mem.char_limit,
            updated_mem.capacity_pct()
        );
        
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "Compresses the memory files using local heuristics (removing fillers and merging duplicates)")]
    async fn memory_compress(
        &self,
        Parameters(CompressParams {
            which,
            allow_merge,
        }): Parameters<CompressParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let target = match which.to_lowercase().as_str() {
            "context" => MemoryTarget::ContextMd,
            "user" => MemoryTarget::UserMd,
            _ => return Err(mcp_error("Invalid memory target. Must be 'context' or 'user'")),
        };
        
        let allow_merge_bool = allow_merge.unwrap_or(false);
        
        let result = match compress_memory(&self.paths, target, false, allow_merge_bool) {
            Ok(res) => res,
            Err(e) => return Err(mcp_error(format!("Failed to compress memory: {e}"))),
        };
        
        let mut response = format!(
            "Compression report for {}:\n",
            result.target.filename()
        );
        
        let saved = result.chars_before.saturating_sub(result.chars_after);
        response.push_str(&format!(
            "Chars: {} -> {} (Saved {})\n",
            result.chars_before, result.chars_after, saved
        ));
        response.push_str(&format!(
            "Entries: {} -> {}\n",
            result.entries_before, result.entries_after
        ));
        
        if result.written {
            response.push_str("Disk write: SUCCESS\n");
            let _ = sync_memory_to_cursor(&self.paths);
        } else {
            response.push_str("Disk write: SKIPPED (no changes or validation issue)\n");
            if let Some(reason) = result.skip_reason {
                response.push_str(&format!("Skip reason: {:?}\n", reason));
            }
        }
        
        if !result.validation_issues.is_empty() {
            response.push_str("\n⚠ Validation Issues encountered:\n");
            for issue in &result.validation_issues {
                response.push_str(&format!("- {}\n", issue));
            }
        }
        
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    #[tool(description = "List or search Cursor agent transcripts for the current project (list | last | search)")]
    async fn session_recall(
        &self,
        Parameters(RecallParams {
            mode,
            query,
            limit,
        }): Parameters<RecallParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let dir = match discover_transcripts_dir(&self.paths.project_root) {
            Ok(Some(d)) => d,
            Ok(None) => {
                return Err(mcp_error(
                    "agent-transcripts not found for this project",
                ));
            }
            Err(e) => return Err(mcp_error(format!("discover transcripts: {e}"))),
        };

        let mode_l = mode.to_lowercase();
        let mut out = String::new();

        match mode_l.as_str() {
            "list" => {
                let sessions = list_sessions(&dir)
                    .map_err(|e| mcp_error(format!("list sessions: {e}")))?;
                out.push_str(&format!("Transcripts: {}\n", dir.display()));
                for s in sessions {
                    out.push_str(&format!("- {} ({} lines)\n", s.id, s.line_count));
                }
            }
            "last" => {
                let n = limit.unwrap_or(15) as usize;
                let sessions = list_sessions(&dir)
                    .map_err(|e| mcp_error(format!("list sessions: {e}")))?;
                let Some(session) = sessions.first() else {
                    out.push_str("No sessions found.\n");
                    return Ok(CallToolResult::success(vec![Content::text(out)]));
                };
                out.push_str(&format!("Session: {}\n\n", session.id));
                let lines = extract_text_lines(&session.path, n)
                    .map_err(|e| mcp_error(format!("read transcript: {e}")))?;
                for (role, text) in lines {
                    out.push_str(&format!("### {role}\n{text}\n\n"));
                }
            }
            "search" => {
                let q = query.as_deref().unwrap_or("").trim();
                if q.is_empty() {
                    return Err(mcp_error("search mode requires query parameter"));
                }
                let lim = limit.unwrap_or(10) as usize;
                let hits = search_transcripts(&dir, q, lim)
                    .map_err(|e| mcp_error(format!("search: {e}")))?;
                if hits.is_empty() {
                    out.push_str(&format!("No hits for '{q}'.\n"));
                }
                for (id, line, preview) in &hits {
                    out.push_str(&format!("{id}:{line} {preview}\n"));
                }
            }
            _ => {
                return Err(mcp_error(
                    "mode must be list, last, or search",
                ));
            }
        }

        Ok(CallToolResult::success(vec![Content::text(out)]))
    }

    #[tool(description = "Runs the Doctor check on the kit and returns JSON format output")]
    async fn doctor(&self) -> Result<CallToolResult, ErrorData> {
        let report = match run_doctor(&self.paths) {
            Ok(rep) => rep,
            Err(e) => return Err(mcp_error(format!("Failed to run doctor: {e}"))),
        };
        
        let mut json_out = serde_json::json!({
            "has_fail": report.has_fail(),
            "checks": []
        });
        
        for c in report.checks {
            let status_str = match c.status {
                brainforge_core::DoctorStatus::Pass => "pass",
                brainforge_core::DoctorStatus::Warn => "warn",
                brainforge_core::DoctorStatus::Fail => "fail",
            };
            json_out["checks"].as_array_mut().unwrap().push(serde_json::json!({
                "name": c.name,
                "status": status_str,
                "detail": c.detail,
            }));
        }
        
        let response = serde_json::to_string_pretty(&json_out).unwrap_or_default();
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
}

#[tool_handler]
impl ServerHandler for BrainForgeMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "brainforge-mcp",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "BrainForge MCP: no início da tarefa chame brainforge_routine e memory_read(context). \
                 Memória canônica em brainforge/memory/.",
            )
    }
}

pub async fn run_server(paths: KitPaths) -> anyhow::Result<()> {
    let server = BrainForgeMcpServer::new(paths);
    let transport = rmcp::transport::stdio();
    server.serve(transport).await?;
    Ok(())
}
