use crate::entities::{ReportId, WarningPresetId};
use derive_builder::Builder;
use derive_is_enum_variant::is_enum_variant;
use serde_with::skip_serializing_none;

/// Form used to perform an admin action on an account and resolve any open reports
///
/// // Example
///
/// ```
/// use mastodon_async::requests::admin::{AccountAction, AccountActionRequest};
/// let request = AccountActionRequest::builder(AccountAction::Silence).text("Hush now").build();
/// ```
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Builder)]
#[builder(
    derive(Debug, PartialEq),
    custom_constructor,
    build_fn(private, name = "try_build"),
    setter(into, strip_option)
)]
pub struct AccountActionRequest {
    /// The type of action to be taken.
    #[builder(private)]
    #[serde(rename = "type")]
    pub action: AccountAction,
    /// The ID of an associated report that caused this action to be taken.
    #[builder(default)]
    pub report_id: Option<ReportId>,
    /// The ID of a preset warning.
    #[builder(default)]
    pub warning_preset_id: Option<WarningPresetId>,
    /// Additional clarification for why this action was taken.
    #[builder(default)]
    pub text: Option<String>,
    /// Should an email be sent to the user with the above information?
    #[builder(default)]
    pub send_email_notification: Option<bool>,
}

impl AccountActionRequest {
    /// Start building a form for performing an admin action on a report.
    pub fn builder(action: AccountAction) -> AccountActionRequestBuilder {
        let mut builder = AccountActionRequestBuilder::create_empty();
        builder.action(action);
        builder
    }
}

impl AccountActionRequestBuilder {
    /// Build the form for performing an admin action on a report.
    pub fn build(&self) -> AccountActionRequest {
        self.try_build()
            .expect("One or more required fields are missing!")
    }
}

/// Action to be performed on the account.
/// https://docs.joinmastodon.org/methods/admin/accounts/#form-data-parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, is_enum_variant)]
#[serde(rename_all = "snake_case")]
pub enum AccountAction {
    /// No action. Can be used to resolve any open reports against the account.
    None,
    /// Force the account's statuses to be marked as containing sensitive media.
    Sensitive,
    /// Prevent the account from logging in.
    Disable,
    /// Silence the account.
    Silence,
    /// Suspend the account.
    Suspend,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialize_action_request() {
        let request = AccountActionRequest::builder(AccountAction::Suspend)
            .report_id(ReportId::new("666"))
            .text("you know what you did")
            .build();
        let ser = serde_json::to_string(&request).expect("Couldn't serialize");
        assert_eq!(
            ser,
            r#"{"type":"suspend","report_id":"666","text":"you know what you did"}"#
        );
    }
}
