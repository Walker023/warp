use warp_graphql::ai::{AgentTaskState, PlatformErrorCode};

use super::terminal::ShareSessionError;
use super::AgentDriverError;
use crate::ai::blocklist::local_agent_task_sync_model::classify_renderable_error;
use crate::i18n::t;
use crate::server::server_api::ai::TaskStatusUpdate;

/// Classify an `AgentDriverError` into a task state and a `TaskStatusUpdate`
/// suitable for reporting via `update_agent_task`.
pub fn classify_driver_error(error: &AgentDriverError) -> (AgentTaskState, TaskStatusUpdate) {
    match error {
        // --- Warp-side errors (task → ERROR) ---
        AgentDriverError::TerminalUnavailable | AgentDriverError::InvalidRuntimeState => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.internal").to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::BootstrapFailed { error } => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.terminal_session_start_failed",
                    error = error
                )
                .to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::ShareSessionFailed { error: share_err } => {
            let message = match share_err {
                ShareSessionError::Internal(_) => {
                    t!("ai_driver.errors.share_session.internal").to_string()
                }
                ShareSessionError::Failed(reason) => {
                    // The reason string comes from the session-sharing layer and is aimed at
                    // interactive users (e.g. "try sharing again"). Provide a cloud-agent-
                    // appropriate message instead of wrapping it, which would produce
                    // repetitive "try again" text.
                    t!("ai_driver.errors.share_session.failed", reason = reason).to_string()
                }
                ShareSessionError::Disabled => {
                    t!("ai_driver.errors.share_session.disabled").to_string()
                }
                ShareSessionError::Timeout => {
                    t!("ai_driver.errors.share_session.timeout").to_string()
                }
                ShareSessionError::Interrupted => {
                    t!("ai_driver.errors.share_session.interrupted").to_string()
                }
            };
            (
                AgentTaskState::Error,
                TaskStatusUpdate::with_error_code(
                    message,
                    match share_err {
                        ShareSessionError::Disabled => PlatformErrorCode::FeatureNotAvailable,
                        _ => PlatformErrorCode::InternalError,
                    },
                ),
            )
        }
        AgentDriverError::WarpDriveSyncFailed => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.warp_drive_sync_failed").to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::NotLoggedIn => {
            let bin = warp_cli::binary_name().unwrap_or_else(|| "warp".to_string());
            (
                AgentTaskState::Error,
                TaskStatusUpdate::with_error_code(
                    t!("ai_driver.errors.authentication_required", bin = &bin).to_string(),
                    PlatformErrorCode::AuthenticationRequired,
                ),
            )
        }
        AgentDriverError::CloudProviderSetupFailed(err) => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.cloud_access_configuration_failed",
                    error = format!("{err:#}")
                )
                .to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),

        // --- User-side errors (task → FAILED) ---
        AgentDriverError::MCPServerNotFound(uuid) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.mcp_server_not_found", uuid = uuid).to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::ManagedMcpResolutionFailed { uid, message } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.managed_mcp_resolution_failed",
                    uid = uid,
                    message = message
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::MCPStartupFailed { details } => {
            let server_lines = details
                .iter()
                .map(|detail| format!("- {detail}"))
                .collect::<Vec<_>>()
                .join("\n");
            (
                AgentTaskState::Failed,
                TaskStatusUpdate::with_error_code(
                    t!(
                        "ai_driver.errors.mcp_startup_failed",
                        server_lines = &server_lines
                    )
                    .to_string(),
                    PlatformErrorCode::EnvironmentSetupFailed,
                ),
            )
        }
        AgentDriverError::MCPJsonParseError(msg) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.mcp_json_parse_failed", message = msg).to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::MCPMissingVariables => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.mcp_missing_variables").to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::ProfileError(name) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.profile_not_found", name = name).to_string(),
                PlatformErrorCode::ResourceNotFound,
            ),
        ),
        AgentDriverError::AIWorkflowNotFound(id) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.saved_prompt_not_found", id = id).to_string(),
                PlatformErrorCode::ResourceNotFound,
            ),
        ),
        AgentDriverError::EnvironmentNotFound(id) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.environment_not_found", id = id).to_string(),
                PlatformErrorCode::ResourceNotFound,
            ),
        ),
        AgentDriverError::EnvironmentSetupFailed(msg) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.environment_setup_failed", message = msg).to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::InvalidWorkingDirectory { path, .. } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.invalid_working_directory",
                    path = path.display()
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),

        // --- Conversation errors ---
        // Delegate to classify_renderable_error for proper ERROR vs FAILED
        // distinction and PlatformErrorCode. This is a belt-and-suspenders
        // fallback — LocalAgentTaskSyncModel handles most conversation errors,
        // but the driver catches them too if the conversation ends with an error.
        AgentDriverError::ConversationError { error } => {
            let (state, update) = classify_renderable_error(error);
            (
                state,
                update.unwrap_or_else(|| {
                    TaskStatusUpdate::with_error_code(
                        error.to_string(),
                        PlatformErrorCode::InternalError,
                    )
                }),
            )
        }

        // --- Cancellation / Blocked (no error code) ---
        AgentDriverError::ConversationCancelled { .. } => (
            AgentTaskState::Cancelled,
            TaskStatusUpdate::message(t!("ai_driver.status.task_cancelled").to_string()),
        ),
        AgentDriverError::ConversationBlocked { blocked_action } => (
            AgentTaskState::Blocked,
            TaskStatusUpdate::message(
                t!(
                    "ai_driver.status.conversation_blocked",
                    action = blocked_action
                )
                .to_string(),
            ),
        ),

        // --- Setup errors ---
        AgentDriverError::TeamMetadataRefreshTimeout => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.team_metadata_refresh_timeout").to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::SkillResolutionFailed(msg) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.skill_resolution_failed", message = msg).to_string(),
                PlatformErrorCode::ResourceNotFound,
            ),
        ),
        AgentDriverError::ConfigBuildFailed(err) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.config_build_failed", error = err).to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::PromptResolutionFailed(err) => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.prompt_resolution_failed", error = err).to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::SecretsFetchFailed(err) => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.secrets_fetch_failed", error = err).to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::AwsBedrockCredentialsFailed(msg) => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.aws_bedrock_credentials_failed",
                    message = msg
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::ConversationLoadFailed(msg) => (
            AgentTaskState::Error,
            TaskStatusUpdate::with_error_code(
                t!("ai_driver.errors.conversation_load_failed", message = msg).to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::ConversationHarnessMismatch {
            conversation_id,
            expected,
            got,
        } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.conversation_harness_mismatch",
                    conversation_id = conversation_id,
                    expected = expected,
                    got = got
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::TaskHarnessMismatch {
            task_id,
            expected,
            got,
        } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.task_harness_mismatch",
                    task_id = task_id,
                    expected = expected,
                    got = got
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::ConversationResumeStateMissing {
            harness,
            conversation_id,
        } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.conversation_resume_state_missing",
                    conversation_id = conversation_id,
                    harness = harness
                )
                .to_string(),
                PlatformErrorCode::ResourceNotFound,
            ),
        ),
        AgentDriverError::HarnessCommandFailed { exit_code } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.harness_command_failed",
                    exit_code = exit_code
                )
                .to_string(),
                PlatformErrorCode::InternalError,
            ),
        ),
        AgentDriverError::HarnessSetupFailed { harness, reason } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.harness_setup_failed",
                    harness = harness,
                    reason = reason
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::HarnessConfigSetupFailed { harness, error } => (
            AgentTaskState::Failed,
            TaskStatusUpdate::with_error_code(
                t!(
                    "ai_driver.errors.harness_config_setup_failed",
                    harness = harness,
                    error = error
                )
                .to_string(),
                PlatformErrorCode::EnvironmentSetupFailed,
            ),
        ),
        AgentDriverError::HarnessAuthCheckFailed { harness, .. } => {
            let message = t!(
                "ai_driver.errors.harness_auth_check_failed",
                harness = harness
            )
            .to_string();
            (
                AgentTaskState::Failed,
                TaskStatusUpdate::with_error_code(
                    message,
                    PlatformErrorCode::AuthenticationRequired,
                ),
            )
        }
        AgentDriverError::HarnessRuntimeFailureDetected {
            harness,
            pattern,
            excerpt,
        } => {
            let message = t!(
                "ai_driver.errors.harness_runtime_failure_detected",
                harness = harness,
                pattern = pattern,
                excerpt = excerpt
            )
            .to_string();
            (
                AgentTaskState::Failed,
                TaskStatusUpdate::with_error_code(
                    message,
                    PlatformErrorCode::AuthenticationRequired,
                ),
            )
        }
    }
}

#[cfg(test)]
#[path = "error_classification_tests.rs"]
mod tests;
