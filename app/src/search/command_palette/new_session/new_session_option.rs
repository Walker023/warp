use std::borrow::Cow;

use warpui::Action;

use crate::i18n::t;
use crate::server::telemetry::AddTabWithShellSource;
use crate::terminal::available_shells::AvailableShell;
use crate::terminal::view::TerminalAction;
use crate::WorkspaceAction;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NewSessionOptionId(pub(crate) String);
impl NewSessionOptionId {
    #[cfg_attr(not(feature = "local_tty"), allow(dead_code))]
    pub(super) fn new(s: String) -> Self {
        Self(s)
    }
}

#[derive(Debug)]
pub(super) enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    fn label(&self) -> String {
        match self {
            Direction::Down => t!("command_palette.new_session.direction_down").to_string(),
            Direction::Right => t!("command_palette.new_session.direction_right").to_string(),
            Direction::Up => t!("command_palette.new_session.direction_up").to_string(),
            Direction::Left => t!("command_palette.new_session.direction_left").to_string(),
        }
    }
}

#[derive(Debug)]
pub(super) enum NewSessionConfig {
    NewTab(AvailableShell),
    NewWindow(AvailableShell),
    Split(Direction, AvailableShell),
}

impl NewSessionConfig {
    fn shell(&self) -> &AvailableShell {
        match self {
            NewSessionConfig::NewTab(shell) => shell,
            NewSessionConfig::NewWindow(shell) => shell,
            NewSessionConfig::Split(_, shell) => shell,
        }
    }
}

#[derive(Debug)]
/// An option for creating a new terminal session
///
/// Contains configuration information like:
/// - which shell to use
/// - how to display the option in the command palette
pub struct NewSessionOption {
    id: NewSessionOptionId,
    description: String,
    config: NewSessionConfig,
}

impl NewSessionOption {
    pub fn id(&self) -> &NewSessionOptionId {
        &self.id
    }

    /// Returns the description (a.k.a. the top line in the command palette entry)
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl NewSessionOption {
    pub(super) fn new(id: NewSessionOptionId, config: NewSessionConfig) -> Self {
        let description = match &config {
            NewSessionConfig::NewTab(shell) => t!(
                "command_palette.new_session.create_new_tab_with_shell",
                shell = shell.short_name()
            )
            .to_string(),
            NewSessionConfig::NewWindow(shell) => t!(
                "command_palette.new_session.create_new_window_with_shell",
                shell = shell.short_name()
            )
            .to_string(),
            NewSessionConfig::Split(direction, shell) => t!(
                "command_palette.new_session.split_pane_with_shell",
                direction = direction.label(),
                shell = shell.short_name()
            )
            .to_string(),
        };
        Self {
            id,
            description,
            config,
        }
    }

    /// Returns an action that should be triggered if this entry is accepted
    pub fn action(&self) -> Box<dyn Action> {
        match &self.config {
            NewSessionConfig::NewTab(shell) => Box::new(WorkspaceAction::AddTabWithShell {
                shell: shell.clone(),
                source: AddTabWithShellSource::CommandPalette,
            }),
            NewSessionConfig::NewWindow(shell) => Box::new(WorkspaceAction::AddWindowWithShell {
                shell: shell.clone(),
            }),
            NewSessionConfig::Split(Direction::Down, shell) => {
                Box::new(TerminalAction::SplitDown(Some(shell.clone())))
            }
            NewSessionConfig::Split(Direction::Up, shell) => {
                Box::new(TerminalAction::SplitUp(Some(shell.clone())))
            }
            NewSessionConfig::Split(Direction::Right, shell) => {
                Box::new(TerminalAction::SplitRight(Some(shell.clone())))
            }
            NewSessionConfig::Split(Direction::Left, shell) => {
                Box::new(TerminalAction::SplitLeft(Some(shell.clone())))
            }
        }
    }

    /// Returns the details (a.k.a. the second line in the command palette entry)
    pub fn details(&self) -> Cow<'_, str> {
        self.config.shell().details()
    }
}
