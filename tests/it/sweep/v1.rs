use confluence::v1;
use confluence::{Client, Error, Transport};

use crate::step::{Run, Sent, Step};

pub(crate) trait Operations: Transport + Sized {
    const ALL: [(&'static str, Run<Self>); 130] = [
        ("addContentWatcher", add_content_watcher::<Self>),
        ("addCustomContentPermissions", add_custom_content_permissions::<Self>),
        (
            "addGroupToContentRestrictionByGroupId",
            add_group_to_content_restriction_by_group_id::<Self>,
        ),
        ("addLabelWatcher", add_label_watcher::<Self>),
        ("addLabelsToContent", add_labels_to_content::<Self>),
        ("addLabelsToSpace", add_labels_to_space::<Self>),
        ("addPermissionToSpace", add_permission_to_space::<Self>),
        ("addRestrictions", add_restrictions::<Self>),
        ("addSpaceWatcher", add_space_watcher::<Self>),
        ("addUserToContentRestriction", add_user_to_content_restriction::<Self>),
        ("addUserToGroupByGroupId", add_user_to_group_by_group_id::<Self>),
        ("archivePages", archive_pages::<Self>),
        ("asyncConvertContentBodyRequest", async_convert_content_body_request::<Self>),
        ("asyncConvertContentBodyResponse", async_convert_content_body_response::<Self>),
        ("bulkAsyncConvertContentBodyRequest", bulk_async_convert_content_body_request::<Self>),
        ("bulkAsyncConvertContentBodyResponse", bulk_async_convert_content_body_response::<Self>),
        ("checkContentPermission", check_content_permission::<Self>),
        ("copyPage", copy_page::<Self>),
        ("copyPageHierarchy", copy_page_hierarchy::<Self>),
        ("createAttachment", create_attachment::<Self>),
        ("createAuditRecord", create_audit_record::<Self>),
        ("createContentTemplate", create_content_template::<Self>),
        ("createGroup", create_group::<Self>),
        ("createOrUpdateAttachments", create_or_update_attachments::<Self>),
        ("createPrivateSpace", create_private_space::<Self>),
        ("createRelationship", create_relationship::<Self>),
        ("createSpace", create_space::<Self>),
        ("createUserProperty", create_user_property::<Self>),
        ("deleteContentVersion", delete_content_version::<Self>),
        ("deleteLabelFromSpace", delete_label_from_space::<Self>),
        ("deletePageTree", delete_page_tree::<Self>),
        ("deleteRelationship", delete_relationship::<Self>),
        ("deleteRestrictions", delete_restrictions::<Self>),
        ("deleteSpace", delete_space::<Self>),
        ("deleteUserProperty", delete_user_property::<Self>),
        ("downloadAttatchment", download_attatchment::<Self>),
        ("exportAuditRecords", export_audit_records::<Self>),
        ("findSourcesForTarget", find_sources_for_target::<Self>),
        ("findTargetFromSource", find_target_from_source::<Self>),
        ("getAllLabelContent", get_all_label_content::<Self>),
        (
            "getAndAsyncConvertMacroBodyByMacroId",
            get_and_async_convert_macro_body_by_macro_id::<Self>,
        ),
        ("getAndConvertMacroBodyByMacroId", get_and_convert_macro_body_by_macro_id::<Self>),
        ("getAnonymousUser", get_anonymous_user::<Self>),
        ("getAuditRecords", get_audit_records::<Self>),
        ("getAuditRecordsForTimePeriod", get_audit_records_for_time_period::<Self>),
        ("getAvailableContentStates", get_available_content_states::<Self>),
        ("getBlueprintTemplates", get_blueprint_templates::<Self>),
        ("getBulkUserLookup", get_bulk_user_lookup::<Self>),
        ("getContentDescendants", get_content_descendants::<Self>),
        ("getContentRestrictionStatusForUser", get_content_restriction_status_for_user::<Self>),
        ("getContentState", get_content_state::<Self>),
        ("getContentStateSettings", get_content_state_settings::<Self>),
        ("getContentTemplate", get_content_template::<Self>),
        ("getContentTemplates", get_content_templates::<Self>),
        ("getContentWatchStatus", get_content_watch_status::<Self>),
        ("getContentsWithState", get_contents_with_state::<Self>),
        ("getCurrentUser", get_current_user::<Self>),
        ("getCustomContentStates", get_custom_content_states::<Self>),
        ("getDescendantsOfType", get_descendants_of_type::<Self>),
        ("getGlobalTheme", get_global_theme::<Self>),
        ("getGroupByGroupId", get_group_by_group_id::<Self>),
        ("getGroupMembersByGroupId", get_group_members_by_group_id::<Self>),
        ("getGroupMembershipsForUser", get_group_memberships_for_user::<Self>),
        ("getGroups", get_groups::<Self>),
        (
            "getIndividualGroupRestrictionStatusByGroupId",
            get_individual_group_restriction_status_by_group_id::<Self>,
        ),
        ("getLabelsForSpace", get_labels_for_space::<Self>),
        ("getLookAndFeelSettings", get_look_and_feel_settings::<Self>),
        ("getMacroBodyByMacroId", get_macro_body_by_macro_id::<Self>),
        ("getModules", get_modules::<Self>),
        ("getPrivacyUnsafeUserEmail", get_privacy_unsafe_user_email::<Self>),
        ("getPrivacyUnsafeUserEmailBulk", get_privacy_unsafe_user_email_bulk::<Self>),
        ("getRelationship", get_relationship::<Self>),
        ("getRestrictions", get_restrictions::<Self>),
        ("getRestrictionsByOperation", get_restrictions_by_operation::<Self>),
        ("getRestrictionsForOperation", get_restrictions_for_operation::<Self>),
        ("getRetentionPeriod", get_retention_period::<Self>),
        ("getSpaceContentStates", get_space_content_states::<Self>),
        ("getSpaceSettings", get_space_settings::<Self>),
        ("getSpaceTheme", get_space_theme::<Self>),
        ("getSystemInfo", get_system_information::<Self>),
        ("getTask", get_task::<Self>),
        ("getTasks", get_tasks::<Self>),
        ("getTheme", get_theme::<Self>),
        ("getThemes", get_themes::<Self>),
        ("getUser", get_user::<Self>),
        ("getUserProperties", get_user_properties::<Self>),
        ("getUserProperty", get_user_property::<Self>),
        ("getViewers", get_viewers::<Self>),
        ("getViews", get_views::<Self>),
        ("getWatchersForSpace", get_watchers_for_space::<Self>),
        ("getWatchesForPage", get_watches_for_page::<Self>),
        ("getWatchesForSpace", get_watches_for_space::<Self>),
        ("isWatchingLabel", is_watching_label::<Self>),
        ("isWatchingSpace", is_watching_space::<Self>),
        ("movePage", move_page::<Self>),
        ("publishLegacyDraft", publish_legacy_draft::<Self>),
        ("publishSharedDraft", publish_shared_draft::<Self>),
        ("registerModules", register_modules::<Self>),
        ("removeContentState", remove_content_state::<Self>),
        ("removeContentWatcher", remove_content_watcher::<Self>),
        ("removeGroupById", remove_group_by_id::<Self>),
        ("removeGroupFromContentRestriction", remove_group_from_content_restriction::<Self>),
        ("removeLabelFromContent", remove_label_from_content::<Self>),
        (
            "removeLabelFromContentUsingQueryParameter",
            remove_label_from_content_using_query_parameter::<Self>,
        ),
        ("removeLabelWatcher", remove_label_watcher::<Self>),
        ("removeMemberFromGroupByGroupId", remove_member_from_group_by_group_id::<Self>),
        ("removeModules", remove_modules::<Self>),
        ("removePermission", remove_permission::<Self>),
        ("removeSpaceWatch", remove_space_watch::<Self>),
        ("removeTemplate", remove_template::<Self>),
        ("removeUserFromContentRestriction", remove_user_from_content_restriction::<Self>),
        ("resetLookAndFeelSettings", reset_look_and_feel_settings::<Self>),
        ("resetSpaceTheme", reset_space_theme::<Self>),
        ("restoreContentVersion", restore_content_version::<Self>),
        ("searchByCQL", search_by_cql::<Self>),
        ("searchContentByCQL", search_content_by_cql::<Self>),
        ("searchGroups", search_groups::<Self>),
        ("searchUser", search_user::<Self>),
        ("setContentState", set_content_state::<Self>),
        ("setRetentionPeriod", set_retention_period::<Self>),
        ("setSpaceTheme", set_space_theme::<Self>),
        ("updateAttachmentData", update_attachment_data::<Self>),
        ("updateAttachmentProperties", update_attachment_properties::<Self>),
        ("updateContentTemplate", update_content_template::<Self>),
        ("updateLookAndFeel", update_look_and_feel::<Self>),
        ("updateLookAndFeelSettings", update_look_and_feel_settings::<Self>),
        ("updateRestrictions", update_restrictions::<Self>),
        ("updateSpace", update_space::<Self>),
        ("updateSpaceSettings", update_space_settings::<Self>),
        ("updateUserProperty", update_user_property::<Self>),
    ];
}

impl<Network> Operations for Network where Network: Transport {}

pub(crate) fn add_content_watcher<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addContentWatcher", "the step names this operation");

