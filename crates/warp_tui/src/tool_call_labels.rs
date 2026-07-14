//! Per-tool, per-state one-line labels for tool-call rows in the TUI
//! transcript, modeled on the GUI's inline action text.

use std::path::Path;

use rust_i18n::t;
use warp::tui_export::{
    AIActionStatus, AIAgentAction, AIAgentActionResultType, AIAgentActionType,
    AskUserQuestionResult, FileGlobV2Result, GrepResult, RequestCommandOutputResult,
    RunAgentsAgentOutcomeKind, RunAgentsResult, SearchCodebaseFailureReason, SearchCodebaseResult,
    StartAgentExecutionMode, SuggestNewConversationResult,
};
use warp_core::command::ExitCode;

use self::ToolCallDisplayState as State;

/// Ground-truth state of the terminal block backing a shell-command tool
/// call, resolved by the caller. When a block exists, its state supersedes
/// the stored action status/result for execution states (mirroring the GUI's
/// `RequestedCommandView`, which derives icon and expandability from the
/// block whenever one exists). Notably, an agent-monitored command's stored
/// result stays a `LongRunningCommandSnapshot` forever, so without the block
/// its row could never leave the "still running" state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CommandBlockState {
    Running,
    Finished { exit_code: ExitCode },
}

/// A shell-command tool call's terminal block as resolved by the caller: its
/// execution state plus the command it actually ran. The block's command
/// supersedes the streamed one, which the user may have edited before
/// accepting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedCommandBlock {
    /// The block's command, when it has one; `None` while the block's
    /// command grid is still empty.
    pub(crate) command: Option<String>,
    pub(crate) state: CommandBlockState,
}

/// Longest rendered length for interpolated values (commands, queries, paths)
/// so tool-call rows stay scannable one-liners.
const MAX_INLINE_LEN: usize = 80;

/// The coarse display state of a tool call, derived from its action status.
///
/// TUI-local presentation collapse of the shared [`AIActionStatus`]; the GUI
/// has no equivalent enum — its per-tool views consume `AIActionStatus`
/// directly and re-derive per-site booleans (queued/cancelled/streaming).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ToolCallDisplayState {
    /// The tool call's arguments are still streaming in: it has no action
    /// status yet and the exchange output is still streaming, so argument
    /// fields may be empty or partial and must not be interpolated.
    Constructing,
    /// No status yet (stream finished), preprocessing, or queued behind
    /// other actions.
    Pending,
    /// Blocked on user confirmation.
    AwaitingApproval,
    /// Executing asynchronously.
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

/// Collapses an optional action status into the coarse display state.
/// `output_streaming` is whether the exchange output is still streaming;
/// a status-less action in a streaming output is still being constructed
/// (mirroring the GUI's `status.is_none() && is_streaming()` gating).
/// A resolved `block_state` supersedes the status for execution states
/// (see [`CommandBlockState`]).
pub(crate) fn tool_call_display_state(
    status: Option<&AIActionStatus>,
    output_streaming: bool,
    block_state: Option<CommandBlockState>,
) -> ToolCallDisplayState {
    // A block existing means the command actually started executing, so its
    // state is authoritative over the action status/result.
    match block_state {
        Some(CommandBlockState::Running) => return State::Running,
        Some(CommandBlockState::Finished { exit_code }) => {
            return if exit_code.is_sigint() {
                State::Cancelled
            } else if exit_code.was_successful() {
                State::Succeeded
            } else {
                State::Failed
            };
        }
        None => {}
    }
    match status {
        None if output_streaming => State::Constructing,
        None | Some(AIActionStatus::Preprocessing | AIActionStatus::Queued) => State::Pending,
        Some(AIActionStatus::Blocked) => State::AwaitingApproval,
        Some(AIActionStatus::RunningAsync) => State::Running,
        Some(finished @ AIActionStatus::Finished(_)) => {
            if finished.is_cancelled() {
                State::Cancelled
            } else if finished.is_failed() {
                State::Failed
            } else {
                State::Succeeded
            }
        }
    }
}

/// The leading status glyph for a tool-call row; the caller colors it to
/// mirror the GUI's inline action icons (`action_icon` in the GUI's
/// `output.rs`): grey circle while pending, yellow block awaiting approval,
/// yellow dot running, green check on success, red x on failure, grey block
/// on cancellation.
pub(crate) fn tool_call_glyph(state: ToolCallDisplayState) -> &'static str {
    match state {
        State::Constructing | State::Pending => "○",
        State::AwaitingApproval | State::Cancelled => "■",
        State::Running => "●",
        State::Succeeded => "✓",
        State::Failed => "✗",
    }
}

