use confluence::v2;
use confluence::{Client, Error, Transport};

use crate::step::{Run, Sent, Step};

pub(crate) trait Operations: Transport + Sized {
    const ALL: [(&'static str, Run<Self>); 252] = [
        ("addBlogPostPermissions", add_blog_post_permissions::<Self>),
        ("addDatabasePermissions", add_database_permissions::<Self>),
        ("addFolderPermissions", add_folder_permissions::<Self>),
        ("addPagePermissions", add_page_permissions::<Self>),
        ("addWhiteboardPermissions", add_whiteboard_permissions::<Self>),
        ("bulkAssignSpacePermissionRoles", bulk_assign_space_permission_roles::<Self>),
        ("bulkRemoveSpacePermissionAccess", bulk_remove_space_permission_access::<Self>),
        ("checkAccessByEmail", check_access_by_email::<Self>),
        ("convertContentIdsToContentTypes", convert_content_ids_to_content_types::<Self>),
        ("createAttachmentProperty", create_attachment_property::<Self>),
        ("createBlogPost", create_blog_post::<Self>),
        ("createBlogpostProperty", create_blogpost_property::<Self>),
        ("createBulkUserLookup", create_bulk_user_lookup::<Self>),
        ("createCommentProperty", create_comment_property::<Self>),
        ("createCustomContent", create_custom_content::<Self>),
        ("createCustomContentProperty", create_custom_content_property::<Self>),
        ("createDatabase", create_database::<Self>),
        ("createDatabaseProperty", create_database_property::<Self>),
        ("createFolder", create_folder::<Self>),
        ("createFolderProperty", create_folder_property::<Self>),
        ("createFooterComment", create_footer_comment::<Self>),
        ("createInlineComment", create_inline_comment::<Self>),
        ("createPage", create_page::<Self>),
        ("createPageProperty", create_page_property::<Self>),
        ("createSmartLink", create_smart_link::<Self>),
        ("createSmartLinkProperty", create_smart_link_property::<Self>),
        ("createSpace", create_space::<Self>),
        ("createSpaceProperty", create_space_property::<Self>),
        ("createSpaceRole", create_space_role::<Self>),
        ("createWhiteboard", create_whiteboard::<Self>),
        ("createWhiteboardProperty", create_whiteboard_property::<Self>),
        ("deleteAttachment", delete_attachment::<Self>),
        ("deleteAttachmentPropertyById", delete_attachment_property_by_id::<Self>),
        ("deleteBlogPost", delete_blog_post::<Self>),
        ("deleteBlogPostPermissions", delete_blog_post_permissions::<Self>),
        ("deleteBlogpostPropertyById", delete_blogpost_property_by_id::<Self>),
        ("deleteCommentPropertyById", delete_comment_property_by_id::<Self>),
        ("deleteCustomContent", delete_custom_content::<Self>),
        ("deleteCustomContentPropertyById", delete_custom_content_property_by_id::<Self>),
        ("deleteDatabase", delete_database::<Self>),
        ("deleteDatabasePermissions", delete_database_permissions::<Self>),
        ("deleteDatabasePropertyById", delete_database_property_by_id::<Self>),
        ("deleteFolder", delete_folder::<Self>),
        ("deleteFolderPermissions", delete_folder_permissions::<Self>),
        ("deleteFolderPropertyById", delete_folder_property_by_id::<Self>),
        ("deleteFooterComment", delete_footer_comment::<Self>),
        ("deleteForgeAppProperty", delete_forge_app_property::<Self>),
        ("deleteInlineComment", delete_inline_comment::<Self>),
        ("deletePage", delete_page::<Self>),
        ("deletePagePermissions", delete_page_permissions::<Self>),
        ("deletePagePropertyById", delete_page_property_by_id::<Self>),
        ("deleteSmartLink", delete_smart_link::<Self>),
        ("deleteSmartLinkPropertyById", delete_smart_link_property_by_id::<Self>),
        (
            "deleteSpaceDefaultClassificationLevel",
            delete_space_default_classification_level::<Self>,
        ),
        ("deleteSpacePropertyById", delete_space_property_by_id::<Self>),
        ("deleteSpaceRole", delete_space_role::<Self>),
        ("deleteWhiteboard", delete_whiteboard::<Self>),
        ("deleteWhiteboardPermissions", delete_whiteboard_permissions::<Self>),
        ("deleteWhiteboardPropertyById", delete_whiteboard_property_by_id::<Self>),
        ("disableAdminKey", disable_admin_key::<Self>),
        ("enableAdminKey", enable_admin_key::<Self>),
        ("generateSpacePermissionCombinations", generate_space_permission_combinations::<Self>),
        ("getAdminKey", get_admin_key::<Self>),
        ("getAttachmentById", get_attachment_by_id::<Self>),
        ("getAttachmentComments", get_attachment_comments::<Self>),
        ("getAttachmentContentProperties", get_attachment_content_properties::<Self>),
        ("getAttachmentContentPropertiesById", get_attachment_content_properties_by_id::<Self>),
        ("getAttachmentLabels", get_attachment_labels::<Self>),
        ("getAttachmentOperations", get_attachment_operations::<Self>),
        ("getAttachmentThumbnailById", get_attachment_thumbnail_by_id::<Self>),
        ("getAttachmentVersionDetails", get_attachment_version_details::<Self>),
        ("getAttachmentVersions", get_attachment_versions::<Self>),
        ("getAttachments", get_attachments::<Self>),
        ("getAvailableSpacePermissions", get_available_space_permissions::<Self>),
        ("getAvailableSpaceRoles", get_available_space_roles::<Self>),
        ("getBlogPostById", get_blog_post_by_id::<Self>),
        ("getBlogPostClassificationLevel", get_blog_post_classification_level::<Self>),
        ("getBlogPostFooterComments", get_blog_post_footer_comments::<Self>),
        ("getBlogPostInlineComments", get_blog_post_inline_comments::<Self>),
        ("getBlogPostLabels", get_blog_post_labels::<Self>),
        ("getBlogPostLikeCount", get_blog_post_like_count::<Self>),
        ("getBlogPostLikeUsers", get_blog_post_like_users::<Self>),
        ("getBlogPostOperations", get_blog_post_operations::<Self>),
        ("getBlogPostPermissions", get_blog_post_permissions::<Self>),
        ("getBlogPostRestrictionState", get_blog_post_restriction_state::<Self>),
        ("getBlogPostVersionDetails", get_blog_post_version_details::<Self>),
        ("getBlogPostVersions", get_blog_post_versions::<Self>),
        ("getBlogPosts", get_blog_posts::<Self>),
        ("getBlogPostsInSpace", get_blog_posts_in_space::<Self>),
        ("getBlogpostAttachments", get_blogpost_attachments::<Self>),
        ("getBlogpostContentProperties", get_blogpost_content_properties::<Self>),
        ("getBlogpostContentPropertiesById", get_blogpost_content_properties_by_id::<Self>),
        ("getChildCustomContent", get_child_custom_content::<Self>),
        ("getChildPages", get_child_pages::<Self>),
        ("getClassificationLevels", get_classification_levels::<Self>),
        ("getCommentContentProperties", get_comment_content_properties::<Self>),
        ("getCommentContentPropertiesById", get_comment_content_properties_by_id::<Self>),
        ("getCustomContentAttachments", get_custom_content_attachments::<Self>),
        ("getCustomContentById", get_custom_content_by_id::<Self>),
        ("getCustomContentByType", get_custom_content_by_type::<Self>),
        ("getCustomContentByTypeInBlogPost", get_custom_content_by_type_in_blog_post::<Self>),
        ("getCustomContentByTypeInPage", get_custom_content_by_type_in_page::<Self>),
        ("getCustomContentByTypeInSpace", get_custom_content_by_type_in_space::<Self>),
        ("getCustomContentComments", get_custom_content_comments::<Self>),
        ("getCustomContentContentProperties", get_custom_content_content_properties::<Self>),
        (
            "getCustomContentContentPropertiesById",
            get_custom_content_content_properties_by_id::<Self>,
        ),
        ("getCustomContentLabels", get_custom_content_labels::<Self>),
        ("getCustomContentOperations", get_custom_content_operations::<Self>),
        ("getCustomContentVersionDetails", get_custom_content_version_details::<Self>),
        ("getCustomContentVersions", get_custom_content_versions::<Self>),
        ("getDataPolicyMetadata", get_data_policy_metadata::<Self>),
        ("getDataPolicySpaces", get_data_policy_spaces::<Self>),
        ("getDatabaseAncestors", get_database_ancestors::<Self>),
        ("getDatabaseById", get_database_by_id::<Self>),
        ("getDatabaseClassificationLevel", get_database_classification_level::<Self>),
        ("getDatabaseContentProperties", get_database_content_properties::<Self>),
        ("getDatabaseContentPropertiesById", get_database_content_properties_by_id::<Self>),
        ("getDatabaseDescendants", get_database_descendants::<Self>),
        ("getDatabaseDirectChildren", get_database_direct_children::<Self>),
        ("getDatabaseEntries", get_database_entries::<Self>),
        ("getDatabaseEntryById", get_database_entry_by_id::<Self>),
        ("getDatabaseFields", get_database_fields::<Self>),
        ("getDatabaseOperations", get_database_operations::<Self>),
        ("getDatabasePermissions", get_database_permissions::<Self>),
        ("getDatabaseRestrictionState", get_database_restriction_state::<Self>),
        ("getDatabaseViews", get_database_views::<Self>),
        ("getFolderAncestors", get_folder_ancestors::<Self>),
        ("getFolderById", get_folder_by_id::<Self>),
        ("getFolderContentProperties", get_folder_content_properties::<Self>),
        ("getFolderContentPropertiesById", get_folder_content_properties_by_id::<Self>),
        ("getFolderDescendants", get_folder_descendants::<Self>),
        ("getFolderDirectChildren", get_folder_direct_children::<Self>),
        ("getFolderOperations", get_folder_operations::<Self>),
        ("getFolderPermissions", get_folder_permissions::<Self>),
        ("getFolderRestrictionState", get_folder_restriction_state::<Self>),
        ("getFooterCommentById", get_footer_comment_by_id::<Self>),
        ("getFooterCommentChildren", get_footer_comment_children::<Self>),
        ("getFooterCommentOperations", get_footer_comment_operations::<Self>),
        ("getFooterCommentVersionDetails", get_footer_comment_version_details::<Self>),
        ("getFooterCommentVersions", get_footer_comment_versions::<Self>),
        ("getFooterComments", get_footer_comments::<Self>),
        ("getFooterLikeCount", get_footer_like_count::<Self>),
        ("getFooterLikeUsers", get_footer_like_users::<Self>),
        ("getForgeAppProperties", get_forge_app_properties::<Self>),
        ("getForgeAppProperty", get_forge_app_property::<Self>),
        ("getInlineCommentById", get_inline_comment_by_id::<Self>),
        ("getInlineCommentChildren", get_inline_comment_children::<Self>),
        ("getInlineCommentOperations", get_inline_comment_operations::<Self>),
        ("getInlineCommentVersionDetails", get_inline_comment_version_details::<Self>),
        ("getInlineCommentVersions", get_inline_comment_versions::<Self>),
        ("getInlineComments", get_inline_comments::<Self>),
        ("getInlineLikeCount", get_inline_like_count::<Self>),
        ("getInlineLikeUsers", get_inline_like_users::<Self>),
        ("getLabelAttachments", get_label_attachments::<Self>),
        ("getLabelBlogPosts", get_label_blog_posts::<Self>),
        ("getLabelPages", get_label_pages::<Self>),
        ("getLabels", get_labels::<Self>),
        ("getPageAncestors", get_page_ancestors::<Self>),
        ("getPageAttachments", get_page_attachments::<Self>),
        ("getPageById", get_page_by_id::<Self>),
        ("getPageClassificationLevel", get_page_classification_level::<Self>),
        ("getPageContentProperties", get_page_content_properties::<Self>),
        ("getPageContentPropertiesById", get_page_content_properties_by_id::<Self>),
        ("getPageDescendants", get_page_descendants::<Self>),
        ("getPageDirectChildren", get_page_direct_children::<Self>),
        ("getPageFooterComments", get_page_footer_comments::<Self>),
        ("getPageInlineComments", get_page_inline_comments::<Self>),
        ("getPageLabels", get_page_labels::<Self>),
        ("getPageLikeCount", get_page_like_count::<Self>),
        ("getPageLikeUsers", get_page_like_users::<Self>),
        ("getPageOperations", get_page_operations::<Self>),
        ("getPagePermissions", get_page_permissions::<Self>),
        ("getPageRestrictionState", get_page_restriction_state::<Self>),
        ("getPageVersionDetails", get_page_version_details::<Self>),
        ("getPageVersions", get_page_versions::<Self>),
        ("getPages", get_pages::<Self>),
        ("getPagesInSpace", get_pages_in_space::<Self>),
        ("getSmartLinkAncestors", get_smart_link_ancestors::<Self>),
        ("getSmartLinkById", get_smart_link_by_id::<Self>),
        ("getSmartLinkContentProperties", get_smart_link_content_properties::<Self>),
        ("getSmartLinkContentPropertiesById", get_smart_link_content_properties_by_id::<Self>),
        ("getSmartLinkDescendants", get_smart_link_descendants::<Self>),
        ("getSmartLinkDirectChildren", get_smart_link_direct_children::<Self>),
        ("getSmartLinkOperations", get_smart_link_operations::<Self>),
        ("getSpaceById", get_space_by_id::<Self>),
        ("getSpaceContentLabels", get_space_content_labels::<Self>),
        ("getSpaceDefaultClassificationLevel", get_space_default_classification_level::<Self>),
        ("getSpaceLabels", get_space_labels::<Self>),
        ("getSpaceOperations", get_space_operations::<Self>),
        (
            "getSpacePermissionTransitionTaskStatus",
            get_space_permission_transition_task_status::<Self>,
        ),
        ("getSpacePermissionsAssignments", get_space_permissions_assignments::<Self>),
        ("getSpaceProperties", get_space_properties::<Self>),
        ("getSpacePropertyById", get_space_property_by_id::<Self>),
        ("getSpaceRoleAssignments", get_space_role_assignments::<Self>),
        ("getSpaceRoleMode", get_space_role_mode::<Self>),
        ("getSpaceRolesById", get_space_roles_by_id::<Self>),
        ("getSpaces", get_spaces::<Self>),
        ("getTaskById", get_task_by_id::<Self>),
        ("getTasks", get_tasks::<Self>),
        ("getWhiteboardAncestors", get_whiteboard_ancestors::<Self>),
        ("getWhiteboardById", get_whiteboard_by_id::<Self>),
        ("getWhiteboardClassificationLevel", get_whiteboard_classification_level::<Self>),
        ("getWhiteboardContentProperties", get_whiteboard_content_properties::<Self>),
        ("getWhiteboardContentPropertiesById", get_whiteboard_content_properties_by_id::<Self>),
        ("getWhiteboardDescendants", get_whiteboard_descendants::<Self>),
        ("getWhiteboardDirectChildren", get_whiteboard_direct_children::<Self>),
        ("getWhiteboardOperations", get_whiteboard_operations::<Self>),
        ("getWhiteboardPermissions", get_whiteboard_permissions::<Self>),
        ("getWhiteboardRestrictionState", get_whiteboard_restriction_state::<Self>),
        ("inviteByEmail", invite_by_email::<Self>),
        ("listSpacePermissionCombinations", list_space_permission_combinations::<Self>),
        ("postBlogPostClassificationLevel", post_blog_post_classification_level::<Self>),
        ("postDatabaseClassificationLevel", post_database_classification_level::<Self>),
        ("postPageClassificationLevel", post_page_classification_level::<Self>),
        ("postRedactBlog", post_redact_blog::<Self>),
        ("postRedactPage", post_redact_page::<Self>),
        ("postWhiteboardClassificationLevel", post_whiteboard_classification_level::<Self>),
        ("putBlogPostClassificationLevel", put_blog_post_classification_level::<Self>),
        ("putDatabaseClassificationLevel", put_database_classification_level::<Self>),
        ("putForgeAppProperty", put_forge_app_property::<Self>),
        ("putPageClassificationLevel", put_page_classification_level::<Self>),
        ("putSpaceDefaultClassificationLevel", put_space_default_classification_level::<Self>),
        ("putWhiteboardClassificationLevel", put_whiteboard_classification_level::<Self>),
        ("setBlogPostPermissions", set_blog_post_permissions::<Self>),
        ("setBlogPostRestrictionState", set_blog_post_restriction_state::<Self>),
        ("setDatabasePermissions", set_database_permissions::<Self>),
        ("setDatabaseRestrictionState", set_database_restriction_state::<Self>),
        ("setFolderPermissions", set_folder_permissions::<Self>),
        ("setFolderRestrictionState", set_folder_restriction_state::<Self>),
        ("setPagePermissions", set_page_permissions::<Self>),
        ("setPageRestrictionState", set_page_restriction_state::<Self>),
        ("setSpaceRoleAssignments", set_space_role_assignments::<Self>),
        ("setWhiteboardPermissions", set_whiteboard_permissions::<Self>),
        ("setWhiteboardRestrictionState", set_whiteboard_restriction_state::<Self>),
        ("updateAttachmentPropertyById", update_attachment_property_by_id::<Self>),
        ("updateBlogPost", update_blog_post::<Self>),
        ("updateBlogpostPropertyById", update_blogpost_property_by_id::<Self>),
        ("updateCommentPropertyById", update_comment_property_by_id::<Self>),
        ("updateCustomContent", update_custom_content::<Self>),
        ("updateCustomContentPropertyById", update_custom_content_property_by_id::<Self>),
        ("updateDatabasePropertyById", update_database_property_by_id::<Self>),
        ("updateFolderPropertyById", update_folder_property_by_id::<Self>),
        ("updateFooterComment", update_footer_comment::<Self>),
        ("updateInlineComment", update_inline_comment::<Self>),
        ("updatePage", update_page::<Self>),
        ("updatePagePropertyById", update_page_property_by_id::<Self>),
        ("updatePageTitle", update_page_title::<Self>),
        ("updateSmartLinkPropertyById", update_smart_link_property_by_id::<Self>),
        ("updateSpacePropertyById", update_space_property_by_id::<Self>),
        ("updateSpaceRole", update_space_role::<Self>),
        ("updateTask", update_task::<Self>),
        ("updateWhiteboardPropertyById", update_whiteboard_property_by_id::<Self>),
    ];
}

impl<Network> Operations for Network where Network: Transport {}

pub(crate) fn add_blog_post_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addBlogPostPermissions", "the step names this operation");

