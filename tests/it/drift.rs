use confluence::v1::shape::{
    ContentRestriction,
    ContentRestrictionRestrictions,
    GetRestrictionsByOperationReply,
    User,
    UserArray,
};
use confluence::v1::{GetCurrentUser, GetRestrictionsByOperation};
use confluence::v2::shape::{
    CheckAccessByEmailReply,
    ChildrenResponse,
    ContentProperty,
    CreateDatabaseReply,
    CreateFolderReply,
    CreateSmartLinkReply,
    CreateWhiteboardReply,
    GetDatabaseByIDReply,
    GetFolderByIDReply,
    GetFolderByIDReplyCollaborators,
    GetFolderByIDReplyDirectChildren,
    GetFolderByIDReplyOperations,
    GetFolderByIDReplyProperties,
    GetPageByIDReply,
    GetPageByIDReplyLinks,
    GetSmartLinkByIDReply,
    GetSpaceRolesByIDReply,
    GetWhiteboardByIDReply,
    Operation,
    SpaceProperty,
    SpacePropertyVersion,
    UpdateFooterCommentReply,
    UpdateFooterCommentReplyLinks,
};
use confluence::v2::{
    CheckAccessByEmail,
    CreateDatabase,
    CreateFolder,
    CreateSmartLink,
    CreateWhiteboard,
    GetDatabaseByID,
    GetFolderByID,
    GetPageByID,
    GetSmartLinkByID,
    GetSpacePropertyByID,
    GetSpaceRolesByID,
    GetWhiteboardByID,
    UpdateFooterComment,
    get_folder_by_id,
};
use serde_json::{Value, json};

use crate::fake::{answer, client, url_of};

const ACCOUNT_ID: &str = "712020:00000000-0000-4000-8000-000000000001";
const DATABASE_CREATED_MS: i64 = 1_794_035_285_002;
const EMPTY: &Value = &Value::Null;
const FOLDER_CREATED_MS: i64 = 1_794_035_284_338;
const PAGE_CREATED: &str = "2026-11-07T07:07:47.088Z";
const ROLE_ID: &str = "0c103ff5-44d0-4b2a-b80d-8690442c95f0";
const SITE_WIKI: &str = "https://example.atlassian.net/wiki";
const SMART_LINK_CREATED_MS: i64 = 1_794_035_285_375;
const WHITEBOARD_CREATED_MS: i64 = 1_794_035_284_560;

#[test]
fn a_database_reads_the_epoch_time_confluence_sends() {
    let body = json!({"createdAt": DATABASE_CREATED_MS, "id": "197124", "type": "database"});
    let confluence = client(vec![answer(200, &body), answer(200, &body)]);
    let created = confluence.send(&CreateDatabase { body: EMPTY }).expect("a created database");
    let found = confluence.send(&GetDatabaseByID { id: 197_124 }).expect("a database");
    let created_ms = created.read().and_then(|reply| reply.member(CreateDatabaseReply::CREATED_AT));
    let found_ms = found.read().and_then(|reply| reply.member(GetDatabaseByIDReply::CREATED_AT));

    assert_eq!(created_ms, Some(DATABASE_CREATED_MS), "the create reply");
    assert_eq!(found_ms, Some(DATABASE_CREATED_MS), "the read reply");
}

#[test]
fn a_folder_read_reads_each_member_its_includes_ask_for() {
    let property_sent = json!({"id": "65753", "key": "sweep-property", "value": {"n": 2_i64}});

    let body = json!({
        "collaborators": [{"accountId": ACCOUNT_ID}],
        "directChildren": {
            "_links": {"self": "/api/v2/folders/262871/direct-children"},
            "meta": {"hasMore": false},
            "results": [{"childPosition": 35_054_911_i64, "id": "262874", "type": "whiteboard"}],
        },
        "id": "262871",
        "operations": {"meta": {"hasMore": false}, "results": [{"operation": "read"}]},
        "properties": {"results": [property_sent]},
    });

    let confluence = client(vec![answer(200, &body)]);

    let options = get_folder_by_id::Options {
        include_collaborators: Some(true),
        include_direct_children: Some(true),
        include_operations: Some(true),
        include_properties: Some(true),
    };

    let found = confluence.send_with(&GetFolderByID { id: 262_871 }, &options).expect("a folder");
    let folder = found.read().expect("an object");
    let mut collaborators = folder.member(GetFolderByIDReply::COLLABORATORS).expect("a list");
    let collaborator = collaborators.next().expect("a collaborator");
    let children = folder.member(GetFolderByIDReply::DIRECT_CHILDREN).expect("children");
    let child = children.member(GetFolderByIDReplyDirectChildren::RESULTS).expect("a list").next();
    let operations = folder.member(GetFolderByIDReply::OPERATIONS).expect("operations");
    let operation = operations.member(GetFolderByIDReplyOperations::RESULTS).expect("a row").next();
    let properties = folder.member(GetFolderByIDReply::PROPERTIES).expect("properties");
    let property = properties.member(GetFolderByIDReplyProperties::RESULTS).expect("a list").next();
    let seen = confluence.network().seen();

    assert_eq!(collaborator.member(GetFolderByIDReplyCollaborators::ACCOUNT_ID), Some(ACCOUNT_ID));
    assert_eq!(child.and_then(|row| row.member(ChildrenResponse::ID)), Some("262874"), "a child");
    assert_eq!(operation.and_then(|row| row.member(Operation::OPERATION)), Some("read"), "a right");
    assert_eq!(property.and_then(|row| row.member(ContentProperty::KEY)), Some("sweep-property"));
    assert!(seen[0].url.contains("include-direct-children=true"), "the include was asked for");
}