/// Returns the one-line transcript label for a tool call in its current state.
pub(crate) fn tool_call_label(
    action: &AIAgentAction,
    status: Option<&AIActionStatus>,
    output_streaming: bool,
    block: Option<&ResolvedCommandBlock>,
) -> String {
    let state = tool_call_display_state(status, output_streaming, block.map(|block| block.state));
    let result = status
        .and_then(AIActionStatus::finished_result)
        .map(|result| &result.result);
    let label = label_for_action(&action.action, state, result, block);
    match state {
        State::AwaitingApproval => {
            t!("warp_tui.tool_calls.awaiting_approval", label = label).to_string()
        }
        State::Constructing
        | State::Pending
        | State::Running
        | State::Succeeded
        | State::Failed
        | State::Cancelled => label,
    }
}

/// Builds the per-tool label body; the awaiting-approval suffix is applied by
/// [`tool_call_label`]. `result` is the finished result, when there is one.
///
/// `Constructing` arms never interpolate argument fields (they may be empty
/// or partial while streaming); their copy is indexed on the GUI's loading
/// messages (`common.rs` `LOAD_OUTPUT_MESSAGE_*` and the requested-command
/// view's "Generating command...").
fn label_for_action(
    action: &AIAgentActionType,
    state: ToolCallDisplayState,
    result: Option<&AIAgentActionResultType>,
    block: Option<&ResolvedCommandBlock>,
) -> String {
    let block_state = block.map(|block| block.state);
    match action {
        AIAgentActionType::RequestCommandOutput { command, .. } => {
            // The streamed command can be edited before acceptance, so
            // prefer the executed command from the finished result or the
            // resolved block over the original suggestion.
            let executed = result
                .and_then(AIAgentActionResultType::command_str)
                .or_else(|| block.and_then(|block| block.command.as_deref()));
            let cmd = single_line(executed.unwrap_or(command));
            match state {
                State::Constructing => t!("warp_tui.tool_calls.command.constructing").to_string(),
                State::Pending | State::AwaitingApproval => {
                    t!("warp_tui.tool_calls.command.pending", command = cmd).to_string()
                }
                State::Running => {
                    t!("warp_tui.tool_calls.command.running", command = cmd).to_string()
                }
                State::Succeeded => match block_state {
                    Some(CommandBlockState::Finished { .. }) => {
                        t!("warp_tui.tool_calls.command.succeeded", command = cmd).to_string()
                    }
                    // No local block: fall back to the stored result. A
                    // snapshot result means the command was still running at
                    // the last point we could observe it.
                    Some(CommandBlockState::Running) | None => match result {
                        Some(AIAgentActionResultType::RequestCommandOutput(
                            RequestCommandOutputResult::LongRunningCommandSnapshot { .. },
                        )) => t!("warp_tui.tool_calls.command.still_running", command = cmd)
                            .to_string(),
                        _ => t!("warp_tui.tool_calls.command.succeeded", command = cmd).to_string(),
                    },
                },
                State::Failed => match block_state {
                    Some(CommandBlockState::Finished { exit_code }) => t!(
                        "warp_tui.tool_calls.command.exited_with_code",
                        command = cmd,
                        code = exit_code.value()
                    )
                    .to_string(),
                    Some(CommandBlockState::Running) | None => match result {
                        Some(AIAgentActionResultType::RequestCommandOutput(
                            RequestCommandOutputResult::Completed { exit_code, .. },
                        )) => t!(
                            "warp_tui.tool_calls.command.exited_with_code",
                            command = cmd,
                            code = exit_code.value()
                        )
                        .to_string(),
                        Some(AIAgentActionResultType::RequestCommandOutput(
                            RequestCommandOutputResult::Denylisted { .. },
                        )) => {
                            t!("warp_tui.tool_calls.command.denylisted", command = cmd).to_string()
                        }
                        _ => t!("warp_tui.tool_calls.command.failed", command = cmd).to_string(),
                    },
                },
                State::Cancelled => {
                    t!("warp_tui.tool_calls.command.cancelled", command = cmd).to_string()
                }
            }
        }
        AIAgentActionType::WriteToLongRunningShellCommand { .. } => match state {
            State::Constructing => {
                t!("warp_tui.tool_calls.write_command_input.constructing").to_string()
            }
            State::Pending | State::AwaitingApproval => {
                t!("warp_tui.tool_calls.write_command_input.pending").to_string()
            }
            State::Running => t!("warp_tui.tool_calls.write_command_input.running").to_string(),
            State::Succeeded => t!("warp_tui.tool_calls.write_command_input.succeeded").to_string(),
            State::Failed => t!("warp_tui.tool_calls.write_command_input.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.write_command_input.cancelled").to_string(),
        },
        AIAgentActionType::ReadFiles(request) => {
            let files = files_summary(request.locations.iter().map(|location| &location.name));
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.read_files.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval | State::Succeeded => {
                    t!("warp_tui.tool_calls.read_files.complete", files = files).to_string()
                }
                State::Running => {
                    t!("warp_tui.tool_calls.read_files.running", files = files).to_string()
                }
                State::Failed => {
                    t!("warp_tui.tool_calls.read_files.failed", files = files).to_string()
                }
                State::Cancelled => {
                    t!("warp_tui.tool_calls.read_files.cancelled", files = files).to_string()
                }
            }
        }
        AIAgentActionType::UploadArtifact(request) => {
            let file = single_line(&request.file_path);
            match state {
                State::Constructing => t!("warp_tui.tool_calls.upload.constructing").to_string(),
                State::Pending | State::AwaitingApproval => {
                    t!("warp_tui.tool_calls.upload.pending", file = file).to_string()
                }
                State::Running => t!("warp_tui.tool_calls.upload.running", file = file).to_string(),
                State::Succeeded => {
                    t!("warp_tui.tool_calls.upload.succeeded", file = file).to_string()
                }
                State::Failed => t!("warp_tui.tool_calls.upload.failed", file = file).to_string(),
                State::Cancelled => {
                    t!("warp_tui.tool_calls.upload.cancelled", file = file).to_string()
                }
            }
        }
        AIAgentActionType::SearchCodebase(request) => {
            let query = single_line(&request.query);
            let scope = request
                .codebase_path
                .as_deref()
                .map(|path| {
                    t!(
                        "warp_tui.tool_calls.search_codebase.scope",
                        path = base_name(path)
                    )
                    .to_string()
                })
                .unwrap_or_default();
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.search_codebase.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval => t!(
                    "warp_tui.tool_calls.search_codebase.pending",
                    query = query,
                    scope = scope
                )
                .to_string(),
                State::Running => t!(
                    "warp_tui.tool_calls.search_codebase.running",
                    query = query,
                    scope = scope
                )
                .to_string(),
                State::Succeeded => match result {
                    Some(AIAgentActionResultType::SearchCodebase(
                        SearchCodebaseResult::Success { files },
                    )) if files.is_empty() => t!(
                        "warp_tui.tool_calls.search_codebase.succeeded_no_results",
                        query = query,
                        scope = scope
                    )
                    .to_string(),
                    Some(AIAgentActionResultType::SearchCodebase(
                        SearchCodebaseResult::Success { files },
                    )) => t!(
                        "warp_tui.tool_calls.search_codebase.succeeded_with_results",
                        query = query,
                        scope = scope,
                        results = count_label(files.len(), CountedNoun::Result)
                    )
                    .to_string(),
                    _ => t!(
                        "warp_tui.tool_calls.search_codebase.succeeded",
                        query = query,
                        scope = scope
                    )
                    .to_string(),
                },
                State::Failed => match result {
                    Some(AIAgentActionResultType::SearchCodebase(
                        SearchCodebaseResult::Failed {
                            reason: SearchCodebaseFailureReason::CodebaseNotIndexed,
                            ..
                        },
                    )) => t!(
                        "warp_tui.tool_calls.search_codebase.not_indexed",
                        query = query,
                        scope = scope
                    )
                    .to_string(),
                    _ => t!(
                        "warp_tui.tool_calls.search_codebase.failed",
                        query = query,
                        scope = scope
                    )
                    .to_string(),
                },
                State::Cancelled => t!(
                    "warp_tui.tool_calls.search_codebase.cancelled",
                    query = query,
                    scope = scope
                )
                .to_string(),
            }
        }
        // Rendered by its own stateful child view (`TuiFileEditsView`); the
        // label path should never be reached for it.
        AIAgentActionType::RequestFileEdits { .. } => {
            log::warn!("tool_call_label called for RequestFileEdits, which has custom rendering");
            String::new()
        }
        AIAgentActionType::Grep { queries, path } => {
            let queries = single_line(&queries.join(", "));
            let path = display_path(path);
            match state {
                State::Constructing => t!("warp_tui.tool_calls.grep.constructing").to_string(),
                State::Pending | State::AwaitingApproval => t!(
                    "warp_tui.tool_calls.grep.pending",
                    queries = queries,
                    path = path
                )
                .to_string(),
                State::Running => t!(
                    "warp_tui.tool_calls.grep.running",
                    queries = queries,
                    path = path
                )
                .to_string(),
                State::Succeeded => match result {
                    Some(AIAgentActionResultType::Grep(GrepResult::Success { matched_files })) => {
                        t!(
                            "warp_tui.tool_calls.grep.succeeded_with_matches",
                            queries = queries,
                            path = path,
                            matches = count_label(matched_files.len(), CountedNoun::MatchingFile)
                        )
                        .to_string()
                    }
                    _ => t!(
                        "warp_tui.tool_calls.grep.succeeded",
                        queries = queries,
                        path = path
                    )
                    .to_string(),
                },
                State::Failed => {
                    t!("warp_tui.tool_calls.grep.failed", queries = queries).to_string()
                }
                State::Cancelled => {
                    t!("warp_tui.tool_calls.grep.cancelled", queries = queries).to_string()
                }
            }
        }
        AIAgentActionType::FileGlob { patterns, path } => {
            file_glob_label(patterns, path.as_deref(), state, None)
        }
        AIAgentActionType::FileGlobV2 {
            patterns,
            search_dir,
        } => {
            let matched_count = match result {
                Some(AIAgentActionResultType::FileGlobV2(FileGlobV2Result::Success {
                    matched_files,
                    ..
                })) => Some(matched_files.len()),
                _ => None,
            };
            file_glob_label(patterns, search_dir.as_deref(), state, matched_count)
        }
        AIAgentActionType::ReadMCPResource { name, uri, .. } => {
            let resource = single_line(uri.as_deref().unwrap_or(name));
            match state {
                // The resource name arrives with the tool-call header (not
                // the streamed args), so include it when present, like the
                // GUI's "Reading \"{name}\" MCP resource..." loading text.
                State::Constructing if name.is_empty() => {
                    t!("warp_tui.tool_calls.mcp_resource.constructing_unnamed").to_string()
                }
                State::Constructing => t!(
                    "warp_tui.tool_calls.mcp_resource.constructing_named",
                    name = name
                )
                .to_string(),
                State::Pending | State::AwaitingApproval | State::Succeeded => t!(
                    "warp_tui.tool_calls.mcp_resource.complete",
                    resource = resource
                )
                .to_string(),
                State::Running => t!(
                    "warp_tui.tool_calls.mcp_resource.running",
                    resource = resource
                )
                .to_string(),
                State::Failed => t!(
                    "warp_tui.tool_calls.mcp_resource.failed",
                    resource = resource
                )
                .to_string(),
                State::Cancelled => t!(
                    "warp_tui.tool_calls.mcp_resource.cancelled",
                    resource = resource
                )
                .to_string(),
            }
        }
        AIAgentActionType::CallMCPTool { name, .. } => {
            let name = single_line(name);
            match state {
                // Like the GUI's "Calling \"{name}\" MCP tool..." loading
                // text; the tool name is available before its args finish.
                State::Constructing if name.is_empty() => {
                    t!("warp_tui.tool_calls.mcp_tool.constructing_unnamed").to_string()
                }
                State::Constructing => t!(
                    "warp_tui.tool_calls.mcp_tool.constructing_named",
                    name = name
                )
                .to_string(),
                State::Pending | State::AwaitingApproval => {
                    t!("warp_tui.tool_calls.mcp_tool.pending", name = name).to_string()
                }
                State::Running => {
                    t!("warp_tui.tool_calls.mcp_tool.running", name = name).to_string()
                }
                State::Succeeded => {
                    t!("warp_tui.tool_calls.mcp_tool.succeeded", name = name).to_string()
                }
                State::Failed => t!("warp_tui.tool_calls.mcp_tool.failed", name = name).to_string(),
                State::Cancelled => {
                    t!("warp_tui.tool_calls.mcp_tool.cancelled", name = name).to_string()
                }
            }
        }
        AIAgentActionType::SuggestNewConversation { .. } => match state {
            State::Constructing => {
                t!("warp_tui.tool_calls.suggest_new_conversation.constructing").to_string()
            }
            State::Pending | State::AwaitingApproval | State::Running | State::Failed => {
                t!("warp_tui.tool_calls.suggest_new_conversation.suggested").to_string()
            }
            State::Succeeded => match result {
                Some(AIAgentActionResultType::SuggestNewConversation(
                    SuggestNewConversationResult::Rejected,
                )) => t!("warp_tui.tool_calls.suggest_new_conversation.continuing").to_string(),
                _ => t!("warp_tui.tool_calls.suggest_new_conversation.started").to_string(),
            },
            State::Cancelled => {
                t!("warp_tui.tool_calls.suggest_new_conversation.cancelled").to_string()
            }
        },
        AIAgentActionType::SuggestPrompt(_) => fallback_label(
            t!("warp_tui.tool_calls.fallback.suggest_prompt").to_string(),
            state,
        ),
        AIAgentActionType::InitProject => fallback_label(
            t!("warp_tui.tool_calls.fallback.init_project").to_string(),
            state,
        ),
        AIAgentActionType::OpenCodeReview => fallback_label(
            t!("warp_tui.tool_calls.fallback.open_code_review").to_string(),
            state,
        ),
        AIAgentActionType::ReadDocuments(request) => {
            let documents = count_label(request.document_ids.len(), CountedNoun::Document);
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.read_documents.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval | State::Succeeded => t!(
                    "warp_tui.tool_calls.read_documents.complete",
                    documents = documents
                )
                .to_string(),
                State::Running => t!(
                    "warp_tui.tool_calls.read_documents.running",
                    documents = documents
                )
                .to_string(),
                State::Failed => t!("warp_tui.tool_calls.read_documents.failed").to_string(),
                State::Cancelled => t!("warp_tui.tool_calls.read_documents.cancelled").to_string(),
            }
        }
        AIAgentActionType::EditDocuments(request) => match state {
            State::Pending | State::AwaitingApproval => {
                t!("warp_tui.tool_calls.edit_documents.pending").to_string()
            }
            State::Constructing | State::Running => {
                t!("warp_tui.tool_calls.edit_documents.running").to_string()
            }
            State::Succeeded => t!(
                "warp_tui.tool_calls.edit_documents.succeeded",
                edits = count_label(request.diffs.len(), CountedNoun::Edit)
            )
            .to_string(),
            State::Failed => t!("warp_tui.tool_calls.edit_documents.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.edit_documents.cancelled").to_string(),
        },
        AIAgentActionType::CreateDocuments(request) => match state {
            State::Pending | State::AwaitingApproval => {
                t!("warp_tui.tool_calls.create_documents.pending").to_string()
            }
            State::Constructing | State::Running => {
                t!("warp_tui.tool_calls.create_documents.running").to_string()
            }
            State::Succeeded => {
                let count = request.documents.len();
                if count > 1 {
                    t!(
                        "warp_tui.tool_calls.create_documents.succeeded_many",
                        count = count
                    )
                    .to_string()
                } else {
                    t!("warp_tui.tool_calls.create_documents.succeeded_plan").to_string()
                }
            }
            State::Failed => t!("warp_tui.tool_calls.create_documents.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.create_documents.cancelled").to_string(),
        },
        AIAgentActionType::ReadShellCommandOutput { .. } => match state {
            State::Pending | State::AwaitingApproval | State::Succeeded => {
                t!("warp_tui.tool_calls.read_command_output.complete").to_string()
            }
            State::Constructing | State::Running => {
                t!("warp_tui.tool_calls.read_command_output.running").to_string()
            }
            State::Failed => t!("warp_tui.tool_calls.read_command_output.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.read_command_output.cancelled").to_string(),
        },
        AIAgentActionType::UseComputer(request) => summary_label(&request.action_summary, state),
        AIAgentActionType::InsertCodeReviewComments { comments, .. } => {
            let comments = count_label(comments.len(), CountedNoun::ReviewComment);
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.review_comments.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval => t!(
                    "warp_tui.tool_calls.review_comments.pending",
                    comments = comments
                )
                .to_string(),
                State::Running => t!(
                    "warp_tui.tool_calls.review_comments.running",
                    comments = comments
                )
                .to_string(),
                State::Succeeded => t!(
                    "warp_tui.tool_calls.review_comments.succeeded",
                    comments = comments
                )
                .to_string(),
                State::Failed => t!("warp_tui.tool_calls.review_comments.failed").to_string(),
                State::Cancelled => t!("warp_tui.tool_calls.review_comments.cancelled").to_string(),
            }
        }
        AIAgentActionType::RequestComputerUse(request) => {
            summary_label(&request.task_summary, state)
        }
        AIAgentActionType::StartRecording { .. } => match state {
            State::Pending | State::AwaitingApproval => {
                t!("warp_tui.tool_calls.start_recording.pending").to_string()
            }
            State::Constructing | State::Running => {
                t!("warp_tui.tool_calls.start_recording.running").to_string()
            }
            State::Succeeded => t!("warp_tui.tool_calls.start_recording.succeeded").to_string(),
            State::Failed => t!("warp_tui.tool_calls.start_recording.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.start_recording.cancelled").to_string(),
        },
        AIAgentActionType::StopRecording { .. } => match state {
            State::Pending | State::AwaitingApproval => {
                t!("warp_tui.tool_calls.stop_recording.pending").to_string()
            }
            State::Constructing | State::Running => {
                t!("warp_tui.tool_calls.stop_recording.running").to_string()
            }
            State::Succeeded => t!("warp_tui.tool_calls.stop_recording.succeeded").to_string(),
            State::Failed => t!("warp_tui.tool_calls.stop_recording.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.stop_recording.cancelled").to_string(),
        },
        AIAgentActionType::ReadSkill(request) => {
            let skill = single_line(&request.skill.to_string());
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.read_skill.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval | State::Succeeded => {
                    t!("warp_tui.tool_calls.read_skill.complete", skill = skill).to_string()
                }
                State::Running => {
                    t!("warp_tui.tool_calls.read_skill.running", skill = skill).to_string()
                }
                State::Failed => {
                    t!("warp_tui.tool_calls.read_skill.failed", skill = skill).to_string()
                }
                State::Cancelled => {
                    t!("warp_tui.tool_calls.read_skill.cancelled", skill = skill).to_string()
                }
            }
        }
        AIAgentActionType::FetchConversation { .. } => match state {
            State::Pending | State::AwaitingApproval => {
                t!("warp_tui.tool_calls.fetch_conversation.pending").to_string()
            }
            State::Constructing | State::Running => {
                t!("warp_tui.tool_calls.fetch_conversation.running").to_string()
            }
            State::Succeeded => t!("warp_tui.tool_calls.fetch_conversation.succeeded").to_string(),
            State::Failed => t!("warp_tui.tool_calls.fetch_conversation.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.fetch_conversation.cancelled").to_string(),
        },
        AIAgentActionType::StartAgent {
            name,
            execution_mode,
            ..
        } => {
            let agent = if matches!(execution_mode, StartAgentExecutionMode::Remote { .. }) {
                t!("warp_tui.tool_calls.start_agent.remote_agent", name = name).to_string()
            } else {
                t!("warp_tui.tool_calls.start_agent.local_agent", name = name).to_string()
            };
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.start_agent.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval => {
                    t!("warp_tui.tool_calls.start_agent.pending", agent = agent).to_string()
                }
                State::Running => {
                    t!("warp_tui.tool_calls.start_agent.running", agent = agent).to_string()
                }
                State::Succeeded => {
                    t!("warp_tui.tool_calls.start_agent.succeeded", name = name).to_string()
                }
                State::Failed => {
                    t!("warp_tui.tool_calls.start_agent.failed", name = name).to_string()
                }
                State::Cancelled => {
                    t!("warp_tui.tool_calls.start_agent.cancelled", name = name).to_string()
                }
            }
        }
        AIAgentActionType::SendMessageToAgent {
            addresses, subject, ..
        } => {
            let subject = single_line(subject);
            match state {
                State::Constructing => {
                    t!("warp_tui.tool_calls.send_agent_message.constructing").to_string()
                }
                State::Pending | State::AwaitingApproval => t!(
                    "warp_tui.tool_calls.send_agent_message.pending",
                    subject = subject
                )
                .to_string(),
                State::Running => t!(
                    "warp_tui.tool_calls.send_agent_message.running",
                    agents = count_label(addresses.len(), CountedNoun::Agent),
                    subject = subject
                )
                .to_string(),
                State::Succeeded => t!(
                    "warp_tui.tool_calls.send_agent_message.succeeded",
                    subject = subject
                )
                .to_string(),
                State::Failed => t!(
                    "warp_tui.tool_calls.send_agent_message.failed",
                    subject = subject
                )
                .to_string(),
                State::Cancelled => {
                    t!("warp_tui.tool_calls.send_agent_message.cancelled").to_string()
                }
            }
        }
        AIAgentActionType::TransferShellCommandControlToUser { reason } => match state {
            State::Constructing => {
                t!("warp_tui.tool_calls.transfer_control.constructing").to_string()
            }
            State::Pending | State::AwaitingApproval | State::Running => t!(
                "warp_tui.tool_calls.transfer_control.running",
                reason = single_line(reason)
            )
            .to_string(),
            State::Succeeded => t!("warp_tui.tool_calls.transfer_control.succeeded").to_string(),
            State::Failed => t!("warp_tui.tool_calls.transfer_control.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.transfer_control.cancelled").to_string(),
        },
        AIAgentActionType::AskUserQuestion { questions } => match state {
            State::Constructing => t!("warp_tui.tool_calls.ask_user.constructing").to_string(),
            State::Pending | State::AwaitingApproval | State::Running => t!(
                "warp_tui.tool_calls.ask_user.asking",
                questions = count_label(questions.len(), CountedNoun::Question)
            )
            .to_string(),
            State::Succeeded => match result {
                Some(AIAgentActionResultType::AskUserQuestion(
                    AskUserQuestionResult::Success { answers },
                )) => {
                    let total = answers.len();
                    let answered = answers.iter().filter(|answer| !answer.is_skipped()).count();
                    if answered == 0 {
                        t!("warp_tui.tool_calls.ask_user.skipped").to_string()
                    } else if answered == total && total == 1 {
                        t!("warp_tui.tool_calls.ask_user.answered_one").to_string()
                    } else if answered == total {
                        t!("warp_tui.tool_calls.ask_user.answered_all", total = total).to_string()
                    } else {
                        t!(
                            "warp_tui.tool_calls.ask_user.answered_partial",
                            answered = answered,
                            total = total
                        )
                        .to_string()
                    }
                }
                Some(AIAgentActionResultType::AskUserQuestion(
                    AskUserQuestionResult::SkippedByAutoApprove { .. },
                )) => t!("warp_tui.tool_calls.ask_user.skipped").to_string(),
                _ => t!("warp_tui.tool_calls.ask_user.answered").to_string(),
            },
            State::Failed => t!("warp_tui.tool_calls.ask_user.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.ask_user.cancelled").to_string(),
        },
        AIAgentActionType::RunAgents(request) => {
            let total = request.agent_run_configs.len();
            match state {
                State::Constructing | State::Pending | State::AwaitingApproval => {
                    t!("warp_tui.tool_calls.run_agents.configuring").to_string()
                }
                State::Running => t!(
                    "warp_tui.tool_calls.run_agents.spawning",
                    agents = count_label(total, CountedNoun::Agent)
                )
                .to_string(),
                State::Succeeded => match result {
                    Some(AIAgentActionResultType::RunAgents(RunAgentsResult::Launched {
                        agents,
                        ..
                    })) => {
                        let launched = agents
                            .iter()
                            .filter(|agent| {
                                matches!(agent.kind, RunAgentsAgentOutcomeKind::Launched { .. })
                            })
                            .count();
                        let total = agents.len();
                        if launched == total {
                            t!(
                                "warp_tui.tool_calls.run_agents.spawned",
                                agents = count_label(total, CountedNoun::Agent)
                            )
                            .to_string()
                        } else if launched == 0 {
                            t!(
                                "warp_tui.tool_calls.run_agents.failed_to_spawn",
                                agents = count_label(total, CountedNoun::Agent)
                            )
                            .to_string()
                        } else {
                            t!(
                                "warp_tui.tool_calls.run_agents.spawned_partial",
                                launched = launched,
                                total = total
                            )
                            .to_string()
                        }
                    }
                    _ => t!(
                        "warp_tui.tool_calls.run_agents.spawned",
                        agents = count_label(total, CountedNoun::Agent)
                    )
                    .to_string(),
                },
                State::Failed => match result {
                    Some(AIAgentActionResultType::RunAgents(RunAgentsResult::Denied {
                        ..
                    })) => t!("warp_tui.tool_calls.run_agents.orchestration_disabled").to_string(),
                    Some(AIAgentActionResultType::RunAgents(RunAgentsResult::Failure {
                        error,
                    })) if !error.is_empty() => t!(
                        "warp_tui.tool_calls.run_agents.orchestration_failed_with_error",
                        error = single_line(error)
                    )
                    .to_string(),
                    _ => t!("warp_tui.tool_calls.run_agents.orchestration_failed").to_string(),
                },
                State::Cancelled => t!("warp_tui.tool_calls.run_agents.cancelled").to_string(),
            }
        }
        AIAgentActionType::WaitForEvents { .. } => match state {
            State::Constructing | State::Pending | State::AwaitingApproval | State::Running => {
                t!("warp_tui.tool_calls.wait_for_events.waiting").to_string()
            }
            State::Succeeded => t!("warp_tui.tool_calls.wait_for_events.succeeded").to_string(),
            State::Failed => t!("warp_tui.tool_calls.wait_for_events.failed").to_string(),
            State::Cancelled => t!("warp_tui.tool_calls.wait_for_events.cancelled").to_string(),
        },
    }
}

