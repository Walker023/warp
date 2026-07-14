use std::borrow::Cow;

use rust_i18n::t;

use super::RenderableBlock;
use super::paint::RenderContext;
use super::placeholder::{self, BlockPlaceholder};
use crate::content::text::BufferBlockStyle;
use crate::extract_block;
use crate::render::model::viewport::ViewportItem;
use crate::render::model::{BlockItem, RenderState};

/// The placeholder text to show in empty plain-text blocks.
pub fn paragraph_placeholder_text(slash_menu_enabled: bool) -> Cow<'static, str> {
    if slash_menu_enabled {
        t!("editor.placeholder.paragraph_with_slash")
    } else {
        t!("editor.placeholder.paragraph_without_slash")
    }
}

/// [`RenderableBlock`] implementation for `Paragraph` blocks.
pub struct RenderableParagraph {
    viewport_item: ViewportItem,
    placeholder: BlockPlaceholder,
}

impl RenderableParagraph {
    pub fn new(viewport_item: ViewportItem) -> Self {
        Self {
            viewport_item,
            placeholder: BlockPlaceholder::new(false),
        }
    }
}

impl RenderableBlock for RenderableParagraph {
    fn viewport_item(&self) -> &ViewportItem {
        &self.viewport_item
    }

    fn layout(
        &mut self,
        model: &RenderState,
        ctx: &mut warpui_core::LayoutContext,
        app: &warpui_core::AppContext,
    ) {
        let placeholder_text = paragraph_placeholder_text(model.selections().len() == 1);
        self.placeholder
            .layout(&self.viewport_item, model, ctx, app, |_| {
                placeholder::Options {
                    text: placeholder_text.as_ref(),
                    block_style: BufferBlockStyle::PlainText,
                }
            });
    }

    fn paint(
        &mut self,
        model: &RenderState,
        ctx: &mut RenderContext,
        _app: &warpui_core::AppContext,
    ) {
        let content = model.content();
        let paragraph = extract_block!(self.viewport_item, content, (block, BlockItem::Paragraph(inner)) => block.paragraph(inner));

        if self
            .placeholder
            .paint(paragraph.content_origin(), model, ctx)
        {
            return;
        }

        let paragraph_styles = &model.styles().base_text;
        ctx.draw_paragraph(&paragraph, paragraph_styles, model);
    }
}
