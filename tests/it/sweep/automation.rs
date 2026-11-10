use confluence::automation;
use confluence::{Client, Error, Transport};

use crate::step::{Run, Sent, Step};

pub(crate) trait Operations: Transport + Sized {
    const ALL: [(&'static str, Run<Self>); 15] = [
        ("createRule", create_rule::<Self>),
        ("createRuleFromTemplate", create_rule_from_template::<Self>),
        ("deleteDisabledRule", delete_disabled_rule::<Self>),
        ("getRuleByUuid", get_rule_by_uuid::<Self>),
        ("getTemplateByTemplateId", get_template_by_template_id::<Self>),
        ("invokeManualRule", invoke_manual_rule::<Self>),
        ("listRuleSummaries", list_rule_summaries::<Self>),
        ("searchManualRulesWithGet", search_manual_rules_with_get::<Self>),
        ("searchManualRulesWithPost", search_manual_rules_with_post::<Self>),
        ("searchRuleSummaries", search_rule_summaries::<Self>),
        ("searchTemplatesWithGet", search_templates_with_get::<Self>),
        ("searchTemplatesWithPost", search_templates_with_post::<Self>),
        ("updateRule", update_rule::<Self>),
        ("updateRuleScope", update_rule_scope::<Self>),
        ("updateRuleState", update_rule_state::<Self>),
    ];
}

impl<Network> Operations for Network where Network: Transport {}

pub(crate) fn create_rule<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createRule", "the step names this operation");

    let operation = automation::CreateRule;
    let options = automation::create_rule::Options { body: step.body() };

    assert!(step.is_spent(), "createRule reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn create_rule_from_template<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "createRuleFromTemplate", "the step names this operation");

    let operation = automation::CreateRuleFromTemplate;
    let options = automation::create_rule_from_template::Options { body: step.body() };

    assert!(step.is_spent(), "createRuleFromTemplate reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn delete_disabled_rule<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "deleteDisabledRule", "the step names this operation");

    let operation = automation::DeleteDisabledRule { rule_uuid: step.text_required("ruleUuid") };

    assert!(step.is_spent(), "deleteDisabledRule reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn get_rule_by_uuid<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getRuleByUuid", "the step names this operation");

    let operation = automation::GetRuleByUUID { rule_uuid: step.text_required("ruleUuid") };

    let options = automation::get_rule_by_uuid::Options {
        redact_sensitive_fields: step.boolean("redactSensitiveFields"),
    };

    assert!(step.is_spent(), "getRuleByUuid reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn get_template_by_template_id<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "getTemplateByTemplateId", "the step names this operation");

    let operation =
        automation::GetTemplateByTemplateID { template_id: step.text_required("templateId") };

    assert!(step.is_spent(), "getTemplateByTemplateId reads every value the step holds");

    step.send_bare(client, &operation)
}

pub(crate) fn invoke_manual_rule<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "invokeManualRule", "the step names this operation");

    let operation = automation::InvokeManualRule { rule_id: step.text_required("ruleId") };
    let options = automation::invoke_manual_rule::Options { body: step.body() };

    assert!(step.is_spent(), "invokeManualRule reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn list_rule_summaries<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "listRuleSummaries", "the step names this operation");

    let operation = automation::ListRuleSummaries;

    let options = automation::list_rule_summaries::Options {
        cursor: step.text("cursor"),
        limit: step.text("limit"),
    };

    assert!(step.is_spent(), "listRuleSummaries reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn search_manual_rules_with_get<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchManualRulesWithGet", "the step names this operation");

    let operation = automation::SearchManualRulesWithGet { cursor: step.text_required("cursor") };
    let options = automation::search_manual_rules_with_get::Options { limit: step.text("limit") };

    assert!(step.is_spent(), "searchManualRulesWithGet reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn search_manual_rules_with_post<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchManualRulesWithPost", "the step names this operation");

    let operation = automation::SearchManualRulesWithPost;
    let options = automation::search_manual_rules_with_post::Options { body: step.body() };

    assert!(step.is_spent(), "searchManualRulesWithPost reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn search_rule_summaries<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchRuleSummaries", "the step names this operation");

    let operation = automation::SearchRuleSummaries;
    let options = automation::search_rule_summaries::Options { body: step.body() };

    assert!(step.is_spent(), "searchRuleSummaries reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn search_templates_with_get<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchTemplatesWithGet", "the step names this operation");

    let operation = automation::SearchTemplatesWithGet;

    let options = automation::search_templates_with_get::Options {
        categories: step.texts("categories"),
        cursor: step.text("cursor"),
        limit: step.text("limit"),
        rule_home: step.text("ruleHome"),
    };

    assert!(step.is_spent(), "searchTemplatesWithGet reads every value the step holds");

    if step.is_walk() {
        return step.walk(client, &operation, options);
    }

    step.send(client, &operation, &options)
}

pub(crate) fn search_templates_with_post<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "searchTemplatesWithPost", "the step names this operation");

    let operation = automation::SearchTemplatesWithPost;
    let options = automation::search_templates_with_post::Options { body: step.body() };

    assert!(step.is_spent(), "searchTemplatesWithPost reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_rule<Network>(client: &Client<Network>, step: &Step<'_>) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateRule", "the step names this operation");

    let operation = automation::UpdateRule { rule_uuid: step.text_required("ruleUuid") };
    let options = automation::update_rule::Options { body: step.body() };

    assert!(step.is_spent(), "updateRule reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_rule_scope<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateRuleScope", "the step names this operation");

    let operation = automation::UpdateRuleScope { rule_uuid: step.text_required("ruleUuid") };
    let options = automation::update_rule_scope::Options { body: step.body() };

    assert!(step.is_spent(), "updateRuleScope reads every value the step holds");

    step.send(client, &operation, &options)
}

pub(crate) fn update_rule_state<Network>(
    client: &Client<Network>,
    step: &Step<'_>,
) -> Result<Sent, Error>
where
    Network: Transport,
{
    assert_eq!(step.identifier(), "updateRuleState", "the step names this operation");

    let operation = automation::UpdateRuleState { rule_uuid: step.text_required("ruleUuid") };
    let options = automation::update_rule_state::Options { body: step.body() };

    assert!(step.is_spent(), "updateRuleState reads every value the step holds");

    step.send(client, &operation, &options)
}