/// Shared label body for both file-glob action versions; only V2 results
/// carry a match count.
fn file_glob_label(
    patterns: &[String],
    path: Option<&str>,
    state: ToolCallDisplayState,
    matched_count: Option<usize>,
) -> String {
    let patterns = single_line(&patterns.join(", "));
    let path = display_path(path.unwrap_or("."));
    match state {
        State::Constructing => t!("warp_tui.tool_calls.file_glob.constructing").to_string(),
        State::Pending | State::AwaitingApproval => t!(
            "warp_tui.tool_calls.file_glob.pending",
            patterns = patterns,
            path = path
        )
        .to_string(),
        State::Running => t!(
            "warp_tui.tool_calls.file_glob.running",
            patterns = patterns,
            path = path
        )
        .to_string(),
        State::Succeeded => match matched_count {
            Some(count) => t!(
                "warp_tui.tool_calls.file_glob.succeeded_with_count",
                files = count_label(count, CountedNoun::File),
                patterns = patterns
            )
            .to_string(),
            None => t!(
                "warp_tui.tool_calls.file_glob.succeeded",
                patterns = patterns
            )
            .to_string(),
        },
        State::Failed => {
            t!("warp_tui.tool_calls.file_glob.failed", patterns = patterns).to_string()
        }
        State::Cancelled => t!(
            "warp_tui.tool_calls.file_glob.cancelled",
            patterns = patterns
        )
        .to_string(),
    }
}

