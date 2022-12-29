//! Module containing all info relating to a status.

use super::prelude::*;
use crate::{entities::card::Card, status_builder::Visibility};
use serde::{Deserialize, Serialize};
use time::{serde::iso8601, OffsetDateTime};

/// A status from the instance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    /// The ID of the status.
    pub id: String,
    /// A Fediverse-unique resource ID.
    pub uri: String,
    /// URL to the status page (can be remote)
    pub url: Option<String>,
    /// The Account which posted the status.
    pub account: Account,
    /// The ID of the status this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_id: Option<String>,
    /// The ID of the account this status is replying to, if the status is
    /// a reply.
    pub in_reply_to_account_id: Option<String>,
    /// If this status is a reblogged Status of another User.
    pub reblog: Option<Box<Status>>,
    /// Body of the status; this will contain HTML
    /// (remote HTML already sanitized)
    pub content: String,
    /// The time the status was created.
    #[serde(with = "iso8601")]
    pub created_at: OffsetDateTime,
    /// An array of Emoji
    pub emojis: Vec<Emoji>,
    /// The numbef or replies to this status.
    pub replies_count: Option<u64>,
    /// The number of reblogs for the status.
    pub reblogs_count: u64,
    /// The number of favourites for the status.
    pub favourites_count: u64,
    /// Whether the application client has reblogged the status.
    pub reblogged: Option<bool>,
    /// Whether the application client has favourited the status.
    pub favourited: Option<bool>,
    /// Whether media attachments should be hidden by default.
    pub sensitive: bool,
    /// If not empty, warning text that should be displayed before the actual
    /// content.
    pub spoiler_text: String,
    /// The visibilty of the status.
    pub visibility: Visibility,
    /// An array of attachments.
    pub media_attachments: Vec<Attachment>,
    /// An array of mentions.
    pub mentions: Vec<Mention>,
    /// An array of tags.
    pub tags: Vec<Tag>,
    /// The associated card
    pub card: Option<Card>,
    /// Name of application used to post status.
    pub application: Option<Application>,
    /// The detected language for the status, if detected.
    pub language: Option<String>,
    /// Whether this is the pinned status for the account that posted it.
    pub pinned: Option<bool>,
}

/// A mention of another user.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Mention {
    /// URL of user's profile (can be remote).
    pub url: String,
    /// The username of the account.
    pub username: String,
    /// Equals `username` for local users, includes `@domain` for remote ones.
    pub acct: String,
    /// Account ID.
    pub id: String,
}

/// Struct representing an emoji within text.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Emoji {
    /// The shortcode of the emoji
    pub shortcode: String,
    /// URL to the emoji static image
    pub static_url: String,
    /// URL to the emoji image
    pub url: String,
}

/// Hashtags in the status. This functions both as a
/// [`Status::Tag`](https://docs.joinmastodon.org/entities/Status/#Tag), and
/// as a [`Tag`](https://docs.joinmastodon.org/entities/Tag/). In the case of
/// the former, at the time of writing, the history field is always empty and
/// the following field is always none.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    /// The hashtag, not including the preceding `#`.
    pub name: String,
    /// The URL of the hashtag.
    pub url: String,
    /// Usage statistics for given days (typically the past week).
    #[serde(default = "Vec::new")]
    pub history: Vec<TagHistory>,
    /// Whether the current token’s authorized user is following this tag.
    pub following: Option<bool>,
}

/// Application details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Application {
    /// Name of the application.
    pub name: String,
    /// Homepage URL of the application.
    pub website: Option<String>,
}

/// Usage statistics for given days (typically the past week).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagHistory {
    /// UNIX timestamp on midnight of the given day.
    pub day: String,
    /// The counted usage of the tag within that day.
    pub uses: String,
    /// The total of accounts using the tag within that day.
    pub accounts: String,
}
