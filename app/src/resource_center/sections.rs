use warp_core::context_flag::ContextFlag;
use warp_core::features::FeatureFlag;
use warpui::ViewContext;

use super::{
    ContentItem, ContentSectionData, FeatureItem, FeatureSection, FeatureSectionData,
    ResourceCenterMainView, Section, Tip, TipAction, TipHint,
};
use crate::i18n::t;

pub fn sections(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<Section> {
    let mut sections = vec![Section::Changelog()];

    if FeatureFlag::AvatarInTabBar.is_enabled() {
        return sections;
    }

    let get_started = FeatureSectionData {
        section_name: FeatureSection::GettingStarted,
        items: vec![
            FeatureItem::new(
                t!("resource_center.items.create_first_block.title"),
                t!("resource_center.items.create_first_block.description"),
                Tip::Hint(TipHint::CreateBlock),
                ctx,
            ),
            FeatureItem::new(
                t!("resource_center.items.navigate_blocks.title"),
                t!("resource_center.items.navigate_blocks.description"),
                Tip::Hint(TipHint::BlockSelect),
                ctx,
            ),
            FeatureItem::new(
                t!("resource_center.items.block_action.title"),
                t!("resource_center.items.block_action.description"),
                Tip::Hint(TipHint::BlockAction),
                ctx,
            ),
            FeatureItem::new(
                t!("resource_center.items.command_palette.title"),
                t!("resource_center.items.command_palette.description"),
                Tip::Action(TipAction::CommandPalette),
                ctx,
            ),
            FeatureItem::new(
                t!("resource_center.items.theme_picker.title"),
                t!("resource_center.items.theme_picker.description"),
                Tip::Action(TipAction::ThemePicker),
                ctx,
            ),
        ],
    };
    sections.push(Section::Feature(get_started));

    let maximize_warp = FeatureSectionData {
        section_name: FeatureSection::MaximizeWarp,
        items: maximize_warp_items(ctx),
    };
    sections.push(Section::Feature(maximize_warp));

    let advanced_setup = ContentSectionData {
        section_name: FeatureSection::AdvancedSetup,
        items: vec![
            ContentItem {
                title: t!("resource_center.items.custom_prompt.title").to_string(),
                description: t!("resource_center.items.custom_prompt.description").to_string(),
                url: "https://docs.warp.dev/terminal/appearance/prompt",
                button_label: t!("resource_center.view_documentation").to_string(),
            },
            ContentItem {
                title: t!("resource_center.items.ide_integration.title").to_string(),
                description: t!("resource_center.items.ide_integration.description").to_string(),
                url: "https://docs.warp.dev/terminal/integrations-and-plugins",
                button_label: t!("resource_center.view_documentation").to_string(),
            },
            ContentItem {
                title: t!("resource_center.items.how_warp_uses_warp.title").to_string(),
                description: t!("resource_center.items.how_warp_uses_warp.description").to_string(),
                url: "https://www.warp.dev/blog/how-warp-uses-warp",
                button_label: t!("resource_center.read_article").to_string(),
            },
        ],
    };
    sections.push(Section::Content(advanced_setup));

    sections
}

fn maximize_warp_items(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<FeatureItem> {
    let mut maximize_warp_items = vec![];

    maximize_warp_items.push(FeatureItem::new(
        t!("resource_center.items.command_search.title"),
        t!("resource_center.items.command_search.description"),
        Tip::Action(TipAction::CommandSearch),
        ctx,
    ));

    maximize_warp_items.push(FeatureItem::new(
        t!("resource_center.items.ai_command_search.title"),
        t!("resource_center.items.ai_command_search.description"),
        Tip::Action(TipAction::AiCommandSearch),
        ctx,
    ));

    if ContextFlag::CreateNewSession.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            t!("resource_center.items.split_panes.title"),
            t!("resource_center.items.split_panes.description"),
            Tip::Action(TipAction::SplitPane),
            ctx,
        ));
    }

    if ContextFlag::LaunchConfigurations.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            t!("resource_center.items.launch_configuration.title"),
            t!("resource_center.items.launch_configuration.description"),
            Tip::Action(TipAction::SaveNewLaunchConfig),
            ctx,
        ));
    }

    maximize_warp_items
}
