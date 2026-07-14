use warpui::elements::{
    Container, CornerRadius, CrossAxisAlignment, Flex, MainAxisSize, MouseStateHandle,
    ParentElement, Radius, Shrinkable, Text,
};
use warpui::fonts::Weight;
use warpui::platform::Cursor;
use warpui::ui_components::button::ButtonVariant;
use warpui::ui_components::components::{Coords, UiComponent, UiComponentStyles};
use warpui::{AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext};

use crate::appearance::Appearance;
use crate::drive::DriveObjectType;
use crate::i18n::t;
use crate::ui_components::blended_colors;
use crate::workspaces::workspace::{BillingMetadata, CustomerType};

const BUTTON_PADDING: f32 = 12.;
const BUTTON_FONT_SIZE: f32 = 14.;
const BUTTON_BORDER_RADIUS: f32 = 4.;

#[derive(Default)]
struct MouseStateHandles {
    button_mouse_state: MouseStateHandle,
}

pub struct SharedObjectsCreationDeniedBody {
    object_type: Option<DriveObjectType>,
    has_admin_permissions: bool,
    is_delinquent_due_to_payment_issue: bool,
    customer_type: CustomerType,
    button_mouse_states: MouseStateHandles,
}

#[derive(Debug, Clone, Copy)]
pub enum SharedObjectsCreationDeniedBodyAction {
    Upgrade,
    ManageBilling,
}

pub enum SharedObjectsCreationDeniedBodyEvent {
    Upgrade,
    ManageBilling,
}

pub(super) fn localized_object_types(object_type: Option<DriveObjectType>) -> String {
    match object_type {
        None => t!("billing_extra.object_types.drive_objects").to_string(),
        Some(DriveObjectType::Workflow) => t!("billing_extra.object_types.workflows").to_string(),
        Some(DriveObjectType::AgentModeWorkflow) => {
            t!("billing_extra.object_types.prompts").to_string()
        }
        Some(DriveObjectType::AIFact) => t!("billing_extra.object_types.ai_facts").to_string(),
        Some(DriveObjectType::AIFactCollection) => {
            t!("billing_extra.object_types.ai_fact_collections").to_string()
        }
        Some(DriveObjectType::Notebook { .. }) => {
            t!("billing_extra.object_types.notebooks").to_string()
        }
        Some(DriveObjectType::Folder) => t!("billing_extra.object_types.folders").to_string(),
        Some(DriveObjectType::EnvVarCollection) => {
            t!("billing_extra.object_types.env_var_collections").to_string()
        }
        Some(DriveObjectType::MCPServer) => {
            t!("billing_extra.object_types.mcp_servers").to_string()
        }
        Some(DriveObjectType::MCPServerCollection) => {
            t!("billing_extra.object_types.mcp_server_collections").to_string()
        }
    }
}

impl SharedObjectsCreationDeniedBody {
    pub fn new(object_type: Option<DriveObjectType>) -> Self {
        Self {
            object_type,
            has_admin_permissions: false,
            is_delinquent_due_to_payment_issue: false,
            customer_type: Default::default(),
            button_mouse_states: Default::default(),
        }
    }

    pub fn update_state(
        &mut self,
        object_type: DriveObjectType,
        has_admin_permissions: bool,
        is_delinquent_due_to_payment_issue: bool,
        customer_type: CustomerType,
        ctx: &mut ViewContext<Self>,
    ) {
        self.object_type = Some(object_type);
        self.has_admin_permissions = has_admin_permissions;
        self.is_delinquent_due_to_payment_issue = is_delinquent_due_to_payment_issue;
        self.customer_type = customer_type;
        ctx.notify();
    }
}

impl Entity for SharedObjectsCreationDeniedBody {
    type Event = SharedObjectsCreationDeniedBodyEvent;
}

impl View for SharedObjectsCreationDeniedBody {
    fn ui_name() -> &'static str {
        "SharedObjectsCreationDeniedBody"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = Appearance::as_ref(app);
        let is_stripe_paid_plan = BillingMetadata::is_stripe_paid_plan(self.customer_type);