/// Labels computer-use calls with their agent-supplied summary, marking only
/// terminal non-success states (matching the GUI, which shows the summary
/// verbatim).
fn summary_label(summary: &str, state: ToolCallDisplayState) -> String {
    let summary = single_line(summary);
    match state {
        State::Constructing => t!("warp_tui.tool_calls.computer_use.constructing").to_string(),
        State::Pending | State::AwaitingApproval | State::Running | State::Succeeded => summary,
        State::Failed => {
            t!("warp_tui.tool_calls.computer_use.failed", summary = summary).to_string()
        }
        State::Cancelled => t!(
            "warp_tui.tool_calls.computer_use.cancelled",
            summary = summary
        )
        .to_string(),
    }
}

/// Generic label for action types without bespoke text, derived from the
/// action's user-friendly name.
fn fallback_label(name: String, state: ToolCallDisplayState) -> String {
    match state {
        State::Pending | State::AwaitingApproval => name,
        State::Constructing | State::Running => {
            t!("warp_tui.tool_calls.fallback.running", name = name).to_string()
        }
        State::Succeeded => t!("warp_tui.tool_calls.fallback.succeeded", name = name).to_string(),
        State::Failed => t!("warp_tui.tool_calls.fallback.failed", name = name).to_string(),
        State::Cancelled => t!("warp_tui.tool_calls.fallback.cancelled", name = name).to_string(),
    }
}

