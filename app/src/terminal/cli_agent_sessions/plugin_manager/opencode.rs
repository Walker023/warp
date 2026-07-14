use std::sync::LazyLock;

use async_trait::async_trait;

use super::{CliAgentPluginManager, PluginInstructionStep, PluginInstructions};

// Keep in sync with the opencode-warp npm package version.
// This version is also hardcoded into UPDATE_INSTRUCTIONS below (so the update
// instructions tell users to pin to this specific version to force OpenCode's
// plugin cache to re-fetch). Update both together.
const MINIMUM_PLUGIN_VERSION: &str = "0.1.5";

pub(super) struct OpenCodePluginManager;

#[async_trait]
impl CliAgentPluginManager for OpenCodePluginManager {
    fn minimum_plugin_version(&self) -> &'static str {
        MINIMUM_PLUGIN_VERSION
    }

    fn can_auto_install(&self) -> bool {
        false
    }

    fn install_instructions(&self) -> &'static PluginInstructions {
        &INSTALL_INSTRUCTIONS
    }

    fn update_instructions(&self) -> &'static PluginInstructions {
        &UPDATE_INSTRUCTIONS
    }
}

static INSTALL_INSTRUCTIONS: LazyLock<PluginInstructions> = LazyLock::new(|| PluginInstructions {
    title: "terminal_ui.plugin_instructions.opencode.install_title",
    subtitle: "terminal_ui.plugin_instructions.opencode.install_subtitle",
    steps: &[
        PluginInstructionStep {
            description: "terminal_ui.plugin_instructions.opencode.open_config",
            command: "~/.config/opencode/opencode.json",
            executable: false,
            link: None,
        },
        PluginInstructionStep {
            description: "terminal_ui.plugin_instructions.opencode.add_plugin",
            command: "\"plugin\": [\"@warp-dot-dev/opencode-warp\"]",
            executable: false,
            link: None,
        },
    ],
    post_install_notes: &["terminal_ui.plugin_instructions.opencode.restart_after_install"],
});

static UPDATE_INSTRUCTIONS: LazyLock<PluginInstructions> = LazyLock::new(|| PluginInstructions {
    title: "terminal_ui.plugin_instructions.opencode.update_title",
    subtitle: "terminal_ui.plugin_instructions.opencode.update_subtitle",
    steps: &[
        PluginInstructionStep {
            description: "terminal_ui.plugin_instructions.opencode.open_config",
            command: "~/.config/opencode/opencode.json",
            executable: false,
            link: None,
        },
        PluginInstructionStep {
            description: "terminal_ui.plugin_instructions.opencode.replace_plugin",
            command: "\"plugin\": [\"@warp-dot-dev/opencode-warp@0.1.5\"]",
            executable: false,
            link: None,
        },
    ],
    post_install_notes: &["terminal_ui.plugin_instructions.opencode.restart_after_update"],
});

#[cfg(test)]
#[path = "opencode_tests.rs"]
mod tests;
