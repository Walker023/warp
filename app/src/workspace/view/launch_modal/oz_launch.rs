use asset_macro::bundled_or_fetched_asset;
use markdown_parser::{FormattedTextFragment, FormattedTextLine};
use warp_core::send_telemetry_from_ctx;
use warpui::assets::asset_cache::AssetSource;
use warpui::{AppContext, SingletonEntity};

use super::{CTAButton, CheckboxConfig, LaunchModalEvent, Slide};
use crate::ai::ambient_agents::telemetry::{CloudAgentTelemetryEvent, CloudModeEntryPoint};
use crate::i18n::t;
use crate::terminal::view::OnboardingIntention;
use crate::ui_components::icons::Icon;
use crate::workspace::action::WorkspaceAction;
use crate::workspace::view::OnboardingTutorial;
use crate::workspaces::user_workspaces::UserWorkspaces;
use crate::workspaces::workspace::{AdminEnablementSetting, UgcCollectionEnablementSetting};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OzLaunchSlide {
    CloudAgents,
    AgentAutomations,
    AgentManagement,
    LaunchCredits,
}

impl Slide for OzLaunchSlide {
    fn modal_title(&self) -> String {
        t!("workspace_search_ui.workspace.oz_launch.modal_title").to_string()
    }

    fn modal_subtext_paragraphs(&self) -> Vec<FormattedTextLine> {
        vec![FormattedTextLine::Line(vec![
            FormattedTextFragment::plain_text(
                t!("workspace_search_ui.workspace.oz_launch.modal_subtext").to_string(),
            ),
        ])]
    }

    fn first() -> Self {
        OzLaunchSlide::CloudAgents
    }

    fn next(&self) -> Option<Self> {
        match self {
            OzLaunchSlide::CloudAgents => Some(OzLaunchSlide::AgentAutomations),
            OzLaunchSlide::AgentAutomations => Some(OzLaunchSlide::AgentManagement),
            OzLaunchSlide::AgentManagement => Some(OzLaunchSlide::LaunchCredits),
            OzLaunchSlide::LaunchCredits => None,
        }
    }

    fn prev(&self) -> Option<Self> {
        match self {
            OzLaunchSlide::CloudAgents => None,
            OzLaunchSlide::AgentAutomations => Some(OzLaunchSlide::CloudAgents),
            OzLaunchSlide::AgentManagement => Some(OzLaunchSlide::AgentAutomations),
            OzLaunchSlide::LaunchCredits => Some(OzLaunchSlide::AgentManagement),
        }
    }

    fn display_text(&self) -> Option<String> {
        Some(match self {
            OzLaunchSlide::CloudAgents => {
                t!("workspace_search_ui.workspace.oz_launch.cloud_agents").to_string()
            }
            OzLaunchSlide::AgentAutomations => {
                t!("workspace_search_ui.workspace.oz_launch.agent_automations").to_string()
            }
            OzLaunchSlide::AgentManagement => {
                t!("workspace_search_ui.workspace.oz_launch.agent_management").to_string()
            }
            OzLaunchSlide::LaunchCredits => {
                t!("workspace_search_ui.workspace.oz_launch.little_gift").to_string()
            }
        })
    }

    fn short_label(&self) -> String {
        match self {
            OzLaunchSlide::CloudAgents => {
                t!("workspace_search_ui.workspace.oz_launch.cloud_agents").to_string()
            }
            OzLaunchSlide::AgentAutomations => {
                t!("workspace_search_ui.workspace.oz_launch.agent_automations").to_string()
            }
            OzLaunchSlide::AgentManagement => {
                t!("workspace_search_ui.workspace.oz_launch.agent_management").to_string()
            }
            OzLaunchSlide::LaunchCredits => {
                t!("workspace_search_ui.workspace.oz_launch.launch_credits").to_string()
            }
        }
    }

    fn title(&self) -> String {
        match self {
            OzLaunchSlide::CloudAgents => {
                t!("workspace_search_ui.workspace.oz_launch.cloud_agents_title").to_string()
            }
            OzLaunchSlide::AgentAutomations => {
                t!("workspace_search_ui.workspace.oz_launch.agent_automations_title").to_string()
            }
            OzLaunchSlide::AgentManagement => {
                t!("workspace_search_ui.workspace.oz_launch.agent_management_title").to_string()
            }
            OzLaunchSlide::LaunchCredits => {
                t!("workspace_search_ui.workspace.oz_launch.launch_credits_title").to_string()
            }
        }
    }

    fn title_icon(&self) -> Option<Icon> {
        None
    }