#[test]
fn a_folder_reads_the_epoch_time_confluence_sends() {
    let body = json!({"createdAt": FOLDER_CREATED_MS, "id": "262871", "type": "folder"});
    let confluence = client(vec![answer(200, &body), answer(200, &body)]);
    let created = confluence.send(&CreateFolder { body: EMPTY }).expect("a created folder");
    let found = confluence.send(&GetFolderByID { id: 262_871 }).expect("a folder");
    let created_ms = created.read().and_then(|reply| reply.member(CreateFolderReply::CREATED_AT));
    let found_ms = found.read().and_then(|reply| reply.member(GetFolderByIDReply::CREATED_AT));

    assert_eq!(created_ms, Some(FOLDER_CREATED_MS), "the create reply");
    assert_eq!(found_ms, Some(FOLDER_CREATED_MS), "the read reply");
}

#[test]
fn a_footer_comment_update_reads_the_base_link_its_read_sibling_documents() {
    let links = json!({"base": SITE_WIKI, "webui": "/spaces/SWP/pages/163851"});
    let body = json!({"_links": links, "id": "65718", "resolutionStatus": "open"});
    let confluence = client(vec![answer(200, &body)]);
    let called = UpdateFooterComment { body: EMPTY, comment_id: 65_718 };
    let updated = confluence.send(&called).expect("an updated comment");
    let comment = updated.read().expect("an object");
    let linked = comment.member(UpdateFooterCommentReply::LINKS).expect("links");

    assert_eq!(linked.member(UpdateFooterCommentReplyLinks::BASE), Some(SITE_WIKI), "the base");
    assert_eq!(comment.member(UpdateFooterCommentReply::RESOLUTION_STATUS), Some("open"));
}

#[test]
fn a_page_keeps_the_text_time_confluence_sends_beside_them() {
    let body = json!({"createdAt": PAGE_CREATED, "id": "163851"});
    let confluence = client(vec![answer(200, &body)]);
    let found = confluence.send(&GetPageByID { id: 163_851 }).expect("a page");
    let page = found.read().expect("an object");

    assert_eq!(page.member(GetPageByIDReply::CREATED_AT), Some(PAGE_CREATED), "a page time");
    assert_eq!(page.member(GetPageByIDReply::ID), Some("163851"), "the id beside it");
}

#[test]
fn a_page_read_keeps_both_the_links_of_a_page_and_the_base_its_reply_adds() {
    let links = json!({"base": SITE_WIKI, "edituiv2": "/edit-v2/163851", "webui": "/x/163851"});
    let body = json!({"_links": links, "id": "163851"});
    let confluence = client(vec![answer(200, &body)]);
    let found = confluence.send(&GetPageByID { id: 163_851 }).expect("a page");
    let page = found.read().expect("an object");
    let linked = page.member(GetPageByIDReply::LINKS).expect("links");

    assert_eq!(linked.member(GetPageByIDReplyLinks::BASE), Some(SITE_WIKI), "the merged base");
    assert_eq!(linked.member(GetPageByIDReplyLinks::WEBUI), Some("/x/163851"), "a page link");
    assert_eq!(linked.member(GetPageByIDReplyLinks::EDITUIV2), Some("/edit-v2/163851"));
}

#[test]
fn a_smart_link_reads_the_epoch_time_confluence_sends() {
    let body = json!({"createdAt": SMART_LINK_CREATED_MS, "id": "197128", "type": "embed"});
    let confluence = client(vec![answer(200, &body), answer(200, &body)]);
    let created = confluence.send(&CreateSmartLink { body: EMPTY }).expect("a created link");
    let found = confluence.send(&GetSmartLinkByID { id: 197_128 }).expect("a smart link");

    let created_ms =
        created.read().and_then(|reply| reply.member(CreateSmartLinkReply::CREATED_AT));

    let found_ms = found.read().and_then(|reply| reply.member(GetSmartLinkByIDReply::CREATED_AT));

    assert_eq!(created_ms, Some(SMART_LINK_CREATED_MS), "the create reply");
    assert_eq!(found_ms, Some(SMART_LINK_CREATED_MS), "the read reply");
}

#[test]
fn a_space_property_reads_the_author_id_confluence_sends_for_created_by() {
    let version = json!({"authorId": ACCOUNT_ID, "message": "", "number": 1_i64});
    let body = json!({"authorId": ACCOUNT_ID, "id": "197187", "key": "sweep", "version": version});
    let confluence = client(vec![answer(200, &body)]);
    let called = GetSpacePropertyByID { property_id: 197_187, space_id: 262_518 };
    let found = confluence.send(&called).expect("a property");
    let property = found.read().expect("an object");
    let held = property.member(SpaceProperty::VERSION).expect("a version");

    assert_eq!(property.member(SpaceProperty::AUTHOR_ID), Some(ACCOUNT_ID), "the property");
    assert_eq!(held.member(SpacePropertyVersion::AUTHOR_ID), Some(ACCOUNT_ID), "its version");
}