    let operation = v1::AddContentWatcher { content_id: step.text_required("contentId") };

    let options = v1::add_content_watcher::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "addContentWatcher reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_custom_content_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addCustomContentPermissions", "the step names this operation");

    let operation = v1::AddCustomContentPermissions {
        body: step.body_required(),
        space_key: step.text_required("spaceKey"),
    };

    assert!(step.is_spent(), "addCustomContentPermissions reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn add_group_to_content_restriction_by_group_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "addGroupToContentRestrictionByGroupId",
        "the step names this operation",
    );

    let operation = v1::AddGroupToContentRestrictionByGroupID {
        group_id: step.text_required("groupId"),
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    assert!(
        step.is_spent(),
        "addGroupToContentRestrictionByGroupId reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn add_label_watcher<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addLabelWatcher", "the step names this operation");

    let operation = v1::AddLabelWatcher { label_name: step.text_required("labelName") };

    let options = v1::add_label_watcher::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "addLabelWatcher reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_labels_to_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addLabelsToContent", "the step names this operation");

    let operation =
        v1::AddLabelsToContent { body: step.body_required(), id: step.text_required("id") };

    assert!(step.is_spent(), "addLabelsToContent reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn add_labels_to_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addLabelsToSpace", "the step names this operation");

    let operation = v1::AddLabelsToSpace {
        body: step.body_required(),
        space_key: step.text_required("spaceKey"),
    };

    assert!(step.is_spent(), "addLabelsToSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn add_permission_to_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addPermissionToSpace", "the step names this operation");

    let operation = v1::AddPermissionToSpace {
        body: step.body_required(),
        space_key: step.text_required("spaceKey"),
    };

    assert!(step.is_spent(), "addPermissionToSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn add_restrictions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addRestrictions", "the step names this operation");

    let operation =
        v1::AddRestrictions { body: step.body_required(), id: step.text_required("id") };

    let options = v1::add_restrictions::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "addRestrictions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_space_watcher<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addSpaceWatcher", "the step names this operation");

    let operation = v1::AddSpaceWatcher { space_key: step.text_required("spaceKey") };

    let options = v1::add_space_watcher::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "addSpaceWatcher reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_user_to_content_restriction<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addUserToContentRestriction", "the step names this operation");

    let operation = v1::AddUserToContentRestriction {
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    let options = v1::add_user_to_content_restriction::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "addUserToContentRestriction reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_user_to_group_by_group_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addUserToGroupByGroupId", "the step names this operation");

    let operation = v1::AddUserToGroupByGroupID {
        body: step.body_required(),
        group_id: step.text_required("groupId"),
    };

    assert!(step.is_spent(), "addUserToGroupByGroupId reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn archive_pages<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "archivePages", "the step names this operation");

    let operation = v1::ArchivePages;
    let options = v1::archive_pages::Options { body: step.body() };

    assert!(step.is_spent(), "archivePages reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn async_convert_content_body_request<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "asyncConvertContentBodyRequest",
        "the step names this operation",
    );

    let operation = v1::AsyncConvertContentBodyRequest {
        body: step.body_required(),
        to_value: step.text_required("to"),
    };

    let options = v1::async_convert_content_body_request::Options {
        allow_cache: step.boolean("allowCache"),
        content_id_context: step.text("contentIdContext"),
        embedded_content_render: step.text("embeddedContentRender"),
        expand: step.texts("expand"),
        space_key_context: step.text("spaceKeyContext"),
    };

    assert!(step.is_spent(), "asyncConvertContentBodyRequest reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn async_convert_content_body_response<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "asyncConvertContentBodyResponse",
        "the step names this operation",
    );

    let operation = v1::AsyncConvertContentBodyResponse { id: step.text_required("id") };

    assert!(step.is_spent(), "asyncConvertContentBodyResponse reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn bulk_async_convert_content_body_request<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "bulkAsyncConvertContentBodyRequest",
        "the step names this operation",
    );

    let operation = v1::BulkAsyncConvertContentBodyRequest { body: step.body_required() };

    assert!(step.is_spent(), "bulkAsyncConvertContentBodyRequest reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn bulk_async_convert_content_body_response<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "bulkAsyncConvertContentBodyResponse",
        "the step names this operation",
    );

    let operation = v1::BulkAsyncConvertContentBodyResponse { ids: step.texts_required("ids") };

    assert!(
        step.is_spent(),
        "bulkAsyncConvertContentBodyResponse reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn check_content_permission<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "checkContentPermission", "the step names this operation");

    let operation =
        v1::CheckContentPermission { body: step.body_required(), id: step.text_required("id") };

    assert!(step.is_spent(), "checkContentPermission reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn copy_page<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "copyPage", "the step names this operation");

    let operation = v1::CopyPage { body: step.body_required(), id: step.text_required("id") };
    let options = v1::copy_page::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "copyPage reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn copy_page_hierarchy<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "copyPageHierarchy", "the step names this operation");

    let operation =
        v1::CopyPageHierarchy { body: step.body_required(), id: step.text_required("id") };

    assert!(step.is_spent(), "copyPageHierarchy reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_attachment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createAttachment", "the step names this operation");

    let operation =
        v1::CreateAttachment { body: step.parts_required(), id: step.text_required("id") };

    let options = v1::create_attachment::Options { status: step.text("status") };

    assert!(step.is_spent(), "createAttachment reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_audit_record<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createAuditRecord", "the step names this operation");

    let operation = v1::CreateAuditRecord { body: step.body_required() };

    assert!(step.is_spent(), "createAuditRecord reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_content_template<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createContentTemplate", "the step names this operation");

    let operation = v1::CreateContentTemplate { body: step.body_required() };

    assert!(step.is_spent(), "createContentTemplate reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_group<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createGroup", "the step names this operation");

    let operation = v1::CreateGroup { body: step.body_required() };

    assert!(step.is_spent(), "createGroup reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_or_update_attachments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createOrUpdateAttachments", "the step names this operation");

    let operation =
        v1::CreateOrUpdateAttachments { body: step.parts_required(), id: step.text_required("id") };

    let options = v1::create_or_update_attachments::Options { status: step.text("status") };

    assert!(step.is_spent(), "createOrUpdateAttachments reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_private_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createPrivateSpace", "the step names this operation");

    let operation = v1::CreatePrivateSpace { body: step.body_required() };

    assert!(step.is_spent(), "createPrivateSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_relationship<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createRelationship", "the step names this operation");

    let operation = v1::CreateRelationship {
        relation_name: step.text_required("relationName"),
        source_key: step.text_required("sourceKey"),
        source_type: step.text_required("sourceType"),
        target_key: step.text_required("targetKey"),
        target_type: step.text_required("targetType"),
    };

    let options = v1::create_relationship::Options {
        source_status: step.text("sourceStatus"),
        source_version: step.number("sourceVersion"),
        target_status: step.text("targetStatus"),
        target_version: step.number("targetVersion"),
    };

    assert!(step.is_spent(), "createRelationship reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createSpace", "the step names this operation");

    let operation = v1::CreateSpace { body: step.body_required() };

    assert!(step.is_spent(), "createSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_user_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createUserProperty", "the step names this operation");

    let operation = v1::CreateUserProperty {
        body: step.body_required(),
        key: step.text_required("key"),
        user_id: step.text_required("userId"),
    };

    assert!(step.is_spent(), "createUserProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_content_version<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteContentVersion", "the step names this operation");

    let operation = v1::DeleteContentVersion {
        id: step.text_required("id"),
        version_number: step.number_required("versionNumber"),
    };

    assert!(step.is_spent(), "deleteContentVersion reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_label_from_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteLabelFromSpace", "the step names this operation");

    let operation = v1::DeleteLabelFromSpace {
        name: step.text_required("name"),
        space_key: step.text_required("spaceKey"),
    };

    let options = v1::delete_label_from_space::Options { prefix: step.text("prefix") };

    assert!(step.is_spent(), "deleteLabelFromSpace reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_page_tree<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deletePageTree", "the step names this operation");

    let operation = v1::DeletePageTree { id: step.text_required("id") };

    assert!(step.is_spent(), "deletePageTree reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_relationship<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteRelationship", "the step names this operation");

    let operation = v1::DeleteRelationship {
        relation_name: step.text_required("relationName"),
        source_key: step.text_required("sourceKey"),
        source_type: step.text_required("sourceType"),
        target_key: step.text_required("targetKey"),
        target_type: step.text_required("targetType"),
    };

    let options = v1::delete_relationship::Options {
        source_status: step.text("sourceStatus"),
        source_version: step.number("sourceVersion"),
        target_status: step.text("targetStatus"),
        target_version: step.number("targetVersion"),
    };

    assert!(step.is_spent(), "deleteRelationship reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_restrictions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteRestrictions", "the step names this operation");

    let operation = v1::DeleteRestrictions { id: step.text_required("id") };
    let options = v1::delete_restrictions::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "deleteRestrictions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteSpace", "the step names this operation");

    let operation = v1::DeleteSpace { space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "deleteSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_user_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteUserProperty", "the step names this operation");

    let operation = v1::DeleteUserProperty {
        key: step.text_required("key"),
        user_id: step.text_required("userId"),
    };

    assert!(step.is_spent(), "deleteUserProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn download_attatchment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "downloadAttatchment", "the step names this operation");

    let operation = v1::DownloadAttatchment {
        attachment_id: step.text_required("attachmentId"),
        id: step.text_required("id"),
    };

    let options = v1::download_attatchment::Options {
        status: step.texts("status"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "downloadAttatchment reads every value the step holds");

    step.download(client, &operation, &options)
}

pub(crate) fn export_audit_records<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "exportAuditRecords", "the step names this operation");

    let operation = v1::ExportAuditRecords;

    let options = v1::export_audit_records::Options {
        end_date: step.text("endDate"),
        format: step.text("format"),
        search_string: step.text("searchString"),
        start_date: step.text("startDate"),
    };

    assert!(step.is_spent(), "exportAuditRecords reads every value the step holds");

    step.download(client, &operation, &options)
}

pub(crate) fn find_sources_for_target<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "findSourcesForTarget", "the step names this operation");

    let operation = v1::FindSourcesForTarget {
        relation_name: step.text_required("relationName"),
        source_type: step.text_required("sourceType"),
        target_key: step.text_required("targetKey"),
        target_type: step.text_required("targetType"),
    };

    let options = v1::find_sources_for_target::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        source_status: step.text("sourceStatus"),
        source_version: step.number("sourceVersion"),
        start: step.number("start"),
        target_status: step.text("targetStatus"),
        target_version: step.number("targetVersion"),
    };

    assert!(step.is_spent(), "findSourcesForTarget reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn find_target_from_source<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "findTargetFromSource", "the step names this operation");

    let operation = v1::FindTargetFromSource {
        relation_name: step.text_required("relationName"),
        source_key: step.text_required("sourceKey"),
        source_type: step.text_required("sourceType"),
        target_type: step.text_required("targetType"),
    };

    let options = v1::find_target_from_source::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        source_status: step.text("sourceStatus"),
        source_version: step.number("sourceVersion"),
        start: step.number("start"),
        target_status: step.text("targetStatus"),
        target_version: step.number("targetVersion"),
    };

    assert!(step.is_spent(), "findTargetFromSource reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_all_label_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAllLabelContent", "the step names this operation");

    let operation = v1::GetAllLabelContent { name: step.text_required("name") };

    let options = v1::get_all_label_content::Options {
        kind: step.text("type"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getAllLabelContent reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_and_async_convert_macro_body_by_macro_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getAndAsyncConvertMacroBodyByMacroId",
        "the step names this operation",
    );

    let operation = v1::GetAndAsyncConvertMacroBodyByMacroID {
        id: step.text_required("id"),
        macro_id: step.text_required("macroId"),
        to_value: step.text_required("to"),
        version: step.number_required("version"),
    };

    let options = v1::get_and_async_convert_macro_body_by_macro_id::Options {
        allow_cache: step.boolean("allowCache"),
        embedded_content_render: step.text("embeddedContentRender"),
        expand: step.texts("expand"),
        space_key_context: step.text("spaceKeyContext"),
    };

    assert!(
        step.is_spent(),
        "getAndAsyncConvertMacroBodyByMacroId reads every value the step holds",
    );

    step.send(client, &operation, &options)
}