/// Collapses text to its first line, capped at [`MAX_INLINE_LEN`] chars, with
/// a trailing `…` when anything was trimmed.
fn single_line(text: &str) -> String {
    let first_line = text.lines().next().unwrap_or_default().trim_end();
    let mut out: String = first_line.chars().take(MAX_INLINE_LEN).collect();
    if first_line.chars().count() > MAX_INLINE_LEN || text.lines().count() > 1 {
        out.push('…');
    }
    out
}

/// Renders a search path for display, mirroring the GUI's treatment of `.`.
fn display_path(path: &str) -> String {
    if path == "." {
        t!("warp_tui.tool_calls.current_directory").to_string()
    } else {
        single_line(path)
    }
}

/// Returns the final path component, falling back to the input when there is none.
fn base_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_owned())
}

/// Summarizes file paths as comma-joined base names for up to 3 files, else a count.
fn files_summary<'a>(paths: impl ExactSizeIterator<Item = &'a String>) -> String {
    if paths.len() > 3 {
        return count_label(paths.len(), CountedNoun::File);
    }
    let names: Vec<String> = paths.map(|path| base_name(path)).collect();
    if names.is_empty() {
        t!("warp_tui.tool_calls.files_generic").to_string()
    } else {
        names.join(", ")
    }
}

