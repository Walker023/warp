use warpui::AppContext;

use super::{CloudObject, GenericStringObjectFormat, JsonObjectType, ObjectType};
use crate::i18n::t;
use crate::server::cloud_objects::update_manager::{
    InitiatedBy, ObjectOperation, OperationSuccessType,
};

pub struct CloudObjectToastMessage;

impl CloudObjectToastMessage {
    pub fn toast_message(
        object: &dyn CloudObject,
        operation: &ObjectOperation,
        success_type: &OperationSuccessType,
        app: &AppContext,
    ) -> Option<String> {
        let object_name = localized_object_name(object.object_type());
        let object_name_lowercase = object_name.to_ascii_lowercase();

        match (object.object_type(), operation, success_type) {
            // We should only show toasts for creates initiated by the user, not by the system
            (
                _,
                ObjectOperation::Create {
                    initiated_by: InitiatedBy::User,
                },
                OperationSuccessType::Success,
            ) => {
                let containing_object_name = object.containing_object_name(app);
                Some(
                    t!(
                        "drive_extra.cloud_object.toast.saved_to",
                        object = object_name,
                        destination = containing_object_name
                    )
                    .to_string(),
                )
            }
            // notebooks intentionally do not have an update message, as they are updated
            // as the user types and so toasts would be VERY noisy
            (ObjectType::Notebook, ObjectOperation::Update, OperationSuccessType::Success) => None,
            (_, ObjectOperation::Update, OperationSuccessType::Success) => Some(
                t!(
                    "drive_extra.cloud_object.toast.updated",
                    object = object_name
                )
                .to_string(),
            ),
            (_, ObjectOperation::MoveToFolder, OperationSuccessType::Success)
            | (_, ObjectOperation::MoveToDrive, OperationSuccessType::Success) => {
                let containing_object_name = object.containing_object_name(app);
                Some(
                    t!(
                        "drive_extra.cloud_object.toast.moved_to",
                        object = object_name,
                        destination = containing_object_name
                    )
                    .to_string(),
                )
            }
            (_, ObjectOperation::Trash, OperationSuccessType::Success) => Some(
                t!(
                    "drive_extra.cloud_object.toast.trashed",
                    object = object_name
                )
                .to_string(),
            ),
            (_, ObjectOperation::Untrash, OperationSuccessType::Success) => Some(
                t!(
                    "drive_extra.cloud_object.toast.restored",
                    object = object_name
                )
                .to_string(),
            ),
            (_, ObjectOperation::Leave, OperationSuccessType::Success) => {
                Some(t!("drive_extra.cloud_object.toast.left", object = object_name).to_string())
            }
            (
                _,
                ObjectOperation::Create {
                    initiated_by: InitiatedBy::User,
                },
                OperationSuccessType::Failure,
            ) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_create",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (
                _,
                ObjectOperation::Create {
                    initiated_by: InitiatedBy::User,
                },
                OperationSuccessType::Denied(message),
            ) => Some(message.to_string()),
            (_, ObjectOperation::Update, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_update",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (_, ObjectOperation::MoveToFolder, OperationSuccessType::Failure)
            | (_, ObjectOperation::MoveToDrive, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_move",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (_, ObjectOperation::Trash, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_trash",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (_, ObjectOperation::Untrash, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_restore",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            // We should only show deletion failure toasts for user-initiated deletions.
            (
                _,
                ObjectOperation::Delete {
                    initiated_by: InitiatedBy::User,
                },
                OperationSuccessType::Failure,
            ) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_delete",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (_, ObjectOperation::Leave, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_leave",
                    object = object_name
                )
                .to_string(),
            ),
            (ObjectType::Workflow, ObjectOperation::Update, OperationSuccessType::Rejection) => {
                Some(t!("drive_extra.cloud_object.toast.workflow_conflict").to_string())
            }
            (
                ObjectType::GenericStringObject(GenericStringObjectFormat::Json(
                    JsonObjectType::EnvVarCollection,
                )),
                ObjectOperation::Update,
                OperationSuccessType::Rejection,
            ) => Some(
                t!("drive_extra.cloud_object.toast.environment_variables_conflict").to_string(),
            ),
            (
                ObjectType::GenericStringObject(GenericStringObjectFormat::Json(
                    JsonObjectType::AIFact,
                )),
                ObjectOperation::Update,
                OperationSuccessType::Rejection,
            ) => Some(t!("drive_extra.cloud_object.toast.rule_conflict").to_string()),
            (_, ObjectOperation::TakeEditAccess, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.failed_start_editing",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (_, ObjectOperation::UpdatePermissions, OperationSuccessType::Success) => Some(
                t!(
                    "drive_extra.cloud_object.toast.permissions_updated",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            (_, ObjectOperation::UpdatePermissions, OperationSuccessType::Failure) => Some(
                t!(
                    "drive_extra.cloud_object.toast.permissions_update_failed",
                    object = object_name_lowercase
                )
                .to_string(),
            ),
            _ => None,
        }
    }

    pub fn toast_deletion_confirm_message(
        num_objects: i32,
        operation: &ObjectOperation,
        success_type: &OperationSuccessType,
    ) -> Option<String> {
        let count_objects_message = match num_objects {
            1 => t!("drive_extra.cloud_object.toast.one_object").to_string(),
            n => t!("drive_extra.cloud_object.toast.objects", count = n).to_string(),
        };
        match (operation, success_type) {
            // We should only show deletion failure toasts for user-initiated deletions.
            (
                ObjectOperation::Delete {
                    initiated_by: InitiatedBy::User,
                },
                OperationSuccessType::Success,
            ) => Some(
                t!(
                    "drive_extra.cloud_object.toast.deleted_forever",
                    objects = count_objects_message
                )
                .to_string(),
            ),
            (ObjectOperation::EmptyTrash, OperationSuccessType::Success) => Some(
                t!(
                    "drive_extra.cloud_object.toast.trash_emptied",
                    objects = count_objects_message
                )
                .to_string(),
            ),
            (ObjectOperation::EmptyTrash, OperationSuccessType::Failure) => {
                Some(t!("drive_extra.cloud_object.toast.empty_trash_failed").to_string())
            }
            (ObjectOperation::EmptyTrash, OperationSuccessType::Rejection) => {
                Some(t!("drive_extra.cloud_object.toast.trash_already_empty").to_string())
            }
            _ => None,
        }
    }
}

fn localized_object_name(object_type: ObjectType) -> String {
    match object_type {
        ObjectType::Notebook => t!("drive_extra.cloud_object.object_names.notebook").to_string(),
        ObjectType::Workflow => t!("drive_extra.cloud_object.object_names.workflow").to_string(),
        ObjectType::Folder => t!("drive_extra.cloud_object.object_names.folder").to_string(),
        ObjectType::GenericStringObject(GenericStringObjectFormat::Json(json_type)) => {
            match json_type {
                JsonObjectType::Preference => {
                    t!("drive_extra.cloud_object.object_names.preference").to_string()
                }
                JsonObjectType::EnvVarCollection => {
                    t!("drive_extra.cloud_object.object_names.environment_variables").to_string()
                }
                JsonObjectType::WorkflowEnum => {
                    t!("drive_extra.cloud_object.object_names.workflow_enum").to_string()
                }
                JsonObjectType::AIFact => {
                    t!("drive_extra.cloud_object.object_names.rule").to_string()
                }
                JsonObjectType::MCPServer => {
                    t!("drive_extra.cloud_object.object_names.mcp_server").to_string()
                }
                JsonObjectType::AIExecutionProfile => {
                    t!("drive_extra.cloud_object.object_names.ai_execution_profile").to_string()
                }
                JsonObjectType::TemplatableMCPServer => {
                    t!("drive_extra.cloud_object.object_names.templatable_mcp_server").to_string()
                }
                JsonObjectType::CloudEnvironment => {
                    t!("drive_extra.cloud_object.object_names.cloud_environment").to_string()
                }
                JsonObjectType::ScheduledAmbientAgent => {
                    t!("drive_extra.cloud_object.object_names.scheduled_ambient_agent").to_string()
                }
                JsonObjectType::CloudAgentConfig => {
                    t!("drive_extra.cloud_object.object_names.cloud_agent_config").to_string()
                }
            }
        }
    }
}