#[test]
fn a_space_role_is_read_by_the_text_id_confluence_gives_it() {
    let body = json!({"id": ROLE_ID, "name": "Collaborator", "type": "SYSTEM"});
    let confluence = client(vec![answer(200, &body)]);
    let found = confluence.send(&GetSpaceRolesByID { id: ROLE_ID }).expect("a role");
    let role = found.read().expect("an object");
    let seen = confluence.network().seen();

    assert_eq!(seen[0].url, url_of(&format!("/wiki/api/v2/space-roles/{ROLE_ID}")), "the id");
    assert_eq!(role.member(GetSpaceRolesByIDReply::NAME), Some("Collaborator"), "the role");
}

#[test]
fn a_user_reads_the_status_and_locale_confluence_sends() {
    let body = json!({"accountId": ACCOUNT_ID, "accountStatus": "active", "locale": "en_US"});
    let confluence = client(vec![answer(200, &body)]);
    let found = confluence.send(&GetCurrentUser).expect("a user");
    let user = found.read().expect("an object");

    assert_eq!(user.member(User::ACCOUNT_STATUS), Some("active"), "the account status");
    assert_eq!(user.member(User::LOCALE), Some("en_US"), "the locale");
    assert_eq!(user.member(User::ACCOUNT_ID), Some(ACCOUNT_ID), "the account beside them");
}

#[test]
fn a_whiteboard_reads_the_epoch_time_confluence_sends() {
    let body = json!({"createdAt": WHITEBOARD_CREATED_MS, "id": "262874", "type": "whiteboard"});
    let confluence = client(vec![answer(200, &body), answer(200, &body)]);
    let created = confluence.send(&CreateWhiteboard { body: EMPTY }).expect("a created board");
    let found = confluence.send(&GetWhiteboardByID { id: 262_874 }).expect("a whiteboard");

    let created_ms =
        created.read().and_then(|reply| reply.member(CreateWhiteboardReply::CREATED_AT));

    let found_ms = found.read().and_then(|reply| reply.member(GetWhiteboardByIDReply::CREATED_AT));

    assert_eq!(created_ms, Some(WHITEBOARD_CREATED_MS), "the create reply");
    assert_eq!(found_ms, Some(WHITEBOARD_CREATED_MS), "the read reply");
}

#[test]
fn an_access_check_reads_the_emails_confluence_names_no_access() {
    let body = json!({"invalidEmails": [], "noAccessEmails": ["sweep-absent@example.invalid"]});
    let confluence = client(vec![answer(200, &body)]);
    let checked = confluence.send(&CheckAccessByEmail { body: EMPTY }).expect("a check");
    let reply = checked.read().expect("an object");
    let denied = reply.member(CheckAccessByEmailReply::NO_ACCESS_EMAILS).expect("a list");
    let invalid = reply.member(CheckAccessByEmailReply::INVALID_EMAILS).expect("a list");
    let named: Vec<&str> = denied.collect();

    assert_eq!(named, ["sweep-absent@example.invalid"], "the address without access");
    assert_eq!(invalid.count(), 0, "no address was malformed");
}

#[test]
fn restrictions_by_operation_read_each_operation_confluence_keys_them_by() {
    let named_user = json!({"accountId": ACCOUNT_ID});
    let users = json!({"limit": 200_i64, "results": [named_user], "size": 1_i64, "start": 0_i64});
    let none = json!({"limit": 200_i64, "results": [], "size": 0_i64, "start": 0_i64});
    let read = json!({"operation": "read", "restrictions": {"group": none, "user": none}});
    let update = json!({"operation": "update", "restrictions": {"group": none, "user": users}});
    let body = json!({"_links": {"base": SITE_WIKI}, "read": read, "update": update});
    let confluence = client(vec![answer(200, &body)]);
    let found = confluence.send(&GetRestrictionsByOperation { id: "197540" }).expect("a reply");
    let reply = found.read().expect("an object");
    let reading = reply.member(GetRestrictionsByOperationReply::READ).expect("the read entry");
    let updating = reply.member(GetRestrictionsByOperationReply::UPDATE).expect("the update entry");
    let restrictions = updating.member(ContentRestriction::RESTRICTIONS).expect("restrictions");
    let named = restrictions.member(ContentRestrictionRestrictions::USER).expect("users");
    let user = named.member(UserArray::RESULTS).expect("a list").next().expect("a user");

    assert_eq!(reading.member(ContentRestriction::OPERATION), Some("read"), "the read entry");
    assert_eq!(updating.member(ContentRestriction::OPERATION), Some("update"), "the update entry");
    assert_eq!(user.member(User::ACCOUNT_ID), Some(ACCOUNT_ID), "the user the update names");
}