#[derive(Clone, Copy)]
enum CountedNoun {
    File,
    Result,
    MatchingFile,
    Document,
    Edit,
    ReviewComment,
    Agent,
    Question,
}

/// Formats a localized counted noun, choosing the English singular key when needed.
fn count_label(count: usize, noun: CountedNoun) -> String {
    let singular = count == 1;
    let key = match noun {
        CountedNoun::File if singular => "warp_tui.tool_calls.counts.file_one",
        CountedNoun::File => "warp_tui.tool_calls.counts.file_other",
        CountedNoun::Result if singular => "warp_tui.tool_calls.counts.result_one",
        CountedNoun::Result => "warp_tui.tool_calls.counts.result_other",
        CountedNoun::MatchingFile if singular => "warp_tui.tool_calls.counts.matching_file_one",
        CountedNoun::MatchingFile => "warp_tui.tool_calls.counts.matching_file_other",
        CountedNoun::Document if singular => "warp_tui.tool_calls.counts.document_one",
        CountedNoun::Document => "warp_tui.tool_calls.counts.document_other",
        CountedNoun::Edit if singular => "warp_tui.tool_calls.counts.edit_one",
        CountedNoun::Edit => "warp_tui.tool_calls.counts.edit_other",
        CountedNoun::ReviewComment if singular => "warp_tui.tool_calls.counts.review_comment_one",
        CountedNoun::ReviewComment => "warp_tui.tool_calls.counts.review_comment_other",
        CountedNoun::Agent if singular => "warp_tui.tool_calls.counts.agent_one",
        CountedNoun::Agent => "warp_tui.tool_calls.counts.agent_other",
        CountedNoun::Question if singular => "warp_tui.tool_calls.counts.question_one",
        CountedNoun::Question => "warp_tui.tool_calls.counts.question_other",
    };
    t!(key, count = count).to_string()
}

#[cfg(test)]
#[path = "tool_call_labels_tests.rs"]
mod tests;