pub(crate) fn get_and_convert_macro_body_by_macro_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getAndConvertMacroBodyByMacroId",
        "the step names this operation",
    );

    let operation = v1::GetAndConvertMacroBodyByMacroID {
        id: step.text_required("id"),
        macro_id: step.text_required("macroId"),
        to_value: step.text_required("to"),
        version: step.number_required("version"),
    };

    let options = v1::get_and_convert_macro_body_by_macro_id::Options {
        embedded_content_render: step.text("embeddedContentRender"),
        expand: step.texts("expand"),
        space_key_context: step.text("spaceKeyContext"),
    };

    assert!(step.is_spent(), "getAndConvertMacroBodyByMacroId reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_anonymous_user<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAnonymousUser", "the step names this operation");

    let operation = v1::GetAnonymousUser;
    let options = v1::get_anonymous_user::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getAnonymousUser reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_audit_records<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAuditRecords", "the step names this operation");

    let operation = v1::GetAuditRecords;

    let options = v1::get_audit_records::Options {
        end_date: step.text("endDate"),
        limit: step.number("limit"),
        search_string: step.text("searchString"),
        start: step.number("start"),
        start_date: step.text("startDate"),
    };

    assert!(step.is_spent(), "getAuditRecords reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_audit_records_for_time_period<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAuditRecordsForTimePeriod", "the step names this operation");

    let operation = v1::GetAuditRecordsForTimePeriod;

    let options = v1::get_audit_records_for_time_period::Options {
        limit: step.number("limit"),
        number: step.number("number"),
        search_string: step.text("searchString"),
        start: step.number("start"),
        units: step.text("units"),
    };

    assert!(step.is_spent(), "getAuditRecordsForTimePeriod reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_available_content_states<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAvailableContentStates", "the step names this operation");

    let operation = v1::GetAvailableContentStates { id: step.text_required("id") };

    assert!(step.is_spent(), "getAvailableContentStates reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_blueprint_templates<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlueprintTemplates", "the step names this operation");

    let operation = v1::GetBlueprintTemplates;

    let options = v1::get_blueprint_templates::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        space_key: step.text("spaceKey"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getBlueprintTemplates reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_bulk_user_lookup<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBulkUserLookup", "the step names this operation");

    let operation = v1::GetBulkUserLookup { account_id: step.text_required("accountId") };
    let options = v1::get_bulk_user_lookup::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getBulkUserLookup reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_content_descendants<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentDescendants", "the step names this operation");

    let operation = v1::GetContentDescendants { id: step.text_required("id") };
    let options = v1::get_content_descendants::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getContentDescendants reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_content_restriction_status_for_user<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getContentRestrictionStatusForUser",
        "the step names this operation",
    );

    let operation = v1::GetContentRestrictionStatusForUser {
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    let options = v1::get_content_restriction_status_for_user::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "getContentRestrictionStatusForUser reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_content_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentState", "the step names this operation");

    let operation = v1::GetContentState { id: step.text_required("id") };
    let options = v1::get_content_state::Options { status: step.text("status") };

    assert!(step.is_spent(), "getContentState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_content_state_settings<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentStateSettings", "the step names this operation");

    let operation = v1::GetContentStateSettings { space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "getContentStateSettings reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_content_template<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentTemplate", "the step names this operation");

    let operation =
        v1::GetContentTemplate { content_template_id: step.text_required("contentTemplateId") };

    let options = v1::get_content_template::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getContentTemplate reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_content_templates<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentTemplates", "the step names this operation");

    let operation = v1::GetContentTemplates;

    let options = v1::get_content_templates::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        space_key: step.text("spaceKey"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getContentTemplates reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_content_watch_status<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentWatchStatus", "the step names this operation");

    let operation = v1::GetContentWatchStatus { content_id: step.text_required("contentId") };

    let options = v1::get_content_watch_status::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "getContentWatchStatus reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_contents_with_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getContentsWithState", "the step names this operation");

    let operation = v1::GetContentsWithState {
        space_key: step.text_required("spaceKey"),
        state_id: step.number_required("state-id"),
    };

    let options = v1::get_contents_with_state::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getContentsWithState reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_current_user<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCurrentUser", "the step names this operation");

    let operation = v1::GetCurrentUser;
    let options = v1::get_current_user::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getCurrentUser reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_states<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentStates", "the step names this operation");

    let operation = v1::GetCustomContentStates;

    assert!(step.is_spent(), "getCustomContentStates reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_descendants_of_type<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDescendantsOfType", "the step names this operation");

    let operation =
        v1::GetDescendantsOfType { id: step.text_required("id"), kind: step.text_required("type") };

    let options = v1::get_descendants_of_type::Options {
        depth: step.text("depth"),
        expand: step.texts("expand"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getDescendantsOfType reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_global_theme<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getGlobalTheme", "the step names this operation");

    let operation = v1::GetGlobalTheme;

    assert!(step.is_spent(), "getGlobalTheme reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_group_by_group_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getGroupByGroupId", "the step names this operation");

    let operation = v1::GetGroupByGroupID { id: step.text_required("id") };

    assert!(step.is_spent(), "getGroupByGroupId reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_group_members_by_group_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getGroupMembersByGroupId", "the step names this operation");

    let operation = v1::GetGroupMembersByGroupID { group_id: step.text_required("groupId") };

    let options = v1::get_group_members_by_group_id::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        should_return_size_total: step.boolean("shouldReturnTotalSize"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getGroupMembersByGroupId reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_group_memberships_for_user<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getGroupMembershipsForUser", "the step names this operation");

    let operation = v1::GetGroupMembershipsForUser { account_id: step.text_required("accountId") };

    let options = v1::get_group_memberships_for_user::Options {
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getGroupMembershipsForUser reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_groups<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getGroups", "the step names this operation");

    let operation = v1::GetGroups;

    let options = v1::get_groups::Options {
        access_type: step.text("accessType"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getGroups reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_individual_group_restriction_status_by_group_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getIndividualGroupRestrictionStatusByGroupId",
        "the step names this operation",
    );

    let operation = v1::GetIndividualGroupRestrictionStatusByGroupID {
        group_id: step.text_required("groupId"),
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    assert!(
        step.is_spent(),
        "getIndividualGroupRestrictionStatusByGroupId reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn get_labels_for_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getLabelsForSpace", "the step names this operation");

    let operation = v1::GetLabelsForSpace { space_key: step.text_required("spaceKey") };

    let options = v1::get_labels_for_space::Options {
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getLabelsForSpace reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_look_and_feel_settings<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getLookAndFeelSettings", "the step names this operation");

    let operation = v1::GetLookAndFeelSettings;
    let options = v1::get_look_and_feel_settings::Options { space_key: step.text("spaceKey") };

    assert!(step.is_spent(), "getLookAndFeelSettings reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_macro_body_by_macro_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getMacroBodyByMacroId", "the step names this operation");

    let operation = v1::GetMacroBodyByMacroID {
        id: step.text_required("id"),
        macro_id: step.text_required("macroId"),
        version: step.number_required("version"),
    };

    assert!(step.is_spent(), "getMacroBodyByMacroId reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_modules<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getModules", "the step names this operation");

    let operation = v1::GetModules;

    assert!(step.is_spent(), "getModules reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_privacy_unsafe_user_email<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPrivacyUnsafeUserEmail", "the step names this operation");

    let operation = v1::GetPrivacyUnsafeUserEmail { account_id: step.text_required("accountId") };

    assert!(step.is_spent(), "getPrivacyUnsafeUserEmail reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_privacy_unsafe_user_email_bulk<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPrivacyUnsafeUserEmailBulk", "the step names this operation");

    let operation =
        v1::GetPrivacyUnsafeUserEmailBulk { account_id: step.texts_required("accountId") };

    assert!(step.is_spent(), "getPrivacyUnsafeUserEmailBulk reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_relationship<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getRelationship", "the step names this operation");

    let operation = v1::GetRelationship {
        relation_name: step.text_required("relationName"),
        source_key: step.text_required("sourceKey"),
        source_type: step.text_required("sourceType"),
        target_key: step.text_required("targetKey"),
        target_type: step.text_required("targetType"),
    };

    let options = v1::get_relationship::Options {
        expand: step.texts("expand"),
        source_status: step.text("sourceStatus"),
        source_version: step.number("sourceVersion"),
        target_status: step.text("targetStatus"),
        target_version: step.number("targetVersion"),
    };

    assert!(step.is_spent(), "getRelationship reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_restrictions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getRestrictions", "the step names this operation");

    let operation = v1::GetRestrictions { id: step.text_required("id") };

    let options = v1::get_restrictions::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getRestrictions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_restrictions_by_operation<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getRestrictionsByOperation", "the step names this operation");

    let operation = v1::GetRestrictionsByOperation { id: step.text_required("id") };
    let options = v1::get_restrictions_by_operation::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getRestrictionsByOperation reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_restrictions_for_operation<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getRestrictionsForOperation", "the step names this operation");

    let operation = v1::GetRestrictionsForOperation {
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    let options = v1::get_restrictions_for_operation::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getRestrictionsForOperation reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_retention_period<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getRetentionPeriod", "the step names this operation");

    let operation = v1::GetRetentionPeriod;

    assert!(step.is_spent(), "getRetentionPeriod reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_content_states<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceContentStates", "the step names this operation");

    let operation = v1::GetSpaceContentStates { space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "getSpaceContentStates reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_settings<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceSettings", "the step names this operation");

    let operation = v1::GetSpaceSettings { space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "getSpaceSettings reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_theme<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceTheme", "the step names this operation");

    let operation = v1::GetSpaceTheme { space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "getSpaceTheme reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_system_information<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSystemInfo", "the step names this operation");

    let operation = v1::GetSystemInfo;

    assert!(step.is_spent(), "getSystemInfo reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_task<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getTask", "the step names this operation");

    let operation = v1::GetTask { id: step.text_required("id") };

    assert!(step.is_spent(), "getTask reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_tasks<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getTasks", "the step names this operation");

    let operation = v1::GetTasks;

    let options = v1::get_tasks::Options {
        key: step.text("key"),
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getTasks reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_theme<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getTheme", "the step names this operation");

    let operation = v1::GetTheme { theme_key: step.text_required("themeKey") };

    assert!(step.is_spent(), "getTheme reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_themes<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getThemes", "the step names this operation");

    let operation = v1::GetThemes;

    let options =
        v1::get_themes::Options { limit: step.number("limit"), start: step.number("start") };

    assert!(step.is_spent(), "getThemes reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_user<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getUser", "the step names this operation");

    let operation = v1::GetUser { account_id: step.text_required("accountId") };
    let options = v1::get_user::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "getUser reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_user_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getUserProperties", "the step names this operation");

    let operation = v1::GetUserProperties { user_id: step.text_required("userId") };

    let options = v1::get_user_properties::Options {
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getUserProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_user_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getUserProperty", "the step names this operation");

    let operation = v1::GetUserProperty {
        key: step.text_required("key"),
        user_id: step.text_required("userId"),
    };

    assert!(step.is_spent(), "getUserProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_viewers<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getViewers", "the step names this operation");

    let operation = v1::GetViewers { content_id: step.text_required("contentId") };
    let options = v1::get_viewers::Options { from_date: step.text("fromDate") };

    assert!(step.is_spent(), "getViewers reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_views<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getViews", "the step names this operation");

    let operation = v1::GetViews { content_id: step.text_required("contentId") };
    let options = v1::get_views::Options { from_date: step.text("fromDate") };

    assert!(step.is_spent(), "getViews reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_watchers_for_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWatchersForSpace", "the step names this operation");

    let operation = v1::GetWatchersForSpace { space_key: step.text_required("spaceKey") };

    let options = v1::get_watchers_for_space::Options {
        limit: step.text("limit"),
        start: step.text("start"),
    };

    assert!(step.is_spent(), "getWatchersForSpace reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_watches_for_page<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWatchesForPage", "the step names this operation");

    let operation = v1::GetWatchesForPage { id: step.text_required("id") };

    let options = v1::get_watches_for_page::Options {
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getWatchesForPage reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_watches_for_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWatchesForSpace", "the step names this operation");

    let operation = v1::GetWatchesForSpace { id: step.text_required("id") };

    let options = v1::get_watches_for_space::Options {
        limit: step.number("limit"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "getWatchesForSpace reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn is_watching_label<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "isWatchingLabel", "the step names this operation");

    let operation = v1::IsWatchingLabel { label_name: step.text_required("labelName") };

    let options = v1::is_watching_label::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "isWatchingLabel reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn is_watching_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "isWatchingSpace", "the step names this operation");

    let operation = v1::IsWatchingSpace { space_key: step.text_required("spaceKey") };

    let options = v1::is_watching_space::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "isWatchingSpace reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn move_page<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "movePage", "the step names this operation");

    let operation = v1::MovePage {
        page_id: step.text_required("pageId"),
        position: step.text_required("position"),
        target_id: step.text_required("targetId"),
    };

    assert!(step.is_spent(), "movePage reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn publish_legacy_draft<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "publishLegacyDraft", "the step names this operation");

    let operation = v1::PublishLegacyDraft {
        body: step.body_required(),
        draft_id: step.text_required("draftId"),
    };

    let options = v1::publish_legacy_draft::Options {
        expand: step.texts("expand"),
        status: step.text("status"),
    };

    assert!(step.is_spent(), "publishLegacyDraft reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn publish_shared_draft<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "publishSharedDraft", "the step names this operation");

    let operation = v1::PublishSharedDraft {
        body: step.body_required(),
        draft_id: step.text_required("draftId"),
    };

    let options = v1::publish_shared_draft::Options {
        expand: step.texts("expand"),
        status: step.text("status"),
    };

    assert!(step.is_spent(), "publishSharedDraft reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn register_modules<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "registerModules", "the step names this operation");

    let operation = v1::RegisterModules { body: step.body_required() };

    assert!(step.is_spent(), "registerModules reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_content_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeContentState", "the step names this operation");

    let operation = v1::RemoveContentState { id: step.text_required("id") };
    let options = v1::remove_content_state::Options { status: step.text("status") };

    assert!(step.is_spent(), "removeContentState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn remove_content_watcher<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeContentWatcher", "the step names this operation");

    let operation = v1::RemoveContentWatcher { content_id: step.text_required("contentId") };

    let options = v1::remove_content_watcher::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "removeContentWatcher reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn remove_group_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeGroupById", "the step names this operation");

    let operation = v1::RemoveGroupByID { id: step.text_required("id") };

    assert!(step.is_spent(), "removeGroupById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_group_from_content_restriction<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "removeGroupFromContentRestriction",
        "the step names this operation",
    );

    let operation = v1::RemoveGroupFromContentRestriction {
        group_id: step.text_required("groupId"),
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    assert!(step.is_spent(), "removeGroupFromContentRestriction reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_label_from_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeLabelFromContent", "the step names this operation");

    let operation = v1::RemoveLabelFromContent {
        id: step.text_required("id"),
        label: step.text_required("label"),
    };

    assert!(step.is_spent(), "removeLabelFromContent reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_label_from_content_using_query_parameter<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "removeLabelFromContentUsingQueryParameter",
        "the step names this operation",
    );

    let operation = v1::RemoveLabelFromContentUsingQueryParameter {
        id: step.text_required("id"),
        name: step.text_required("name"),
    };

    assert!(
        step.is_spent(),
        "removeLabelFromContentUsingQueryParameter reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn remove_label_watcher<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeLabelWatcher", "the step names this operation");

    let operation = v1::RemoveLabelWatcher { label_name: step.text_required("labelName") };

    let options = v1::remove_label_watcher::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "removeLabelWatcher reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn remove_member_from_group_by_group_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "removeMemberFromGroupByGroupId",
        "the step names this operation",
    );

    let operation = v1::RemoveMemberFromGroupByGroupID {
        account_id: step.text_required("accountId"),
        group_id: step.text_required("groupId"),
    };

    let options = v1::remove_member_from_group_by_group_id::Options {
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "removeMemberFromGroupByGroupId reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn remove_modules<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeModules", "the step names this operation");

    let operation = v1::RemoveModules { module_key: step.texts_required("moduleKey") };

    assert!(step.is_spent(), "removeModules reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_permission<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removePermission", "the step names this operation");

    let operation = v1::RemovePermission {
        id: step.number_required("id"),
        space_key: step.text_required("spaceKey"),
    };

    assert!(step.is_spent(), "removePermission reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_space_watch<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeSpaceWatch", "the step names this operation");

    let operation = v1::RemoveSpaceWatch { space_key: step.text_required("spaceKey") };

    let options = v1::remove_space_watch::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "removeSpaceWatch reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn remove_template<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "removeTemplate", "the step names this operation");

    let operation =
        v1::RemoveTemplate { content_template_id: step.text_required("contentTemplateId") };

    assert!(step.is_spent(), "removeTemplate reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn remove_user_from_content_restriction<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "removeUserFromContentRestriction",
        "the step names this operation",
    );

    let operation = v1::RemoveUserFromContentRestriction {
        id: step.text_required("id"),
        operation_key: step.text_required("operationKey"),
    };

    let options = v1::remove_user_from_content_restriction::Options {
        account_id: step.text("accountId"),
        key: step.text("key"),
        username: step.text("username"),
    };

    assert!(step.is_spent(), "removeUserFromContentRestriction reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn reset_look_and_feel_settings<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "resetLookAndFeelSettings", "the step names this operation");

    let operation = v1::ResetLookAndFeelSettings;
    let options = v1::reset_look_and_feel_settings::Options { space_key: step.text("spaceKey") };

    assert!(step.is_spent(), "resetLookAndFeelSettings reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn reset_space_theme<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "resetSpaceTheme", "the step names this operation");

    let operation = v1::ResetSpaceTheme { space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "resetSpaceTheme reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn restore_content_version<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "restoreContentVersion", "the step names this operation");

    let operation =
        v1::RestoreContentVersion { body: step.body_required(), id: step.text_required("id") };

    let options = v1::restore_content_version::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "restoreContentVersion reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn search_by_cql<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchByCQL", "the step names this operation");

    let operation = v1::SearchByCQL { cql: step.text_required("cql") };

    let options = v1::search_by_cql::Options {
        cache_buster: step.number("_"),
        cqlcontext: step.text("cqlcontext"),
        cursor: step.text("cursor"),
        excerpt: step.text("excerpt"),
        exclude_spaces_current: step.boolean("excludeCurrentSpaces"),
        expand: step.texts("expand"),
        include_archived_spaces: step.boolean("includeArchivedSpaces"),
        limit: step.number("limit"),
        next: step.boolean("next"),
        previous: step.boolean("prev"),
        site_permission_type_filter: step.text("sitePermissionTypeFilter"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "searchByCQL reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn search_content_by_cql<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchContentByCQL", "the step names this operation");

    let operation = v1::SearchContentByCQL { cql: step.text_required("cql") };

    let options = v1::search_content_by_cql::Options {
        cqlcontext: step.text("cqlcontext"),
        cursor: step.text("cursor"),
        expand: step.texts("expand"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "searchContentByCQL reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn search_groups<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchGroups", "the step names this operation");

    let operation = v1::SearchGroups { query: step.text_required("query") };

    let options = v1::search_groups::Options {
        limit: step.number("limit"),
        should_return_size_total: step.boolean("shouldReturnTotalSize"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "searchGroups reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn search_user<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchUser", "the step names this operation");

    let operation = v1::SearchUser { cql: step.text_required("cql") };

    let options = v1::search_user::Options {
        expand: step.texts("expand"),
        limit: step.number("limit"),
        site_permission_type_filter: step.text("sitePermissionTypeFilter"),
        start: step.number("start"),
    };

    assert!(step.is_spent(), "searchUser reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn set_content_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setContentState", "the step names this operation");

    let operation = v1::SetContentState {
        body: step.body_required(),
        id: step.text_required("id"),
        status: step.text_required("status"),
    };

    assert!(step.is_spent(), "setContentState reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn set_retention_period<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setRetentionPeriod", "the step names this operation");

    let operation = v1::SetRetentionPeriod { body: step.body_required() };

    assert!(step.is_spent(), "setRetentionPeriod reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn set_space_theme<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setSpaceTheme", "the step names this operation");

    let operation =
        v1::SetSpaceTheme { body: step.body_required(), space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "setSpaceTheme reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_attachment_data<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateAttachmentData", "the step names this operation");

    let operation = v1::UpdateAttachmentData {
        attachment_id: step.text_required("attachmentId"),
        body: step.parts_required(),
        id: step.text_required("id"),
    };

    assert!(step.is_spent(), "updateAttachmentData reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_attachment_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateAttachmentProperties", "the step names this operation");

    let operation = v1::UpdateAttachmentProperties {
        attachment_id: step.text_required("attachmentId"),
        body: step.body_required(),
        id: step.text_required("id"),
    };

    assert!(step.is_spent(), "updateAttachmentProperties reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_content_template<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateContentTemplate", "the step names this operation");

    let operation = v1::UpdateContentTemplate { body: step.body_required() };

    assert!(step.is_spent(), "updateContentTemplate reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_look_and_feel<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateLookAndFeel", "the step names this operation");

    let operation = v1::UpdateLookAndFeel { body: step.body_required() };

    assert!(step.is_spent(), "updateLookAndFeel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_look_and_feel_settings<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateLookAndFeelSettings", "the step names this operation");

    let operation = v1::UpdateLookAndFeelSettings { body: step.body_required() };
    let options = v1::update_look_and_feel_settings::Options { space_key: step.text("spaceKey") };

    assert!(step.is_spent(), "updateLookAndFeelSettings reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_restrictions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateRestrictions", "the step names this operation");

    let operation =
        v1::UpdateRestrictions { body: step.body_required(), id: step.text_required("id") };

    let options = v1::update_restrictions::Options { expand: step.texts("expand") };

    assert!(step.is_spent(), "updateRestrictions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateSpace", "the step names this operation");

    let operation =
        v1::UpdateSpace { body: step.body_required(), space_key: step.text_required("spaceKey") };

    assert!(step.is_spent(), "updateSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_space_settings<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateSpaceSettings", "the step names this operation");

    let operation = v1::UpdateSpaceSettings {
        body: step.body_required(),
        space_key: step.text_required("spaceKey"),
    };

    assert!(step.is_spent(), "updateSpaceSettings reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_user_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateUserProperty", "the step names this operation");

    let operation = v1::UpdateUserProperty {
        body: step.body_required(),
        key: step.text_required("key"),
        user_id: step.text_required("userId"),
    };

    assert!(step.is_spent(), "updateUserProperty reads every value the step holds");

    step.send_bare(client, &operation)
}
