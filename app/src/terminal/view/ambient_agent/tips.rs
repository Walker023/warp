//! Tips for cloud mode loading screen.

use warpui::keymap::Keystroke;
use warpui::AppContext;

use crate::ai::agent_tips::AITip;
use crate::i18n::t;

/// A cloud mode tip with text and optional link.
#[derive(Clone, Debug)]
pub struct CloudModeTip {
    text: String,
    link: Option<String>,
}

impl CloudModeTip {
    pub fn new(text: impl Into<String>, link: Option<impl Into<String>>) -> Self {
        Self {
            text: text.into(),
            link: link.map(|l| l.into()),
        }
    }
}

impl AITip for CloudModeTip {
    fn keystroke(&self, _app: &AppContext) -> Option<Keystroke> {
        None
    }

    fn link(&self) -> Option<String> {
        self.link.clone()
    }

    fn description(&self) -> &str {
        &self.text
    }

    // Uses the default implementation which adds "Tip: " prefix and parses backticks as inline code
}

/// Returns a collection of tips for the cloud mode loading screen.
pub fn get_cloud_mode_tips() -> Vec<CloudModeTip> {
    vec![
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.install_slack").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/slack"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.build_with_sdks").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.set_secrets").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.view_runs").to_string(),
            Some("https://oz.warp.dev"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.join_shared_run").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/viewing-cloud-agent-runs"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.recurring_agents").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.linear_bug_fix").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/linear"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.ci_failure_fix").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.github_action").to_string(),
            Some("https://github.com/warpdotdev/oz-agent-action"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.rest_api").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.reusable_environments").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/environments"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.share_links").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/viewing-cloud-agent-runs"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.share_flag").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.fork_locally").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/viewing-cloud-agent-runs"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.database_tools").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.clean_feature_flags").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.tag_oz_linear").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/linear"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.remote_runners").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.configure_mcp").to_string(),
            Some("https://docs.warp.dev/agent-platform/capabilities/mcp"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.oz_agent_run").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.view_team_runs").to_string(),
            Some("https://oz.warp.dev"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.triage_github").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.daily_issue_summary").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.review_prs").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.create_environment").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/environments"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.webhook_incidents").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.respond_to_alerts").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.personal_secrets").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.team_secrets").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.dependency_updates").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.format_and_lint").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.create_schedule").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.pause_schedule").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.list_mcp").to_string(),
            Some("https://docs.warp.dev/agent-platform/capabilities/mcp"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.slack_bot").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/slack"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.slack_mentions").to_string(),
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/slack"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.typescript_sdk").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.python_sdk").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.monitor_api").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            t!("terminal_ui.ambient_agent.tips.activity_dashboard").to_string(),
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
    ]
}