    let operation =
        v2::AddBlogPostPermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::add_blog_post_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "addBlogPostPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_database_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addDatabasePermissions", "the step names this operation");

    let operation =
        v2::AddDatabasePermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::add_database_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "addDatabasePermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_folder_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addFolderPermissions", "the step names this operation");

    let operation =
        v2::AddFolderPermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::add_folder_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "addFolderPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_page_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addPagePermissions", "the step names this operation");

    let operation =
        v2::AddPagePermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::add_page_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "addPagePermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn add_whiteboard_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "addWhiteboardPermissions", "the step names this operation");

    let operation =
        v2::AddWhiteboardPermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::add_whiteboard_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "addWhiteboardPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn bulk_assign_space_permission_roles<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "bulkAssignSpacePermissionRoles",
        "the step names this operation",
    );

    let operation = v2::BulkAssignSpacePermissionRoles { body: step.body_required() };

    assert!(step.is_spent(), "bulkAssignSpacePermissionRoles reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn bulk_remove_space_permission_access<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "bulkRemoveSpacePermissionAccess",
        "the step names this operation",
    );

    let operation = v2::BulkRemoveSpacePermissionAccess { body: step.body_required() };

    assert!(step.is_spent(), "bulkRemoveSpacePermissionAccess reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn check_access_by_email<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "checkAccessByEmail", "the step names this operation");

    let operation = v2::CheckAccessByEmail { body: step.body_required() };

    assert!(step.is_spent(), "checkAccessByEmail reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn convert_content_ids_to_content_types<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "convertContentIdsToContentTypes",
        "the step names this operation",
    );

    let operation = v2::ConvertContentIDsToContentTypes { body: step.body_required() };

    assert!(step.is_spent(), "convertContentIdsToContentTypes reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_attachment_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createAttachmentProperty", "the step names this operation");

    let operation = v2::CreateAttachmentProperty {
        attachment_id: step.text_required("attachment-id"),
        body: step.body_required(),
    };

    assert!(step.is_spent(), "createAttachmentProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_blog_post<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createBlogPost", "the step names this operation");

    let operation = v2::CreateBlogPost { body: step.body_required() };
    let options = v2::create_blog_post::Options { private: step.boolean("private") };

    assert!(step.is_spent(), "createBlogPost reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_blogpost_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createBlogpostProperty", "the step names this operation");

    let operation = v2::CreateBlogpostProperty {
        blogpost_id: step.number_required("blogpost-id"),
        body: step.body_required(),
    };

    assert!(step.is_spent(), "createBlogpostProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_bulk_user_lookup<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createBulkUserLookup", "the step names this operation");

    let operation = v2::CreateBulkUserLookup { body: step.body_required() };

    assert!(step.is_spent(), "createBulkUserLookup reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_comment_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createCommentProperty", "the step names this operation");

    let operation = v2::CreateCommentProperty {
        body: step.body_required(),
        comment_id: step.number_required("comment-id"),
    };

    assert!(step.is_spent(), "createCommentProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_custom_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createCustomContent", "the step names this operation");

    let operation = v2::CreateCustomContent { body: step.body_required() };

    assert!(step.is_spent(), "createCustomContent reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_custom_content_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createCustomContentProperty", "the step names this operation");

    let operation = v2::CreateCustomContentProperty {
        body: step.body_required(),
        custom_content_id: step.number_required("custom-content-id"),
    };

    assert!(step.is_spent(), "createCustomContentProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_database<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createDatabase", "the step names this operation");

    let operation = v2::CreateDatabase { body: step.body_required() };
    let options = v2::create_database::Options { private: step.boolean("private") };

    assert!(step.is_spent(), "createDatabase reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_database_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createDatabaseProperty", "the step names this operation");

    let operation =
        v2::CreateDatabaseProperty { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "createDatabaseProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_folder<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createFolder", "the step names this operation");

    let operation = v2::CreateFolder { body: step.body_required() };

    assert!(step.is_spent(), "createFolder reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_folder_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createFolderProperty", "the step names this operation");

    let operation =
        v2::CreateFolderProperty { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "createFolderProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_footer_comment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createFooterComment", "the step names this operation");

    let operation = v2::CreateFooterComment { body: step.body_required() };

    assert!(step.is_spent(), "createFooterComment reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_inline_comment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createInlineComment", "the step names this operation");

    let operation = v2::CreateInlineComment { body: step.body_required() };

    assert!(step.is_spent(), "createInlineComment reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_page<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createPage", "the step names this operation");

    let operation = v2::CreatePage { body: step.body_required() };

    let options = v2::create_page::Options {
        embedded: step.boolean("embedded"),
        private: step.boolean("private"),
        root_level: step.boolean("root-level"),
    };

    assert!(step.is_spent(), "createPage reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_page_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createPageProperty", "the step names this operation");

    let operation = v2::CreatePageProperty {
        body: step.body_required(),
        page_id: step.number_required("page-id"),
    };

    assert!(step.is_spent(), "createPageProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_smart_link<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createSmartLink", "the step names this operation");

    let operation = v2::CreateSmartLink { body: step.body_required() };

    assert!(step.is_spent(), "createSmartLink reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_smart_link_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createSmartLinkProperty", "the step names this operation");

    let operation =
        v2::CreateSmartLinkProperty { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "createSmartLinkProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createSpace", "the step names this operation");

    let operation = v2::CreateSpace { body: step.body_required() };

    assert!(step.is_spent(), "createSpace reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_space_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createSpaceProperty", "the step names this operation");

    let operation = v2::CreateSpaceProperty {
        body: step.body_required(),
        space_id: step.number_required("space-id"),
    };

    assert!(step.is_spent(), "createSpaceProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_space_role<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createSpaceRole", "the step names this operation");

    let operation = v2::CreateSpaceRole { body: step.body_required() };

    assert!(step.is_spent(), "createSpaceRole reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn create_whiteboard<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createWhiteboard", "the step names this operation");

    let operation = v2::CreateWhiteboard { body: step.body_required() };
    let options = v2::create_whiteboard::Options { private: step.boolean("private") };

    assert!(step.is_spent(), "createWhiteboard reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_whiteboard_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createWhiteboardProperty", "the step names this operation");

    let operation =
        v2::CreateWhiteboardProperty { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "createWhiteboardProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_attachment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteAttachment", "the step names this operation");

    let operation = v2::DeleteAttachment { id: step.number_required("id") };
    let options = v2::delete_attachment::Options { purge: step.boolean("purge") };

    assert!(step.is_spent(), "deleteAttachment reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_attachment_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteAttachmentPropertyById", "the step names this operation");

    let operation = v2::DeleteAttachmentPropertyByID {
        attachment_id: step.text_required("attachment-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteAttachmentPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_blog_post<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteBlogPost", "the step names this operation");

    let operation = v2::DeleteBlogPost { id: step.number_required("id") };

    let options = v2::delete_blog_post::Options {
        draft: step.boolean("draft"),
        purge: step.boolean("purge"),
    };

    assert!(step.is_spent(), "deleteBlogPost reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_blog_post_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteBlogPostPermissions", "the step names this operation");

    let operation = v2::DeleteBlogPostPermissions { id: step.number_required("id") };

    let options = v2::delete_blog_post_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "deleteBlogPostPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_blogpost_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteBlogpostPropertyById", "the step names this operation");

    let operation = v2::DeleteBlogpostPropertyByID {
        blogpost_id: step.number_required("blogpost-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteBlogpostPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_comment_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteCommentPropertyById", "the step names this operation");

    let operation = v2::DeleteCommentPropertyByID {
        comment_id: step.number_required("comment-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteCommentPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_custom_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteCustomContent", "the step names this operation");

    let operation = v2::DeleteCustomContent { id: step.number_required("id") };
    let options = v2::delete_custom_content::Options { purge: step.boolean("purge") };

    assert!(step.is_spent(), "deleteCustomContent reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_custom_content_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "deleteCustomContentPropertyById",
        "the step names this operation",
    );

    let operation = v2::DeleteCustomContentPropertyByID {
        custom_content_id: step.number_required("custom-content-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteCustomContentPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_database<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteDatabase", "the step names this operation");

    let operation = v2::DeleteDatabase { id: step.number_required("id") };

    assert!(step.is_spent(), "deleteDatabase reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_database_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteDatabasePermissions", "the step names this operation");

    let operation = v2::DeleteDatabasePermissions { id: step.number_required("id") };

    let options = v2::delete_database_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "deleteDatabasePermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_database_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteDatabasePropertyById", "the step names this operation");

    let operation = v2::DeleteDatabasePropertyByID {
        database_id: step.number_required("database-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteDatabasePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_folder<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteFolder", "the step names this operation");

    let operation = v2::DeleteFolder { id: step.number_required("id") };

    assert!(step.is_spent(), "deleteFolder reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_folder_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteFolderPermissions", "the step names this operation");

    let operation = v2::DeleteFolderPermissions { id: step.number_required("id") };

    let options = v2::delete_folder_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "deleteFolderPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_folder_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteFolderPropertyById", "the step names this operation");

    let operation = v2::DeleteFolderPropertyByID {
        folder_id: step.number_required("folder-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteFolderPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_footer_comment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteFooterComment", "the step names this operation");

    let operation = v2::DeleteFooterComment { comment_id: step.number_required("comment-id") };

    assert!(step.is_spent(), "deleteFooterComment reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_forge_app_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteForgeAppProperty", "the step names this operation");

    let operation = v2::DeleteForgeAppProperty { property_key: step.text_required("propertyKey") };

    assert!(step.is_spent(), "deleteForgeAppProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_inline_comment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteInlineComment", "the step names this operation");

    let operation = v2::DeleteInlineComment { comment_id: step.number_required("comment-id") };

    assert!(step.is_spent(), "deleteInlineComment reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_page<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deletePage", "the step names this operation");

    let operation = v2::DeletePage { id: step.number_required("id") };

    let options =
        v2::delete_page::Options { draft: step.boolean("draft"), purge: step.boolean("purge") };

    assert!(step.is_spent(), "deletePage reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_page_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deletePagePermissions", "the step names this operation");

    let operation = v2::DeletePagePermissions { id: step.number_required("id") };

    let options = v2::delete_page_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "deletePagePermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_page_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deletePagePropertyById", "the step names this operation");

    let operation = v2::DeletePagePropertyByID {
        page_id: step.number_required("page-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deletePagePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_smart_link<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteSmartLink", "the step names this operation");

    let operation = v2::DeleteSmartLink { id: step.number_required("id") };

    assert!(step.is_spent(), "deleteSmartLink reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_smart_link_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteSmartLinkPropertyById", "the step names this operation");

    let operation = v2::DeleteSmartLinkPropertyByID {
        embed_id: step.number_required("embed-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "deleteSmartLinkPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_space_default_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "deleteSpaceDefaultClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::DeleteSpaceDefaultClassificationLevel { id: step.number_required("id") };

    assert!(
        step.is_spent(),
        "deleteSpaceDefaultClassificationLevel reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn delete_space_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteSpacePropertyById", "the step names this operation");

    let operation = v2::DeleteSpacePropertyByID {
        property_id: step.number_required("property-id"),
        space_id: step.number_required("space-id"),
    };

    assert!(step.is_spent(), "deleteSpacePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_space_role<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteSpaceRole", "the step names this operation");

    let operation = v2::DeleteSpaceRole { id: step.text_required("id") };

    assert!(step.is_spent(), "deleteSpaceRole reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_whiteboard<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteWhiteboard", "the step names this operation");

    let operation = v2::DeleteWhiteboard { id: step.number_required("id") };

    assert!(step.is_spent(), "deleteWhiteboard reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn delete_whiteboard_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteWhiteboardPermissions", "the step names this operation");

    let operation = v2::DeleteWhiteboardPermissions { id: step.number_required("id") };

    let options = v2::delete_whiteboard_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "deleteWhiteboardPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_whiteboard_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteWhiteboardPropertyById", "the step names this operation");

    let operation = v2::DeleteWhiteboardPropertyByID {
        property_id: step.number_required("property-id"),
        whiteboard_id: step.number_required("whiteboard-id"),
    };

    assert!(step.is_spent(), "deleteWhiteboardPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn disable_admin_key<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "disableAdminKey", "the step names this operation");

    let operation = v2::DisableAdminKey;

    assert!(step.is_spent(), "disableAdminKey reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn enable_admin_key<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "enableAdminKey", "the step names this operation");

    let operation = v2::EnableAdminKey;
    let options = v2::enable_admin_key::Options { body: step.body() };

    assert!(step.is_spent(), "enableAdminKey reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn generate_space_permission_combinations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "generateSpacePermissionCombinations",
        "the step names this operation",
    );

    let operation = v2::GenerateSpacePermissionCombinations;

    assert!(
        step.is_spent(),
        "generateSpacePermissionCombinations reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn get_admin_key<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAdminKey", "the step names this operation");

    let operation = v2::GetAdminKey;

    assert!(step.is_spent(), "getAdminKey reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_attachment_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentById", "the step names this operation");

    let operation = v2::GetAttachmentByID { id: step.text_required("id") };

    let options = v2::get_attachment_by_id::Options {
        include_collaborators: step.boolean("include-collaborators"),
        include_labels: step.boolean("include-labels"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
        include_version: step.boolean("include-version"),
        include_versions: step.boolean("include-versions"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getAttachmentById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_attachment_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentComments", "the step names this operation");

    let operation = v2::GetAttachmentComments { id: step.text_required("id") };

    let options = v2::get_attachment_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getAttachmentComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_attachment_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getAttachmentContentProperties",
        "the step names this operation",
    );

    let operation =
        v2::GetAttachmentContentProperties { attachment_id: step.text_required("attachment-id") };

    let options = v2::get_attachment_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getAttachmentContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_attachment_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getAttachmentContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetAttachmentContentPropertiesByID {
        attachment_id: step.text_required("attachment-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getAttachmentContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_attachment_labels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentLabels", "the step names this operation");

    let operation = v2::GetAttachmentLabels { id: step.number_required("id") };

    let options = v2::get_attachment_labels::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getAttachmentLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_attachment_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentOperations", "the step names this operation");

    let operation = v2::GetAttachmentOperations { id: step.text_required("id") };

    assert!(step.is_spent(), "getAttachmentOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_attachment_thumbnail_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentThumbnailById", "the step names this operation");

    let operation = v2::GetAttachmentThumbnailByID { id: step.text_required("id") };

    let options = v2::get_attachment_thumbnail_by_id::Options {
        height: step.number("height"),
        version: step.number("version"),
        width: step.number("width"),
    };

    assert!(step.is_spent(), "getAttachmentThumbnailById reads every value the step holds");

    step.download(client, &operation, &options)
}

pub(crate) fn get_attachment_version_details<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentVersionDetails", "the step names this operation");

    let operation = v2::GetAttachmentVersionDetails {
        attachment_id: step.text_required("attachment-id"),
        version_number: step.number_required("version-number"),
    };

    assert!(step.is_spent(), "getAttachmentVersionDetails reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_attachment_versions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachmentVersions", "the step names this operation");

    let operation = v2::GetAttachmentVersions { id: step.text_required("id") };

    let options = v2::get_attachment_versions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getAttachmentVersions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_attachments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAttachments", "the step names this operation");

    let operation = v2::GetAttachments;

    let options = v2::get_attachments::Options {
        cursor: step.text("cursor"),
        filename: step.text("filename"),
        limit: step.number("limit"),
        media_type: step.text("mediaType"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getAttachments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_available_space_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAvailableSpacePermissions", "the step names this operation");

    let operation = v2::GetAvailableSpacePermissions;

    let options = v2::get_available_space_permissions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getAvailableSpacePermissions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_available_space_roles<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getAvailableSpaceRoles", "the step names this operation");

    let operation = v2::GetAvailableSpaceRoles;

    let options = v2::get_available_space_roles::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
        role_type: step.text("role-type"),
        space_id: step.text("space-id"),
    };

    assert!(step.is_spent(), "getAvailableSpaceRoles reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostById", "the step names this operation");

    let operation = v2::GetBlogPostByID { id: step.number_required("id") };

    let options = v2::get_blog_post_by_id::Options {
        body_format: step.text("body-format"),
        get_draft: step.boolean("get-draft"),
        include_collaborators: step.boolean("include-collaborators"),
        include_favorited_by_user_status_current: step
            .boolean("include-favorited-by-current-user-status"),
        include_labels: step.boolean("include-labels"),
        include_likes: step.boolean("include-likes"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
        include_version: step.boolean("include-version"),
        include_versions: step.boolean("include-versions"),
        include_webresources: step.boolean("include-webresources"),
        status: step.texts("status"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getBlogPostById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getBlogPostClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::GetBlogPostClassificationLevel { id: step.number_required("id") };
    let options = v2::get_blog_post_classification_level::Options { status: step.text("status") };

    assert!(step.is_spent(), "getBlogPostClassificationLevel reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_footer_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostFooterComments", "the step names this operation");

    let operation = v2::GetBlogPostFooterComments { id: step.number_required("id") };

    let options = v2::get_blog_post_footer_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getBlogPostFooterComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_inline_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostInlineComments", "the step names this operation");

    let operation = v2::GetBlogPostInlineComments { id: step.number_required("id") };

    let options = v2::get_blog_post_inline_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        resolution_status: step.texts("resolution-status"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getBlogPostInlineComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_labels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostLabels", "the step names this operation");

    let operation = v2::GetBlogPostLabels { id: step.number_required("id") };

    let options = v2::get_blog_post_labels::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getBlogPostLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_like_count<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostLikeCount", "the step names this operation");

    let operation = v2::GetBlogPostLikeCount { id: step.number_required("id") };

    assert!(step.is_spent(), "getBlogPostLikeCount reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_blog_post_like_users<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostLikeUsers", "the step names this operation");

    let operation = v2::GetBlogPostLikeUsers { id: step.number_required("id") };

    let options = v2::get_blog_post_like_users::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getBlogPostLikeUsers reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostOperations", "the step names this operation");

    let operation = v2::GetBlogPostOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getBlogPostOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_blog_post_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostPermissions", "the step names this operation");

    let operation = v2::GetBlogPostPermissions { id: step.number_required("id") };

    let options = v2::get_blog_post_permissions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "getBlogPostPermissions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_post_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostRestrictionState", "the step names this operation");

    let operation = v2::GetBlogPostRestrictionState { id: step.number_required("id") };

    assert!(step.is_spent(), "getBlogPostRestrictionState reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_blog_post_version_details<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostVersionDetails", "the step names this operation");

    let operation = v2::GetBlogPostVersionDetails {
        blogpost_id: step.number_required("blogpost-id"),
        version_number: step.number_required("version-number"),
    };

    assert!(step.is_spent(), "getBlogPostVersionDetails reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_blog_post_versions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostVersions", "the step names this operation");

    let operation = v2::GetBlogPostVersions { id: step.number_required("id") };

    let options = v2::get_blog_post_versions::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getBlogPostVersions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_posts<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPosts", "the step names this operation");

    let operation = v2::GetBlogPosts;

    let options = v2::get_blog_posts::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        id: step.numbers("id"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        space_id: step.numbers("space-id"),
        status: step.texts("status"),
        title: step.text("title"),
    };

    assert!(step.is_spent(), "getBlogPosts reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blog_posts_in_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogPostsInSpace", "the step names this operation");

    let operation = v2::GetBlogPostsInSpace { id: step.number_required("id") };

    let options = v2::get_blog_posts_in_space::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        status: step.texts("status"),
        title: step.text("title"),
    };

    assert!(step.is_spent(), "getBlogPostsInSpace reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blogpost_attachments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogpostAttachments", "the step names this operation");

    let operation = v2::GetBlogpostAttachments { id: step.number_required("id") };

    let options = v2::get_blogpost_attachments::Options {
        cursor: step.text("cursor"),
        filename: step.text("filename"),
        limit: step.number("limit"),
        media_type: step.text("mediaType"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getBlogpostAttachments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blogpost_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getBlogpostContentProperties", "the step names this operation");

    let operation =
        v2::GetBlogpostContentProperties { blogpost_id: step.number_required("blogpost-id") };

    let options = v2::get_blogpost_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getBlogpostContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_blogpost_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getBlogpostContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetBlogpostContentPropertiesByID {
        blogpost_id: step.number_required("blogpost-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getBlogpostContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_child_custom_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getChildCustomContent", "the step names this operation");

    let operation = v2::GetChildCustomContent { id: step.number_required("id") };

    let options = v2::get_child_custom_content::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getChildCustomContent reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_child_pages<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getChildPages", "the step names this operation");

    let operation = v2::GetChildPages { id: step.number_required("id") };

    let options = v2::get_child_pages::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getChildPages reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_classification_levels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getClassificationLevels", "the step names this operation");

    let operation = v2::GetClassificationLevels;

    assert!(step.is_spent(), "getClassificationLevels reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_comment_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCommentContentProperties", "the step names this operation");

    let operation =
        v2::GetCommentContentProperties { comment_id: step.number_required("comment-id") };

    let options = v2::get_comment_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCommentContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_comment_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getCommentContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetCommentContentPropertiesByID {
        comment_id: step.number_required("comment-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getCommentContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_custom_content_attachments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentAttachments", "the step names this operation");

    let operation = v2::GetCustomContentAttachments { id: step.number_required("id") };

    let options = v2::get_custom_content_attachments::Options {
        cursor: step.text("cursor"),
        filename: step.text("filename"),
        limit: step.number("limit"),
        media_type: step.text("mediaType"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getCustomContentAttachments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentById", "the step names this operation");

    let operation = v2::GetCustomContentByID { id: step.number_required("id") };

    let options = v2::get_custom_content_by_id::Options {
        body_format: step.text("body-format"),
        include_collaborators: step.boolean("include-collaborators"),
        include_labels: step.boolean("include-labels"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
        include_version: step.boolean("include-version"),
        include_versions: step.boolean("include-versions"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getCustomContentById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_by_type<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentByType", "the step names this operation");

    let operation = v2::GetCustomContentByType { kind: step.text_required("type") };

    let options = v2::get_custom_content_by_type::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        id: step.numbers("id"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        space_id: step.numbers("space-id"),
    };

    assert!(step.is_spent(), "getCustomContentByType reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_by_type_in_blog_post<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getCustomContentByTypeInBlogPost",
        "the step names this operation",
    );

    let operation = v2::GetCustomContentByTypeInBlogPost {
        id: step.number_required("id"),
        kind: step.text_required("type"),
    };

    let options = v2::get_custom_content_by_type_in_blog_post::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCustomContentByTypeInBlogPost reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_by_type_in_page<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentByTypeInPage", "the step names this operation");

    let operation = v2::GetCustomContentByTypeInPage {
        id: step.number_required("id"),
        kind: step.text_required("type"),
    };

    let options = v2::get_custom_content_by_type_in_page::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCustomContentByTypeInPage reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_by_type_in_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentByTypeInSpace", "the step names this operation");

    let operation = v2::GetCustomContentByTypeInSpace {
        id: step.number_required("id"),
        kind: step.text_required("type"),
    };

    let options = v2::get_custom_content_by_type_in_space::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getCustomContentByTypeInSpace reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentComments", "the step names this operation");

    let operation = v2::GetCustomContentComments { id: step.number_required("id") };

    let options = v2::get_custom_content_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCustomContentComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getCustomContentContentProperties",
        "the step names this operation",
    );

    let operation = v2::GetCustomContentContentProperties {
        custom_content_id: step.number_required("custom-content-id"),
    };

    let options = v2::get_custom_content_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCustomContentContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getCustomContentContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetCustomContentContentPropertiesByID {
        custom_content_id: step.number_required("custom-content-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(
        step.is_spent(),
        "getCustomContentContentPropertiesById reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn get_custom_content_labels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentLabels", "the step names this operation");

    let operation = v2::GetCustomContentLabels { id: step.number_required("id") };

    let options = v2::get_custom_content_labels::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCustomContentLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_custom_content_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentOperations", "the step names this operation");

    let operation = v2::GetCustomContentOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getCustomContentOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_custom_content_version_details<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getCustomContentVersionDetails",
        "the step names this operation",
    );

    let operation = v2::GetCustomContentVersionDetails {
        custom_content_id: step.number_required("custom-content-id"),
        version_number: step.number_required("version-number"),
    };

    assert!(step.is_spent(), "getCustomContentVersionDetails reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_custom_content_versions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getCustomContentVersions", "the step names this operation");

    let operation = v2::GetCustomContentVersions {
        custom_content_id: step.number_required("custom-content-id"),
    };

    let options = v2::get_custom_content_versions::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getCustomContentVersions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_data_policy_metadata<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDataPolicyMetadata", "the step names this operation");

    let operation = v2::GetDataPolicyMetadata;

    assert!(step.is_spent(), "getDataPolicyMetadata reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_data_policy_spaces<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDataPolicySpaces", "the step names this operation");

    let operation = v2::GetDataPolicySpaces;

    let options = v2::get_data_policy_spaces::Options {
        cursor: step.text("cursor"),
        ids: step.numbers("ids"),
        keys: step.texts("keys"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getDataPolicySpaces reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_ancestors<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseAncestors", "the step names this operation");

    let operation = v2::GetDatabaseAncestors { id: step.number_required("id") };
    let options = v2::get_database_ancestors::Options { limit: step.number("limit") };

    assert!(step.is_spent(), "getDatabaseAncestors reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseById", "the step names this operation");

    let operation = v2::GetDatabaseByID { id: step.number_required("id") };

    let options = v2::get_database_by_id::Options {
        include_collaborators: step.boolean("include-collaborators"),
        include_direct_children: step.boolean("include-direct-children"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
    };

    assert!(step.is_spent(), "getDatabaseById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getDatabaseClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::GetDatabaseClassificationLevel { id: step.number_required("id") };

    assert!(step.is_spent(), "getDatabaseClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_database_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseContentProperties", "the step names this operation");

    let operation = v2::GetDatabaseContentProperties { id: step.number_required("id") };

    let options = v2::get_database_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getDatabaseContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getDatabaseContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetDatabaseContentPropertiesByID {
        database_id: step.number_required("database-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getDatabaseContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_database_descendants<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseDescendants", "the step names this operation");

    let operation = v2::GetDatabaseDescendants { id: step.number_required("id") };

    let options = v2::get_database_descendants::Options {
        cursor: step.text("cursor"),
        depth: step.number("depth"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getDatabaseDescendants reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_direct_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseDirectChildren", "the step names this operation");

    let operation = v2::GetDatabaseDirectChildren { id: step.number_required("id") };

    let options = v2::get_database_direct_children::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getDatabaseDirectChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_entries<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseEntries", "the step names this operation");

    let operation = v2::GetDatabaseEntries { id: step.text_required("id") };

    let options = v2::get_database_entries::Options {
        cursor: step.text("cursor"),
        filter: step.pairs("filter"),
        if_none_match: step.text("If-None-Match"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getDatabaseEntries reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_entry_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseEntryById", "the step names this operation");

    let operation = v2::GetDatabaseEntryByID {
        entry_id: step.text_required("entryId"),
        id: step.text_required("id"),
    };

    let options =
        v2::get_database_entry_by_id::Options { if_none_match: step.text("If-None-Match") };

    assert!(step.is_spent(), "getDatabaseEntryById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_fields<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseFields", "the step names this operation");

    let operation = v2::GetDatabaseFields { id: step.text_required("id") };

    let options = v2::get_database_fields::Options {
        cursor: step.text("cursor"),
        if_none_match: step.text("If-None-Match"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getDatabaseFields reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseOperations", "the step names this operation");

    let operation = v2::GetDatabaseOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getDatabaseOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_database_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabasePermissions", "the step names this operation");

    let operation = v2::GetDatabasePermissions { id: step.number_required("id") };

    let options = v2::get_database_permissions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "getDatabasePermissions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_database_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseRestrictionState", "the step names this operation");

    let operation = v2::GetDatabaseRestrictionState { id: step.number_required("id") };

    assert!(step.is_spent(), "getDatabaseRestrictionState reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_database_views<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getDatabaseViews", "the step names this operation");

    let operation = v2::GetDatabaseViews { id: step.text_required("id") };

    let options = v2::get_database_views::Options {
        cursor: step.text("cursor"),
        if_none_match: step.text("If-None-Match"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getDatabaseViews reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_ancestors<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderAncestors", "the step names this operation");

    let operation = v2::GetFolderAncestors { id: step.number_required("id") };
    let options = v2::get_folder_ancestors::Options { limit: step.number("limit") };

    assert!(step.is_spent(), "getFolderAncestors reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderById", "the step names this operation");

    let operation = v2::GetFolderByID { id: step.number_required("id") };

    let options = v2::get_folder_by_id::Options {
        include_collaborators: step.boolean("include-collaborators"),
        include_direct_children: step.boolean("include-direct-children"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
    };

    assert!(step.is_spent(), "getFolderById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderContentProperties", "the step names this operation");

    let operation = v2::GetFolderContentProperties { id: step.number_required("id") };

    let options = v2::get_folder_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getFolderContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getFolderContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetFolderContentPropertiesByID {
        folder_id: step.number_required("folder-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getFolderContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_folder_descendants<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderDescendants", "the step names this operation");

    let operation = v2::GetFolderDescendants { id: step.number_required("id") };

    let options = v2::get_folder_descendants::Options {
        cursor: step.text("cursor"),
        depth: step.number("depth"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getFolderDescendants reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_direct_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderDirectChildren", "the step names this operation");

    let operation = v2::GetFolderDirectChildren { id: step.number_required("id") };

    let options = v2::get_folder_direct_children::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getFolderDirectChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderOperations", "the step names this operation");

    let operation = v2::GetFolderOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getFolderOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_folder_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderPermissions", "the step names this operation");

    let operation = v2::GetFolderPermissions { id: step.number_required("id") };

    let options = v2::get_folder_permissions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "getFolderPermissions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_folder_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFolderRestrictionState", "the step names this operation");

    let operation = v2::GetFolderRestrictionState { id: step.number_required("id") };

    assert!(step.is_spent(), "getFolderRestrictionState reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_footer_comment_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterCommentById", "the step names this operation");

    let operation = v2::GetFooterCommentByID { comment_id: step.number_required("comment-id") };

    let options = v2::get_footer_comment_by_id::Options {
        body_format: step.text("body-format"),
        include_likes: step.boolean("include-likes"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
        include_version: step.boolean("include-version"),
        include_versions: step.boolean("include-versions"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getFooterCommentById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_footer_comment_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterCommentChildren", "the step names this operation");

    let operation = v2::GetFooterCommentChildren { id: step.number_required("id") };

    let options = v2::get_footer_comment_children::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getFooterCommentChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_footer_comment_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterCommentOperations", "the step names this operation");

    let operation = v2::GetFooterCommentOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getFooterCommentOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_footer_comment_version_details<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getFooterCommentVersionDetails",
        "the step names this operation",
    );

    let operation = v2::GetFooterCommentVersionDetails {
        id: step.number_required("id"),
        version_number: step.number_required("version-number"),
    };

    assert!(step.is_spent(), "getFooterCommentVersionDetails reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_footer_comment_versions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterCommentVersions", "the step names this operation");

    let operation = v2::GetFooterCommentVersions { id: step.number_required("id") };

    let options = v2::get_footer_comment_versions::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getFooterCommentVersions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_footer_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterComments", "the step names this operation");

    let operation = v2::GetFooterComments;

    let options = v2::get_footer_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getFooterComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_footer_like_count<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterLikeCount", "the step names this operation");

    let operation = v2::GetFooterLikeCount { id: step.number_required("id") };

    assert!(step.is_spent(), "getFooterLikeCount reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_footer_like_users<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getFooterLikeUsers", "the step names this operation");

    let operation = v2::GetFooterLikeUsers { id: step.number_required("id") };

    let options = v2::get_footer_like_users::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getFooterLikeUsers reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_forge_app_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getForgeAppProperties", "the step names this operation");

    let operation = v2::GetForgeAppProperties;

    let options = v2::get_forge_app_properties::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getForgeAppProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_forge_app_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getForgeAppProperty", "the step names this operation");

    let operation = v2::GetForgeAppProperty { property_key: step.text_required("propertyKey") };

    assert!(step.is_spent(), "getForgeAppProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_inline_comment_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineCommentById", "the step names this operation");

    let operation = v2::GetInlineCommentByID { comment_id: step.number_required("comment-id") };

    let options = v2::get_inline_comment_by_id::Options {
        body_format: step.text("body-format"),
        include_likes: step.boolean("include-likes"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
        include_version: step.boolean("include-version"),
        include_versions: step.boolean("include-versions"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getInlineCommentById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_inline_comment_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineCommentChildren", "the step names this operation");

    let operation = v2::GetInlineCommentChildren { id: step.number_required("id") };

    let options = v2::get_inline_comment_children::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getInlineCommentChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_inline_comment_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineCommentOperations", "the step names this operation");

    let operation = v2::GetInlineCommentOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getInlineCommentOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_inline_comment_version_details<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getInlineCommentVersionDetails",
        "the step names this operation",
    );

    let operation = v2::GetInlineCommentVersionDetails {
        id: step.number_required("id"),
        version_number: step.number_required("version-number"),
    };

    assert!(step.is_spent(), "getInlineCommentVersionDetails reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_inline_comment_versions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineCommentVersions", "the step names this operation");

    let operation = v2::GetInlineCommentVersions { id: step.number_required("id") };

    let options = v2::get_inline_comment_versions::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getInlineCommentVersions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_inline_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineComments", "the step names this operation");

    let operation = v2::GetInlineComments;

    let options = v2::get_inline_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getInlineComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_inline_like_count<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineLikeCount", "the step names this operation");

    let operation = v2::GetInlineLikeCount { id: step.number_required("id") };

    assert!(step.is_spent(), "getInlineLikeCount reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_inline_like_users<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getInlineLikeUsers", "the step names this operation");

    let operation = v2::GetInlineLikeUsers { id: step.number_required("id") };

    let options = v2::get_inline_like_users::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getInlineLikeUsers reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_label_attachments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getLabelAttachments", "the step names this operation");

    let operation = v2::GetLabelAttachments { id: step.number_required("id") };

    let options = v2::get_label_attachments::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getLabelAttachments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_label_blog_posts<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getLabelBlogPosts", "the step names this operation");

    let operation = v2::GetLabelBlogPosts { id: step.number_required("id") };

    let options = v2::get_label_blog_posts::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        space_id: step.numbers("space-id"),
    };

    assert!(step.is_spent(), "getLabelBlogPosts reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_label_pages<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getLabelPages", "the step names this operation");

    let operation = v2::GetLabelPages { id: step.number_required("id") };

    let options = v2::get_label_pages::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        space_id: step.numbers("space-id"),
    };

    assert!(step.is_spent(), "getLabelPages reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_labels<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getLabels", "the step names this operation");

    let operation = v2::GetLabels;

    let options = v2::get_labels::Options {
        cursor: step.text("cursor"),
        label_id: step.numbers("label-id"),
        limit: step.number("limit"),
        prefix: step.texts("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_ancestors<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageAncestors", "the step names this operation");

    let operation = v2::GetPageAncestors { id: step.number_required("id") };
    let options = v2::get_page_ancestors::Options { limit: step.number("limit") };

    assert!(step.is_spent(), "getPageAncestors reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_attachments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageAttachments", "the step names this operation");

    let operation = v2::GetPageAttachments { id: step.number_required("id") };

    let options = v2::get_page_attachments::Options {
        cursor: step.text("cursor"),
        filename: step.text("filename"),
        limit: step.number("limit"),
        media_type: step.text("mediaType"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getPageAttachments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageById", "the step names this operation");

    let operation = v2::GetPageByID { id: step.number_required("id") };

    let options = v2::get_page_by_id::Options {
        body_format: step.text("body-format"),
        get_draft: step.boolean("get-draft"),
        include_collaborators: step.boolean("include-collaborators"),
        include_direct_children: step.boolean("include-direct-children"),
        include_favorited_by_user_status_current: step
            .boolean("include-favorited-by-current-user-status"),
        include_labels: step.boolean("include-labels"),
        include_likes: step.boolean("include-likes"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
        include_version: step.boolean("include-version"),
        include_versions: step.boolean("include-versions"),
        include_webresources: step.boolean("include-webresources"),
        status: step.texts("status"),
        version: step.number("version"),
    };

    assert!(step.is_spent(), "getPageById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageClassificationLevel", "the step names this operation");

    let operation = v2::GetPageClassificationLevel { id: step.number_required("id") };
    let options = v2::get_page_classification_level::Options { status: step.text("status") };

    assert!(step.is_spent(), "getPageClassificationLevel reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageContentProperties", "the step names this operation");

    let operation = v2::GetPageContentProperties { page_id: step.number_required("page-id") };

    let options = v2::get_page_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getPageContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageContentPropertiesById", "the step names this operation");

    let operation = v2::GetPageContentPropertiesByID {
        page_id: step.number_required("page-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getPageContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_page_descendants<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageDescendants", "the step names this operation");

    let operation = v2::GetPageDescendants { id: step.number_required("id") };

    let options = v2::get_page_descendants::Options {
        cursor: step.text("cursor"),
        depth: step.number("depth"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getPageDescendants reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_direct_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageDirectChildren", "the step names this operation");

    let operation = v2::GetPageDirectChildren { id: step.number_required("id") };

    let options = v2::get_page_direct_children::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getPageDirectChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_footer_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageFooterComments", "the step names this operation");

    let operation = v2::GetPageFooterComments { id: step.number_required("id") };

    let options = v2::get_page_footer_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getPageFooterComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_inline_comments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageInlineComments", "the step names this operation");

    let operation = v2::GetPageInlineComments { id: step.number_required("id") };

    let options = v2::get_page_inline_comments::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        resolution_status: step.texts("resolution-status"),
        sort: step.text("sort"),
        status: step.texts("status"),
    };

    assert!(step.is_spent(), "getPageInlineComments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_labels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageLabels", "the step names this operation");

    let operation = v2::GetPageLabels { id: step.number_required("id") };

    let options = v2::get_page_labels::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getPageLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_like_count<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageLikeCount", "the step names this operation");

    let operation = v2::GetPageLikeCount { id: step.number_required("id") };

    assert!(step.is_spent(), "getPageLikeCount reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_page_like_users<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageLikeUsers", "the step names this operation");

    let operation = v2::GetPageLikeUsers { id: step.number_required("id") };

    let options = v2::get_page_like_users::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getPageLikeUsers reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageOperations", "the step names this operation");

    let operation = v2::GetPageOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getPageOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_page_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPagePermissions", "the step names this operation");

    let operation = v2::GetPagePermissions { id: step.number_required("id") };

    let options = v2::get_page_permissions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "getPagePermissions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_page_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageRestrictionState", "the step names this operation");

    let operation = v2::GetPageRestrictionState { id: step.number_required("id") };

    assert!(step.is_spent(), "getPageRestrictionState reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_page_version_details<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageVersionDetails", "the step names this operation");

    let operation = v2::GetPageVersionDetails {
        page_id: step.number_required("page-id"),
        version_number: step.number_required("version-number"),
    };

    assert!(step.is_spent(), "getPageVersionDetails reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_page_versions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPageVersions", "the step names this operation");

    let operation = v2::GetPageVersions { id: step.number_required("id") };

    let options = v2::get_page_versions::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getPageVersions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_pages<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPages", "the step names this operation");

    let operation = v2::GetPages;

    let options = v2::get_pages::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        id: step.numbers("id"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        space_id: step.numbers("space-id"),
        status: step.texts("status"),
        subtype: step.text("subtype"),
        title: step.text("title"),
    };

    assert!(step.is_spent(), "getPages reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_pages_in_space<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getPagesInSpace", "the step names this operation");

    let operation = v2::GetPagesInSpace { id: step.number_required("id") };

    let options = v2::get_pages_in_space::Options {
        body_format: step.text("body-format"),
        cursor: step.text("cursor"),
        depth: step.text("depth"),
        limit: step.number("limit"),
        sort: step.text("sort"),
        status: step.texts("status"),
        title: step.text("title"),
    };

    assert!(step.is_spent(), "getPagesInSpace reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_smart_link_ancestors<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSmartLinkAncestors", "the step names this operation");

    let operation = v2::GetSmartLinkAncestors { id: step.number_required("id") };
    let options = v2::get_smart_link_ancestors::Options { limit: step.number("limit") };

    assert!(step.is_spent(), "getSmartLinkAncestors reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_smart_link_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSmartLinkById", "the step names this operation");

    let operation = v2::GetSmartLinkByID { id: step.number_required("id") };

    let options = v2::get_smart_link_by_id::Options {
        include_collaborators: step.boolean("include-collaborators"),
        include_direct_children: step.boolean("include-direct-children"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
    };

    assert!(step.is_spent(), "getSmartLinkById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_smart_link_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSmartLinkContentProperties", "the step names this operation");

    let operation = v2::GetSmartLinkContentProperties { id: step.number_required("id") };

    let options = v2::get_smart_link_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getSmartLinkContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_smart_link_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getSmartLinkContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetSmartLinkContentPropertiesByID {
        embed_id: step.number_required("embed-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "getSmartLinkContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_smart_link_descendants<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSmartLinkDescendants", "the step names this operation");

    let operation = v2::GetSmartLinkDescendants { id: step.number_required("id") };

    let options = v2::get_smart_link_descendants::Options {
        cursor: step.text("cursor"),
        depth: step.number("depth"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getSmartLinkDescendants reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_smart_link_direct_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSmartLinkDirectChildren", "the step names this operation");

    let operation = v2::GetSmartLinkDirectChildren { id: step.number_required("id") };

    let options = v2::get_smart_link_direct_children::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getSmartLinkDirectChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_smart_link_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSmartLinkOperations", "the step names this operation");

    let operation = v2::GetSmartLinkOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getSmartLinkOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceById", "the step names this operation");

    let operation = v2::GetSpaceByID { id: step.number_required("id") };

    let options = v2::get_space_by_id::Options {
        description_format: step.text("description-format"),
        include_icon: step.boolean("include-icon"),
        include_labels: step.boolean("include-labels"),
        include_operations: step.boolean("include-operations"),
        include_permissions: step.boolean("include-permissions"),
        include_properties: step.boolean("include-properties"),
        include_role_assignments: step.boolean("include-role-assignments"),
    };

    assert!(step.is_spent(), "getSpaceById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_space_content_labels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceContentLabels", "the step names this operation");

    let operation = v2::GetSpaceContentLabels { id: step.number_required("id") };

    let options = v2::get_space_content_labels::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getSpaceContentLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_space_default_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getSpaceDefaultClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::GetSpaceDefaultClassificationLevel { id: step.number_required("id") };

    assert!(step.is_spent(), "getSpaceDefaultClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_labels<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceLabels", "the step names this operation");

    let operation = v2::GetSpaceLabels { id: step.number_required("id") };

    let options = v2::get_space_labels::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        prefix: step.text("prefix"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getSpaceLabels reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_space_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceOperations", "the step names this operation");

    let operation = v2::GetSpaceOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getSpaceOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_permission_transition_task_status<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getSpacePermissionTransitionTaskStatus",
        "the step names this operation",
    );

    let operation =
        v2::GetSpacePermissionTransitionTaskStatus { task_id: step.text_required("taskId") };

    assert!(
        step.is_spent(),
        "getSpacePermissionTransitionTaskStatus reads every value the step holds",
    );

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_permissions_assignments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getSpacePermissionsAssignments",
        "the step names this operation",
    );

    let operation = v2::GetSpacePermissionsAssignments { id: step.number_required("id") };

    let options = v2::get_space_permissions_assignments::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getSpacePermissionsAssignments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_space_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceProperties", "the step names this operation");

    let operation = v2::GetSpaceProperties { space_id: step.number_required("space-id") };

    let options = v2::get_space_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getSpaceProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_space_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpacePropertyById", "the step names this operation");

    let operation = v2::GetSpacePropertyByID {
        property_id: step.number_required("property-id"),
        space_id: step.number_required("space-id"),
    };

    assert!(step.is_spent(), "getSpacePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_role_assignments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceRoleAssignments", "the step names this operation");

    let operation = v2::GetSpaceRoleAssignments { id: step.number_required("id") };

    let options = v2::get_space_role_assignments::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
        role_id: step.text("role-id"),
        role_type: step.text("role-type"),
    };

    assert!(step.is_spent(), "getSpaceRoleAssignments reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_space_role_mode<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceRoleMode", "the step names this operation");

    let operation = v2::GetSpaceRoleMode;

    assert!(step.is_spent(), "getSpaceRoleMode reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_space_roles_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaceRolesById", "the step names this operation");

    let operation = v2::GetSpaceRolesByID { id: step.text_required("id") };

    assert!(step.is_spent(), "getSpaceRolesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_spaces<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getSpaces", "the step names this operation");

    let operation = v2::GetSpaces;

    let options = v2::get_spaces::Options {
        cursor: step.text("cursor"),
        description_format: step.text("description-format"),
        favorited_by: step.text("favorited-by"),
        ids: step.numbers("ids"),
        include_icon: step.boolean("include-icon"),
        keys: step.texts("keys"),
        kind: step.text("type"),
        labels: step.texts("labels"),
        limit: step.number("limit"),
        not_favorited_by: step.text("not-favorited-by"),
        sort: step.text("sort"),
        status: step.text("status"),
    };

    assert!(step.is_spent(), "getSpaces reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_task_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getTaskById", "the step names this operation");

    let operation = v2::GetTaskByID { id: step.number_required("id") };
    let options = v2::get_task_by_id::Options { body_format: step.text("body-format") };

    assert!(step.is_spent(), "getTaskById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_tasks<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getTasks", "the step names this operation");

    let operation = v2::GetTasks;

    let options = v2::get_tasks::Options {
        assigned_to: step.texts("assigned-to"),
        blogpost_id: step.numbers("blogpost-id"),
        body_format: step.text("body-format"),
        completed_at_from: step.number("completed-at-from"),
        completed_at_to: step.number("completed-at-to"),
        completed_by: step.texts("completed-by"),
        created_at_from: step.number("created-at-from"),
        created_at_to: step.number("created-at-to"),
        created_by: step.texts("created-by"),
        cursor: step.text("cursor"),
        due_at_from: step.number("due-at-from"),
        due_at_to: step.number("due-at-to"),
        include_blank_tasks: step.boolean("include-blank-tasks"),
        limit: step.number("limit"),
        page_id: step.numbers("page-id"),
        space_id: step.numbers("space-id"),
        status: step.text("status"),
        task_id: step.numbers("task-id"),
    };

    assert!(step.is_spent(), "getTasks reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_ancestors<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardAncestors", "the step names this operation");

    let operation = v2::GetWhiteboardAncestors { id: step.number_required("id") };
    let options = v2::get_whiteboard_ancestors::Options { limit: step.number("limit") };

    assert!(step.is_spent(), "getWhiteboardAncestors reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardById", "the step names this operation");

    let operation = v2::GetWhiteboardByID { id: step.number_required("id") };

    let options = v2::get_whiteboard_by_id::Options {
        include_collaborators: step.boolean("include-collaborators"),
        include_direct_children: step.boolean("include-direct-children"),
        include_operations: step.boolean("include-operations"),
        include_properties: step.boolean("include-properties"),
    };

    assert!(step.is_spent(), "getWhiteboardById reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getWhiteboardClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::GetWhiteboardClassificationLevel { id: step.number_required("id") };

    assert!(step.is_spent(), "getWhiteboardClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_whiteboard_content_properties<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getWhiteboardContentProperties",
        "the step names this operation",
    );

    let operation = v2::GetWhiteboardContentProperties { id: step.number_required("id") };

    let options = v2::get_whiteboard_content_properties::Options {
        cursor: step.text("cursor"),
        key: step.text("key"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getWhiteboardContentProperties reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_content_properties_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "getWhiteboardContentPropertiesById",
        "the step names this operation",
    );

    let operation = v2::GetWhiteboardContentPropertiesByID {
        property_id: step.number_required("property-id"),
        whiteboard_id: step.number_required("whiteboard-id"),
    };

    assert!(step.is_spent(), "getWhiteboardContentPropertiesById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_whiteboard_descendants<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardDescendants", "the step names this operation");

    let operation = v2::GetWhiteboardDescendants { id: step.number_required("id") };

    let options = v2::get_whiteboard_descendants::Options {
        cursor: step.text("cursor"),
        depth: step.number("depth"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "getWhiteboardDescendants reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_direct_children<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardDirectChildren", "the step names this operation");

    let operation = v2::GetWhiteboardDirectChildren { id: step.number_required("id") };

    let options = v2::get_whiteboard_direct_children::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        sort: step.text("sort"),
    };

    assert!(step.is_spent(), "getWhiteboardDirectChildren reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_operations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardOperations", "the step names this operation");

    let operation = v2::GetWhiteboardOperations { id: step.number_required("id") };

    assert!(step.is_spent(), "getWhiteboardOperations reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_whiteboard_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardPermissions", "the step names this operation");

    let operation = v2::GetWhiteboardPermissions { id: step.number_required("id") };

    let options = v2::get_whiteboard_permissions::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
        operation: step.text("operation"),
        principal_id: step.text("principal-id"),
        principal_type: step.text("principal-type"),
    };

    assert!(step.is_spent(), "getWhiteboardPermissions reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn get_whiteboard_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getWhiteboardRestrictionState", "the step names this operation");

    let operation = v2::GetWhiteboardRestrictionState { id: step.number_required("id") };

    assert!(step.is_spent(), "getWhiteboardRestrictionState reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn invite_by_email<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "inviteByEmail", "the step names this operation");

    let operation = v2::InviteByEmail { body: step.body_required() };

    assert!(step.is_spent(), "inviteByEmail reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn list_space_permission_combinations<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "listSpacePermissionCombinations",
        "the step names this operation",
    );

    let operation = v2::ListSpacePermissionCombinations;

    let options = v2::list_space_permission_combinations::Options {
        cursor: step.text("cursor"),
        limit: step.number("limit"),
    };

    assert!(step.is_spent(), "listSpacePermissionCombinations reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn post_blog_post_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "postBlogPostClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PostBlogPostClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "postBlogPostClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn post_database_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "postDatabaseClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PostDatabaseClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "postDatabaseClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn post_page_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "postPageClassificationLevel", "the step names this operation");

    let operation = v2::PostPageClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "postPageClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn post_redact_blog<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "postRedactBlog", "the step names this operation");

    let operation = v2::PostRedactBlog { id: step.number_required("id") };
    let options = v2::post_redact_blog::Options { body: step.body() };

    assert!(step.is_spent(), "postRedactBlog reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn post_redact_page<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "postRedactPage", "the step names this operation");

    let operation = v2::PostRedactPage { id: step.number_required("id") };
    let options = v2::post_redact_page::Options { body: step.body() };

    assert!(step.is_spent(), "postRedactPage reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn post_whiteboard_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "postWhiteboardClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PostWhiteboardClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "postWhiteboardClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn put_blog_post_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "putBlogPostClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PutBlogPostClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "putBlogPostClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn put_database_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "putDatabaseClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PutDatabaseClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "putDatabaseClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn put_forge_app_property<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "putForgeAppProperty", "the step names this operation");

    let operation = v2::PutForgeAppProperty {
        body: step.body_required(),
        property_key: step.text_required("propertyKey"),
    };

    assert!(step.is_spent(), "putForgeAppProperty reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn put_page_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "putPageClassificationLevel", "the step names this operation");

    let operation = v2::PutPageClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "putPageClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn put_space_default_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "putSpaceDefaultClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PutSpaceDefaultClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "putSpaceDefaultClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn put_whiteboard_classification_level<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "putWhiteboardClassificationLevel",
        "the step names this operation",
    );

    let operation = v2::PutWhiteboardClassificationLevel {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    assert!(step.is_spent(), "putWhiteboardClassificationLevel reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn set_blog_post_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setBlogPostPermissions", "the step names this operation");

    let operation =
        v2::SetBlogPostPermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::set_blog_post_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setBlogPostPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_blog_post_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setBlogPostRestrictionState", "the step names this operation");

    let operation = v2::SetBlogPostRestrictionState {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    let options = v2::set_blog_post_restriction_state::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setBlogPostRestrictionState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_database_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setDatabasePermissions", "the step names this operation");

    let operation =
        v2::SetDatabasePermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::set_database_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setDatabasePermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_database_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setDatabaseRestrictionState", "the step names this operation");

    let operation = v2::SetDatabaseRestrictionState {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    let options = v2::set_database_restriction_state::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setDatabaseRestrictionState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_folder_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setFolderPermissions", "the step names this operation");

    let operation =
        v2::SetFolderPermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::set_folder_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setFolderPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_folder_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setFolderRestrictionState", "the step names this operation");

    let operation = v2::SetFolderRestrictionState {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    let options = v2::set_folder_restriction_state::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setFolderRestrictionState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_page_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setPagePermissions", "the step names this operation");

    let operation =
        v2::SetPagePermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::set_page_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setPagePermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_page_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setPageRestrictionState", "the step names this operation");

    let operation =
        v2::SetPageRestrictionState { body: step.body_required(), id: step.number_required("id") };

    let options = v2::set_page_restriction_state::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setPageRestrictionState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_space_role_assignments<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setSpaceRoleAssignments", "the step names this operation");

    let operation =
        v2::SetSpaceRoleAssignments { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "setSpaceRoleAssignments reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn set_whiteboard_permissions<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setWhiteboardPermissions", "the step names this operation");

    let operation =
        v2::SetWhiteboardPermissions { body: step.body_required(), id: step.number_required("id") };

    let options = v2::set_whiteboard_permissions::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setWhiteboardPermissions reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn set_whiteboard_restriction_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "setWhiteboardRestrictionState", "the step names this operation");

    let operation = v2::SetWhiteboardRestrictionState {
        body: step.body_required(),
        id: step.number_required("id"),
    };

    let options = v2::set_whiteboard_restriction_state::Options {
        allow_self_lockout: step.boolean("allow-self-lockout"),
    };

    assert!(step.is_spent(), "setWhiteboardRestrictionState reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_attachment_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateAttachmentPropertyById", "the step names this operation");

    let operation = v2::UpdateAttachmentPropertyByID {
        attachment_id: step.text_required("attachment-id"),
        body: step.body_required(),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateAttachmentPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_blog_post<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateBlogPost", "the step names this operation");

    let operation =
        v2::UpdateBlogPost { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "updateBlogPost reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_blogpost_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateBlogpostPropertyById", "the step names this operation");

    let operation = v2::UpdateBlogpostPropertyByID {
        blogpost_id: step.number_required("blogpost-id"),
        body: step.body_required(),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateBlogpostPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_comment_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateCommentPropertyById", "the step names this operation");

    let operation = v2::UpdateCommentPropertyByID {
        body: step.body_required(),
        comment_id: step.number_required("comment-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateCommentPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_custom_content<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateCustomContent", "the step names this operation");

    let operation =
        v2::UpdateCustomContent { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "updateCustomContent reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_custom_content_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(
        step.identifier(),
        "updateCustomContentPropertyById",
        "the step names this operation",
    );

    let operation = v2::UpdateCustomContentPropertyByID {
        body: step.body_required(),
        custom_content_id: step.number_required("custom-content-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateCustomContentPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_database_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateDatabasePropertyById", "the step names this operation");

    let operation = v2::UpdateDatabasePropertyByID {
        body: step.body_required(),
        database_id: step.number_required("database-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateDatabasePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_folder_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateFolderPropertyById", "the step names this operation");

    let operation = v2::UpdateFolderPropertyByID {
        body: step.body_required(),
        folder_id: step.number_required("folder-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateFolderPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_footer_comment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateFooterComment", "the step names this operation");

    let operation = v2::UpdateFooterComment {
        body: step.body_required(),
        comment_id: step.number_required("comment-id"),
    };

    assert!(step.is_spent(), "updateFooterComment reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_inline_comment<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateInlineComment", "the step names this operation");

    let operation = v2::UpdateInlineComment {
        body: step.body_required(),
        comment_id: step.number_required("comment-id"),
    };

    assert!(step.is_spent(), "updateInlineComment reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_page<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updatePage", "the step names this operation");

    let operation = v2::UpdatePage { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "updatePage reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_page_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updatePagePropertyById", "the step names this operation");

    let operation = v2::UpdatePagePropertyByID {
        body: step.body_required(),
        page_id: step.number_required("page-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updatePagePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_page_title<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updatePageTitle", "the step names this operation");

    let operation =
        v2::UpdatePageTitle { body: step.body_required(), id: step.number_required("id") };

    assert!(step.is_spent(), "updatePageTitle reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_smart_link_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateSmartLinkPropertyById", "the step names this operation");

    let operation = v2::UpdateSmartLinkPropertyByID {
        body: step.body_required(),
        embed_id: step.number_required("embed-id"),
        property_id: step.number_required("property-id"),
    };

    assert!(step.is_spent(), "updateSmartLinkPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_space_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateSpacePropertyById", "the step names this operation");

    let operation = v2::UpdateSpacePropertyByID {
        body: step.body_required(),
        property_id: step.number_required("property-id"),
        space_id: step.number_required("space-id"),
    };

    assert!(step.is_spent(), "updateSpacePropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_space_role<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateSpaceRole", "the step names this operation");

    let operation =
        v2::UpdateSpaceRole { body: step.body_required(), id: step.text_required("id") };

    assert!(step.is_spent(), "updateSpaceRole reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn update_task<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateTask", "the step names this operation");

    let operation = v2::UpdateTask { body: step.body_required(), id: step.number_required("id") };
    let options = v2::update_task::Options { body_format: step.text("body-format") };

    assert!(step.is_spent(), "updateTask reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_whiteboard_property_by_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateWhiteboardPropertyById", "the step names this operation");

    let operation = v2::UpdateWhiteboardPropertyByID {
        body: step.body_required(),
        property_id: step.number_required("property-id"),
        whiteboard_id: step.number_required("whiteboard-id"),
    };

    assert!(step.is_spent(), "updateWhiteboardPropertyById reads every value the step holds");

    step.send_bare(client, &operation)
}