        let object_types = localized_object_types(self.object_type);
        let has_specific_object_type = self.object_type.is_some();
        let sub_header = match (
            self.is_delinquent_due_to_payment_issue,
            self.has_admin_permissions,
            self.customer_type,
        ) {
            (true, true, _) if is_stripe_paid_plan => t!(
                "billing_extra.restricted_update",
                object_types = object_types
            )
            .to_string(),
            (true, true, _) => t!(
                "billing_extra.restricted_support",
                object_types = object_types
            )
            .to_string(),
            (true, false, _) => t!(
                "billing_extra.restricted_admin",
                object_types = object_types
            )
            .to_string(),
            (false, true, CustomerType::Prosumer) if has_specific_object_type => t!(
                "billing_extra.pro_upgrade_build",
                object_types = object_types
            )
            .to_string(),
            (false, false, CustomerType::Prosumer) if has_specific_object_type => {
                t!("billing_extra.pro_admin_build", object_types = object_types).to_string()
            }
            (false, true, CustomerType::Prosumer) => t!(
                "billing_extra.pro_upgrade_turbo",
                object_types = object_types
            )
            .to_string(),
            (false, false, CustomerType::Prosumer) => {
                t!("billing_extra.pro_admin_turbo", object_types = object_types).to_string()
            }
            (false, true, _) => {
                t!("billing_extra.free_upgrade", object_types = object_types).to_string()
            }
            (false, false, _) => {
                t!("billing_extra.free_admin", object_types = object_types).to_string()
            }
        };

        let mut body = Flex::column()
            .with_cross_axis_alignment(CrossAxisAlignment::Stretch)
            .with_child(
                Container::new(
                    Text::new(sub_header, appearance.ui_font_family(), 14.)
                        .with_color(blended_colors::text_sub(
                            appearance.theme(),
                            appearance.theme().background(),
                        ))
                        .finish(),
                )
                .finish(),
            );

        // Only render an action button if:
        // 1. the team is delinquent + user is an admin + the team is on a stripe paid plan
        // OR
        // 2. if the team is not delinquent.
        // In the case where the team is delinquent and user is NOT an admin, or if the
        // team is delinquent but the team is not on a stripe paid plan, we don't render
        // any action button.
        if self.is_delinquent_due_to_payment_issue
            && self.has_admin_permissions
            && is_stripe_paid_plan
        {
            body.add_child(
                Container::new(
                    Flex::row()
                        .with_child(
                            Shrinkable::new(
                                0.5,
                                self.render_button(
                                    appearance,
                                    t!("billing_extra.manage_billing").to_string(),
                                    self.button_mouse_states.button_mouse_state.clone(),
                                    SharedObjectsCreationDeniedBodyAction::ManageBilling,
                                ),
                            )
                            .finish(),
                        )
                        .with_main_axis_size(MainAxisSize::Max)
                        .finish(),
                )
                .with_margin_top(24.)
                .finish(),
            )
        } else if !self.is_delinquent_due_to_payment_issue {
            body.add_child(
                Container::new(
                    Flex::row()
                        .with_child(
                            Shrinkable::new(
                                0.5,
                                self.render_button(
                                    appearance,
                                    t!("billing_extra.compare_plans").to_string(),
                                    self.button_mouse_states.button_mouse_state.clone(),
                                    SharedObjectsCreationDeniedBodyAction::Upgrade,
                                ),
                            )
                            .finish(),
                        )
                        .with_main_axis_size(MainAxisSize::Max)
                        .finish(),
                )
                .with_margin_top(24.)
                .finish(),
            )
        }

        body.finish()
    }
}

impl SharedObjectsCreationDeniedBody {
    fn render_button(
        &self,
        appearance: &Appearance,
        label: String,
        mouse_state: MouseStateHandle,
        action: SharedObjectsCreationDeniedBodyAction,
    ) -> Box<dyn Element> {
        appearance
            .ui_builder()
            .button(ButtonVariant::Accent, mouse_state)
            .with_centered_text_label(label)
            .with_style(UiComponentStyles {
                font_size: Some(BUTTON_FONT_SIZE),
                font_weight: Some(Weight::Semibold),
                border_radius: Some(CornerRadius::with_all(Radius::Pixels(BUTTON_BORDER_RADIUS))),
                padding: Some(Coords::uniform(BUTTON_PADDING)),
                ..Default::default()
            })
            .build()
            .with_cursor(Cursor::PointingHand)
            .on_click(move |ctx, _, _| ctx.dispatch_typed_action(action))
            .finish()
    }
}

impl TypedActionView for SharedObjectsCreationDeniedBody {
    type Action = SharedObjectsCreationDeniedBodyAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            SharedObjectsCreationDeniedBodyAction::Upgrade => {
                ctx.emit(SharedObjectsCreationDeniedBodyEvent::Upgrade)
            }
            SharedObjectsCreationDeniedBodyAction::ManageBilling => {
                ctx.emit(SharedObjectsCreationDeniedBodyEvent::ManageBilling)
            }
        }
    }
}