    fn content(&self) -> String {
        match self {
            OzLaunchSlide::CloudAgents => {
                t!("workspace_search_ui.workspace.oz_launch.cloud_agents_content").to_string()
            }
            OzLaunchSlide::AgentAutomations => {
                t!("workspace_search_ui.workspace.oz_launch.agent_automations_content").to_string()
            }
            OzLaunchSlide::AgentManagement => {
                t!("workspace_search_ui.workspace.oz_launch.agent_management_content").to_string()
            }
            OzLaunchSlide::LaunchCredits => {
                t!("workspace_search_ui.workspace.oz_launch.launch_credits_content").to_string()
            }
        }
    }

    fn image(&self) -> AssetSource {
        // TODO: Replace with new images once provided.
        match self {
            OzLaunchSlide::CloudAgents => {
                bundled_or_fetched_asset!("png/oz_cloud_agents.png")
            }
            OzLaunchSlide::AgentAutomations => {
                bundled_or_fetched_asset!("png/oz_agent_automations.png")
            }
            OzLaunchSlide::AgentManagement => {
                bundled_or_fetched_asset!("png/oz_agent_management.png")
            }
            OzLaunchSlide::LaunchCredits => {
                bundled_or_fetched_asset!("png/oz_launch_credits.png")
            }
        }
    }

    fn all() -> Vec<Self> {
        vec![
            OzLaunchSlide::CloudAgents,
            OzLaunchSlide::AgentAutomations,
            OzLaunchSlide::AgentManagement,
            OzLaunchSlide::LaunchCredits,
        ]
    }

    fn cta_button(&self) -> CTAButton<Self> {
        match self {
            OzLaunchSlide::CloudAgents
            | OzLaunchSlide::AgentAutomations
            | OzLaunchSlide::AgentManagement => {
                let next = self.next().expect("Non-final slides should have a next");
                CTAButton::next_slide(
                    next,
                    t!(
                        "workspace_search_ui.workspace.oz_launch.next",
                        name = next.short_label()
                    )
                    .to_string(),
                )
            }
            OzLaunchSlide::LaunchCredits => CTAButton::custom(
                t!("workspace_search_ui.workspace.oz_launch.try_it_out").to_string(),
                |ctx| {
                    send_telemetry_from_ctx!(
                        CloudAgentTelemetryEvent::EnteredCloudMode {
                            entry_point: CloudModeEntryPoint::OzLaunchModal,
                        },
                        ctx
                    );
                    ctx.emit(LaunchModalEvent::Close);
                    ctx.dispatch_typed_action(&WorkspaceAction::StartAgentOnboardingTutorial(
                        OnboardingTutorial::NoProject {
                            intention: OnboardingIntention::AgentDrivenDevelopment,
                        },
                    ));
                    ctx.dispatch_typed_action(&WorkspaceAction::AddAmbientAgentTab);
                },
            ),
        }
    }

    fn secondary_cta_button(&self) -> Option<CTAButton<Self>> {
        match self {
            OzLaunchSlide::LaunchCredits => Some(CTAButton::close(
                t!("workspace_search_ui.workspace.oz_launch.skip_for_now").to_string(),
            )),
            OzLaunchSlide::CloudAgents
            | OzLaunchSlide::AgentAutomations
            | OzLaunchSlide::AgentManagement => None,
        }
    }

    fn checkbox_config(&self) -> Option<CheckboxConfig> {
        Some(CheckboxConfig {
            label: t!("workspace_search_ui.workspace.oz_launch.sync_conversations").to_string(),
            description: t!(
                "workspace_search_ui.workspace.oz_launch.sync_conversations_description"
            )
            .to_string(),
        })
    }

    fn should_show_checkbox(&self, app: &AppContext) -> bool {
        let cloud_storage_setting =
            UserWorkspaces::as_ref(app).get_cloud_conversation_storage_enablement_setting();
        let ugc_setting = UserWorkspaces::as_ref(app).get_ugc_collection_enablement_setting();

        // Show checkbox only when user has control over cloud storage AND UGC is not force-enabled.
        matches!(
            cloud_storage_setting,
            AdminEnablementSetting::RespectUserSetting
        ) && !matches!(ugc_setting, UgcCollectionEnablementSetting::Enable)
    }

    fn on_close(&self, ctx: &mut warpui::ViewContext<super::LaunchModal<Self>>) {
        ctx.dispatch_typed_action(&WorkspaceAction::StartAgentOnboardingTutorial(
            OnboardingTutorial::NoProject {
                intention: OnboardingIntention::AgentDrivenDevelopment,
            },
        ));
    }
}

pub fn init(app: &mut warpui::AppContext) {
    super::init::<OzLaunchSlide>(app);
}
