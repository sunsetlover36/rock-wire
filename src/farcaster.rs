use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, EnumString};

// ---- Responses
#[derive(Debug, Clone, Serialize)]
pub struct WebhookPayload {
    pub created_at: u64,
    pub event: WebhookEvent,
}
impl<'de> Deserialize<'de> for WebhookPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RawPayload {
            created_at: u64,
            #[serde(rename = "type")]
            event_type: String,
            data: serde_json::Value,
        }

        let raw = RawPayload::deserialize(deserializer)?;
        let event = match raw.event_type.as_str() {
            "cast.created" => {
                let data: CastCreatedData =
                    serde_json::from_value(raw.data).map_err(serde::de::Error::custom)?;
                WebhookEvent::CastCreated(data)
            }
            other => {
                return Err(serde::de::Error::custom(format!(
                    "unknown event type: {other}"
                )));
            }
        };

        Ok(WebhookPayload {
            created_at: raw.created_at,
            event,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebhookEvent {
    #[serde(rename = "cast.created")]
    CastCreated(CastCreatedData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastCreatedData {
    pub object: String,
    pub hash: String,
    pub author: User,
    pub app: Option<UserDehydrated>,
    pub thread_hash: String,

    pub parent_hash: Option<String>,
    pub parent_url: Option<String>,
    pub root_parent_url: Option<String>,

    pub parent_author: Author,
    pub text: String,
    pub timestamp: String,
    pub embeds: Vec<serde_json::Value>,
    pub channel: Option<serde_json::Value>,
    pub reactions: CastReactionsSummary,
    pub replies: Replies,
    pub mentioned_profiles: Vec<User>,
    pub mentioned_profiles_ranges: Vec<Range>,
    pub mentioned_channels: Vec<ChannelDehydrated>,
    pub mentioned_channels_ranges: Vec<Range>,
    pub event_timestamp: String,
}

pub type Fid = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub fid: Option<Fid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastReactionPreview {
    pub fid: Fid,
    pub fname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastReactionsSummary {
    pub likes_count: u64,
    pub recasts_count: u64,

    #[serde(default)]
    pub likes: Vec<CastReactionPreview>,

    #[serde(default)]
    pub recasts: Vec<CastReactionPreview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Replies {
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub object: String,
    pub fid: Fid,
    pub username: String,
    pub display_name: Option<String>,
    pub pfp_url: Option<String>,
    pub custody_address: String,
    pub registered_at: String,
    pub pro: Option<Pro>,
    pub profile: Profile,
    pub follower_count: u64,
    pub following_count: u64,
    pub verifications: Vec<String>,
    pub verified_addresses: VerifiedAddresses,
    pub auth_addresses: Vec<AuthAddress>,
    pub verified_accounts: Vec<VerifiedAccount>,
    pub url: Option<String>,
    pub experimental: Option<Experimental>,
    pub viewer_context: Option<ViewerContext>,

    pub score: Option<f64>,
}

fn deserialize_optional_fid<'de, D>(deserializer: D) -> Result<Option<Fid>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;

    match value {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Number(n)) => n
            .as_u64()
            .ok_or_else(|| serde::de::Error::custom("invalid fid number"))
            .map(Some),
        Some(serde_json::Value::String(s)) => {
            s.parse::<Fid>().map(Some).map_err(serde::de::Error::custom)
        }
        Some(other) => Err(serde::de::Error::custom(format!(
            "invalid fid value: {other}"
        ))),
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDehydrated {
    pub object: Option<String>,

    #[serde(deserialize_with = "deserialize_optional_fid")]
    pub fid: Option<Fid>,

    pub username: Option<String>,
    pub display_name: Option<String>,
    pub pfp_url: Option<String>,
    pub custody_address: Option<String>,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pro {
    pub status: Option<String>,
    pub subscribed_at: Option<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub bio: Bio,

    pub location: Option<Location>,
    pub banner: Option<Banner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bio {
    pub text: String,

    #[serde(default)]
    pub mentioned_profiles: Vec<UserDehydrated>,

    #[serde(default)]
    pub mentioned_profiles_ranges: Vec<Range>,

    #[serde(default)]
    pub mentioned_channels: Vec<ChannelDehydrated>,

    #[serde(default)]
    pub mentioned_channels_ranges: Vec<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelDehydrated {
    pub object: String,
    pub id: String,
    pub name: String,
    pub image_url: Option<String>,
    pub viewer_context: Option<ChannelViewerContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,

    #[serde(default)]
    pub address: Option<Address>,

    pub radius: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub city: Option<String>,
    pub state: Option<String>,
    pub state_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Banner {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAddresses {
    pub eth_addresses: Vec<String>,
    pub sol_addresses: Vec<String>,
    pub primary: PrimaryAddresses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryAddresses {
    pub eth_address: Option<String>,
    pub sol_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthAddress {
    pub address: String,
    pub app: Option<UserDehydrated>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedAccount {
    pub platform: Option<String>,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experimental {
    pub neynar_user_score: f64,
    pub deprecation_notice: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedCast {
    pub hash: String,
    pub author: Author,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewerContext {
    pub following: bool,
    pub followed_by: bool,
    pub blocking: bool,
    pub blocked_by: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelViewerContext {
    pub following: bool,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastViewerContext {
    pub liked: bool,
    pub recasted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub object: String,
    pub id: String,
    pub name: String,
    pub url: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub follower_count: Option<u64>,
    pub member_count: Option<u64>,
    pub parent_url: Option<String>,
    pub pinned_cast_hash: Option<String>,
    pub viewer_context: Option<ChannelViewerContext>,
    pub lead: Option<Box<User>>,
    pub moderator: Option<Box<User>>,

    #[serde(default)]
    pub hosts: Vec<User>,

    #[serde(default)]
    pub moderator_fids: Vec<Fid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub object: String,
    pub hash: String,
    pub author: User,
    pub channel: Option<Channel>,

    #[serde(default)]
    pub embeds: Vec<serde_json::Value>,

    pub thread_hash: String,
    pub parent_hash: Option<String>,
    pub parent_url: Option<String>,
    pub root_parent_url: Option<String>,

    pub parent_author: Author,
    pub text: String,
    pub timestamp: String,

    pub reactions: CastReactionsSummary,
    pub replies: Replies,

    #[serde(default)]
    pub mentioned_profiles: Vec<User>,

    #[serde(default)]
    pub mentioned_profiles_ranges: Vec<Range>,

    #[serde(default)]
    pub mentioned_channels: Vec<ChannelDehydrated>,

    #[serde(default)]
    pub mentioned_channels_ranges: Vec<Range>,

    pub app: Option<UserDehydrated>,

    pub author_channel_context: Option<ChannelViewerContext>,
    pub viewer_context: Option<CastViewerContext>,
    pub r#type: Option<String>,
}

// -- Get cast response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCastResponse {
    pub cast: Cast,
}
// --

// -- Bulk fetch casts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkFetchCastsResponse {
    pub result: BulkFetchCastsResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkFetchCastsResult {
    pub casts: Vec<Cast>,
}
// --

// -- Cast conversation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastConversationResponse {
    pub conversation: CastConversation,
    pub next: Option<NextCursor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastConversation {
    pub cast: ConversationCast,

    #[serde(default)]
    pub chronological_parent_casts: Vec<Cast>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationCast {
    #[serde(flatten)]
    pub cast: Cast,

    #[serde(default)]
    pub direct_replies: Vec<ConversationCast>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextCursor {
    pub cursor: Option<String>,
}
// --

// -- Reactions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionsResponse {
    pub next: Option<NextCursor>,

    #[serde(default)]
    pub reactions: Vec<Reaction>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReactionKind {
    Like,
    Recast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub object: String,
    pub reaction_timestamp: String,
    pub reaction_type: ReactionKind,
    pub user: User,
    pub app: Option<UserDehydrated>,
}

// -- Send cast response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCastResponse {
    pub success: bool,
    pub cast: CreatedCast,
}
// --

// -- Delete cast response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCastResponse {
    pub success: bool,
    pub message: String,
}
// --

// -- Reaction response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionRecord {
    pub hash: String,
    pub target: String,

    #[serde(rename = "type")]
    pub kind: ReactionKind,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionResponse {
    pub success: bool,
    pub message: Option<String>,
    pub reaction: Option<ReactionRecord>,
}
// --

// -- Get user by username response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByUsernameResponse {
    pub user: User,
}
// --

// -- Get users by FIDs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersByFidsResponse {
    pub users: Vec<User>,
    pub next: Option<NextCursor>,
}
// --

// -- Search users response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    pub result: SearchUsersResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersResult {
    #[serde(default)]
    pub users: Vec<User>,

    pub next: Option<NextCursor>,
}
// --

// -- User casts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCastsResponse {
    #[serde(default)]
    pub casts: Vec<Cast>,

    pub next: Option<NextCursor>,
}
// --

// -- Signer response
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SignerStatus {
    Generated,
    PendingApproval,
    Approved,
    Revoked,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr, EnumString)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SignerPermission {
    WriteAll,
    ReadOnly,
    None,
    PublishCast,
    DeleteCast,
    PublishReaction,
    DeleteReaction,
    UpdateProfile,
    FollowUser,
    UnfollowUser,
    FollowChannel,
    UnfollowChannel,
    AddVerification,
    RemoveVerification,
    WriteFrameAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerResponse {
    pub public_key: String,
    pub signer_uuid: String,
    pub status: SignerStatus,
    pub fid: Option<Fid>,
    pub object: Option<String>,

    #[serde(default)]
    pub permissions: Vec<SignerPermission>,
    pub signer_approval_url: Option<String>,
}
// --

// -- Follow response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUserResponse {
    pub details: Vec<FollowUserResponseDetail>,
    pub success: bool,
}

pub type UnfollowUserResponse = FollowUserResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUserResponseDetail {
    pub hash: String,
    pub success: bool,
    pub target_fid: Fid,
}
// --

// -- Feed response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedResponse {
    #[serde(default)]
    pub casts: Vec<Cast>,

    pub next: Option<NextCursor>,
}

pub type ForYouFeedResponse = FeedResponse;
pub type FollowingFeedResponse = FeedResponse;
pub type FilteredFeedResponse = FeedResponse;
pub type TrendingFeedResponse = FeedResponse;
pub type ChannelFeedResponse = FeedResponse;
// --

// -- Followers responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follower {
    pub object: String,
    pub user: User,
    pub app: Option<UserDehydrated>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowerDehydrated {
    pub object: String,
    pub user: UserDehydrated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowersResponse {
    pub next: Option<NextCursor>,

    #[serde(default)]
    pub users: Vec<Follower>,
}

pub type FollowingResponse = FollowersResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelevantFollowersResponse {
    #[serde(default)]
    pub all_relevant_followers_dehydrated: Vec<FollowerDehydrated>,

    #[serde(default)]
    pub top_relevant_followers_hydrated: Vec<Follower>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReciprocalFollower {
    pub object: String,
    pub timestamp: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReciprocalFollowersResponse {
    pub next: Option<NextCursor>,

    #[serde(default)]
    pub users: Vec<ReciprocalFollower>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowSuggestionsResponse {
    pub next: Option<NextCursor>,

    #[serde(default)]
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestFriend {
    pub fid: Fid,
    pub mutual_affinity_score: f64,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestFriendsResponse {
    #[serde(default)]
    pub users: Vec<BestFriend>,
    pub next: Option<NextCursor>,
}
// --

// -- Notifications response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationsResponse {
    pub next: Option<NextCursor>,

    // FIXME: needs type or a proper response check from Neynar
    #[serde(default)]
    pub notifications: Vec<serde_json::Value>,
    pub unseen_notifications_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub object: Option<String>,

    #[serde(rename = "type")]
    pub kind: NotificationKind,

    pub most_recent_timestamp: String,

    #[serde(default)]
    pub seen: bool,

    pub count: Option<u64>,
    pub cast: Option<Cast>,

    #[serde(default)]
    pub follows: Vec<NotificationFollow>,

    #[serde(default)]
    pub reactions: Vec<NotificationReaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Eq, PartialEq, Hash, AsRefStr, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum NotificationFilterKind {
    Follows,
    Recasts,
    Likes,
    Mentions,
    Replies,
    Quotes,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Eq, PartialEq, Hash, AsRefStr, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum NotificationKind {
    Follow,
    Follows,
    Recast,
    Recasts,
    Like,
    Likes,
    Mention,
    Mentions,
    Reply,
    Replies,
    Quote,
    Quotes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationFollow {
    pub object: Option<String>,
    pub user: User,
    pub app: Option<UserDehydrated>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationReaction {
    pub object: Option<String>,
    pub user: User,

    // FIXME: needs type or a proper response check from Neynar
    pub cast: serde_json::Value,
}
// --
// ----

// ---- Requests
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
#[serde(rename_all = "lowercase")]
pub enum CastIdentifierKind {
    Url,
    Hash,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCastParams {
    pub identifier: String,

    #[serde(rename = "type")]
    pub id_type: CastIdentifierKind,

    pub viewer_fid: Option<Fid>,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr, EnumString, EnumIter,
)]
#[serde(rename_all = "lowercase")]
pub enum CastSortKind {
    Trending,
    Likes,
    Recasts,
    Replies,
    Recent,
}

// -- Bulk fetch casts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkFetchCastsParams {
    pub casts: Vec<String>,
    pub sort_type: CastSortKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkFetchCastsRawQuery {
    pub casts: String,
    pub sort_type: CastSortKind,
}
impl From<&BulkFetchCastsParams> for BulkFetchCastsRawQuery {
    fn from(params: &BulkFetchCastsParams) -> Self {
        Self {
            casts: params.casts.join(","),
            sort_type: params.sort_type,
        }
    }
}
// --

// -- Cast conversation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum ConversationSortKind {
    Chron,
    DescChron,
    Algorithmic,
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum ConversationFoldKind {
    Above,
    Below,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CastConversationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_depth: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_chronological_parent_casts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<ConversationSortKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fold: Option<ConversationFoldKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCastConversationParams {
    pub identifier: String,

    #[serde(rename = "type")]
    pub id_type: CastIdentifierKind,

    #[serde(flatten)]
    pub options: CastConversationOptions,
}
impl GetCastConversationParams {
    pub fn new(
        identifier: String,
        id_type: CastIdentifierKind,
        options: CastConversationOptions,
    ) -> Self {
        Self {
            identifier,
            id_type,
            options,
        }
    }
}
// --

// -- Reactions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetReactionsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReactionFilter {
    All,
    Likes,
    Recasts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReactionsParams {
    pub hash: String,
    pub types: Vec<ReactionFilter>,

    #[serde(flatten)]
    pub options: GetReactionsOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReactionsRawQuery {
    pub hash: String,
    pub types: String,
    pub viewer_fid: Option<Fid>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
}
impl From<&GetReactionsParams> for GetReactionsRawQuery {
    fn from(params: &GetReactionsParams) -> Self {
        Self {
            hash: params.hash.clone(),
            types: params
                .types
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<_>>()
                .join(","),
            viewer_fid: params.options.viewer_fid,
            limit: params.options.limit,
            cursor: params.options.cursor.clone(),
        }
    }
}
// --

// -- Cast reactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishReactionParams {
    pub reaction_type: ReactionKind,
    pub signer_uuid: String,
    pub target: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idem: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_author_fid: Option<Fid>,
}
pub type DeleteReactionParams = PublishReactionParams;
// --

// -- Send cast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastEmbedCastId {
    pub fid: u32,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CastEmbed {
    Url { url: String },
    CastId { cast_id: CastEmbedCastId },
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendCastParams {
    pub signer_uuid: String,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<CastEmbed>>,
}
// --

// -- Delete cast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCastParams {
    pub signer_uuid: String,
    pub target_hash: String,
}
// --

// -- Get users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserByUsernameParams {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUsersByFidsParams {
    pub fids: Vec<Fid>,
}
// --

// -- Search users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersParams {
    pub q: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
// --

// -- Get user casts
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserCastsParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_replies: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}
// --

// -- Follows
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUserParams {
    pub signer_uuid: String,
    pub target_fids: Vec<Fid>,
}

pub type UnfollowUserParams = FollowUserParams;
// --

// -- Followers and following
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FollowSortKind {
    DescChron,
    Algorithmic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFollowersParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<FollowSortKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetFollowersParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_follow_page(self.fid, self.viewer_fid, self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRelevantFollowersParams {
    pub target_fid: Fid,
    pub viewer_fid: Fid,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetRelevantFollowersParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.target_fid == 0 {
            return Err("target_fid must be positive");
        }
        if self.viewer_fid == 0 {
            return Err("viewer_fid must be positive");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetReciprocalFollowersParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<FollowSortKind>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetReciprocalFollowersParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_follow_page(self.fid, self.viewer_fid, self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFollowingParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<FollowSortKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetFollowingParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_follow_page(self.fid, self.viewer_fid, self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFollowSuggestionsParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetFollowSuggestionsParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_follow_page(self.fid, self.viewer_fid, self.limit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBestFriendsParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
impl GetBestFriendsParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_follow_page(self.fid, None, self.limit)
    }
}

fn validate_follow_page(
    fid: Fid,
    viewer_fid: Option<Fid>,
    limit: Option<u8>,
) -> Result<(), &'static str> {
    if fid == 0 {
        return Err("fid must be positive");
    }
    if viewer_fid == Some(0) {
        return Err("viewer_fid must be positive");
    }
    if limit.is_some_and(|limit| !(1..=100).contains(&limit)) {
        return Err("limit must be between 1 and 100");
    }
    Ok(())
}
// --

// Feeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetForYouFeedParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFollowingFeedParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_recasts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

// -- Filtered feed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeedFilterKind {
    Fids,
    ParentUrl,
    ChannelId,
    EmbedUrl,
    EmbedTypes,
    GlobalTrending,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
pub enum EmbedType {
    #[serde(rename = "text")]
    #[strum(serialize = "text")]
    Text,
    #[serde(rename = "image")]
    #[strum(serialize = "image")]
    Image,
    #[serde(rename = "video")]
    #[strum(serialize = "video")]
    Video,
    #[serde(rename = "audio")]
    #[strum(serialize = "audio")]
    Audio,
    #[serde(rename = "text/html")]
    #[strum(serialize = "text/html")]
    TextHtml,
    #[serde(rename = "text/plain")]
    #[strum(serialize = "text/plain")]
    TextPlain,
    #[serde(rename = "image/jpeg")]
    #[strum(serialize = "image/jpeg")]
    ImageJpeg,
    #[serde(rename = "image/png")]
    #[strum(serialize = "image/png")]
    ImagePng,
    #[serde(rename = "image/gif")]
    #[strum(serialize = "image/gif")]
    ImageGif,
    #[serde(rename = "image/webp")]
    #[strum(serialize = "image/webp")]
    ImageWebp,
    #[serde(rename = "image/svg+xml")]
    #[strum(serialize = "image/svg+xml")]
    ImageSvgXml,
    #[serde(rename = "image/heif")]
    #[strum(serialize = "image/heif")]
    ImageHeif,
    #[serde(rename = "video/mp4")]
    #[strum(serialize = "video/mp4")]
    VideoMp4,
    #[serde(rename = "video/quicktime")]
    #[strum(serialize = "video/quicktime")]
    VideoQuicktime,
    #[serde(rename = "audio/mpeg")]
    #[strum(serialize = "audio/mpeg")]
    AudioMpeg,
    #[serde(rename = "application/pdf")]
    #[strum(serialize = "application/pdf")]
    ApplicationPdf,
    #[serde(rename = "application/json")]
    #[strum(serialize = "application/json")]
    ApplicationJson,
    #[serde(rename = "application/x-mpegurl")]
    #[strum(serialize = "application/x-mpegurl")]
    ApplicationXMpegurl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "filter_type", rename_all = "snake_case")]
pub enum FeedFilter {
    Fids {
        fids: Vec<Fid>,
    },
    ParentUrl {
        parent_url: String,
    },
    ChannelId {
        channel_id: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        members_only: Option<bool>,
    },
    EmbedUrl {
        embed_url: String,
    },
    EmbedTypes {
        embed_types: Vec<EmbedType>,
    },
    GlobalTrending,
}
impl FeedFilter {
    pub fn kind(&self) -> FeedFilterKind {
        match self {
            Self::Fids { .. } => FeedFilterKind::Fids,
            Self::ParentUrl { .. } => FeedFilterKind::ParentUrl,
            Self::ChannelId { .. } => FeedFilterKind::ChannelId,
            Self::EmbedUrl { .. } => FeedFilterKind::EmbedUrl,
            Self::EmbedTypes { .. } => FeedFilterKind::EmbedTypes,
            Self::GlobalTrending => FeedFilterKind::GlobalTrending,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeedByFiltersParams {
    #[serde(flatten)]
    pub filter: FeedFilter,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_recasts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetFeedByFiltersParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        match &self.filter {
            FeedFilter::Fids { fids } => {
                if fids.is_empty() || fids.len() > 100 {
                    return Err("fids must contain between 1 and 100 FIDs");
                }
                if fids.contains(&0) {
                    return Err("fids must be positive");
                }
            }
            FeedFilter::EmbedTypes { embed_types } if embed_types.is_empty() => {
                return Err("embed_types must not be empty");
            }
            _ => {}
        }
        if self.limit.is_some_and(|limit| !(1..=100).contains(&limit)) {
            return Err("limit must be between 1 and 100");
        }
        if self.viewer_fid == Some(0) {
            return Err("viewer_fid must be positive");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFeedByFiltersRawQuery {
    pub feed_type: &'static str,
    pub filter_type: FeedFilterKind,
    pub fids: Option<String>,
    pub parent_url: Option<String>,
    pub channel_id: Option<String>,
    pub members_only: Option<bool>,
    pub embed_url: Option<String>,
    pub embed_types: Option<String>,
    pub with_recasts: Option<bool>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
    pub viewer_fid: Option<Fid>,
}
impl From<&GetFeedByFiltersParams> for GetFeedByFiltersRawQuery {
    fn from(params: &GetFeedByFiltersParams) -> Self {
        let (fids, parent_url, channel_id, members_only, embed_url, embed_types) =
            match &params.filter {
                FeedFilter::Fids { fids } => (
                    Some(
                        fids.iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                FeedFilter::ParentUrl { parent_url } => {
                    (None, Some(parent_url.clone()), None, None, None, None)
                }
                FeedFilter::ChannelId {
                    channel_id,
                    members_only,
                } => (
                    None,
                    None,
                    Some(channel_id.clone()),
                    *members_only,
                    None,
                    None,
                ),
                FeedFilter::EmbedUrl { embed_url } => {
                    (None, None, None, None, Some(embed_url.clone()), None)
                }
                FeedFilter::EmbedTypes { embed_types } => (
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(
                        embed_types
                            .iter()
                            .map(|embed_type| embed_type.as_ref())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ),
                FeedFilter::GlobalTrending => (None, None, None, None, None, None),
            };

        Self {
            feed_type: "filter",
            filter_type: params.filter.kind(),
            fids,
            parent_url,
            channel_id,
            members_only,
            embed_url,
            embed_types,
            with_recasts: params.with_recasts,
            limit: params.limit,
            cursor: params.cursor.clone(),
            viewer_fid: params.viewer_fid,
        }
    }
}

// -- Trending feed
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
pub enum TrendingTimeWindow {
    #[serde(rename = "1h")]
    #[strum(serialize = "1h")]
    OneHour,
    #[serde(rename = "6h")]
    #[strum(serialize = "6h")]
    SixHours,
    #[serde(rename = "12h")]
    #[strum(serialize = "12h")]
    TwelveHours,
    #[serde(rename = "24h")]
    #[strum(serialize = "24h")]
    TwentyFourHours,
    #[serde(rename = "7d")]
    #[strum(serialize = "7d")]
    SevenDays,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, Hash, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum TrendingProvider {
    Neynar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrendingFeedParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_window: Option<TrendingTimeWindow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<TrendingProvider>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetTrendingFeedParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.channel_id.is_some() && self.parent_url.is_some() {
            return Err("channel_id and parent_url cannot be used together");
        }
        if self.time_window == Some(TrendingTimeWindow::SevenDays) && self.channel_id.is_none() {
            return Err("time_window=7d requires channel_id");
        }
        if self.limit.is_some_and(|limit| !(1..=10).contains(&limit)) {
            return Err("limit must be between 1 and 10");
        }
        if self.viewer_fid == Some(0) {
            return Err("viewer_fid must be positive");
        }
        Ok(())
    }
}

// -- Feed by channel IDs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFeedByChannelIdsParams {
    pub channel_ids: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_recasts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_fid: Option<Fid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_replies: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fids: Option<Vec<Fid>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_moderate: Option<bool>,

    #[serde(default, skip_serializing)]
    pub experimental: bool,
}
impl GetFeedByChannelIdsParams {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.channel_ids.is_empty() || self.channel_ids.len() > 10 {
            return Err("channel_ids must contain between 1 and 10 IDs");
        }
        if self.fids.as_ref().is_some_and(|fids| fids.len() > 10) {
            return Err("fids must contain at most 10 FIDs");
        }
        if self.fids.as_ref().is_some_and(|fids| fids.contains(&0)) {
            return Err("fids must be positive");
        }
        if self.limit.is_some_and(|limit| !(1..=100).contains(&limit)) {
            return Err("limit must be between 1 and 100");
        }
        if self.viewer_fid == Some(0) {
            return Err("viewer_fid must be positive");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GetFeedByChannelIdsRawQuery {
    pub channel_ids: String,
    pub with_recasts: Option<bool>,
    pub viewer_fid: Option<Fid>,
    pub with_replies: Option<bool>,
    pub members_only: Option<bool>,
    pub fids: Option<String>,
    pub limit: Option<u8>,
    pub cursor: Option<String>,
    pub should_moderate: Option<bool>,
}
impl From<&GetFeedByChannelIdsParams> for GetFeedByChannelIdsRawQuery {
    fn from(params: &GetFeedByChannelIdsParams) -> Self {
        Self {
            channel_ids: params.channel_ids.join(","),
            with_recasts: params.with_recasts,
            viewer_fid: params.viewer_fid,
            with_replies: params.with_replies,
            members_only: params.members_only,
            fids: params.fids.as_ref().map(|fids| {
                fids.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            }),
            limit: params.limit,
            cursor: params.cursor.clone(),
            should_moderate: params.should_moderate,
        }
    }
}
// --

// -- Neynar API format for FIDs -> "1, 2, 3"
#[derive(Debug, Clone, Serialize)]
pub struct GetUsersByFidsRawQuery {
    pub fids: String,
}
impl From<&GetUsersByFidsParams> for GetUsersByFidsRawQuery {
    fn from(params: &GetUsersByFidsParams) -> Self {
        Self {
            fids: params
                .fids
                .iter()
                .map(|fid| fid.to_string())
                .collect::<Vec<_>>()
                .join(","),
        }
    }
}

// -- Signer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSignerStatusParams {
    pub signer_uuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterSignedKeyParams {
    pub app_fid: Fid,
    pub deadline: u64,
    pub signature: String,
    pub signer_uuid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor: Option<SignedKeyRequestSponsor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedKeyRequestSponsor {
    pub fid: Fid,
    pub signature: String,

    #[serde(default)]
    pub sponsored_by_neynar: bool,
}
// --

// -- Notifications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetNotificationsParams {
    pub fid: Fid,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<NotificationFilterKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNotificationsRawQuery {
    pub fid: Fid,

    #[serde(rename = "type")]
    pub kinds: Option<String>,

    pub limit: Option<u8>,
    pub cursor: Option<String>,
}
impl From<&GetNotificationsParams> for GetNotificationsRawQuery {
    fn from(params: &GetNotificationsParams) -> Self {
        Self {
            fid: params.fid,
            kinds: params.kinds.as_ref().map(|kinds| {
                kinds
                    .iter()
                    .map(|kind| kind.as_ref())
                    .collect::<Vec<_>>()
                    .join(",")
            }),
            limit: params.limit,
            cursor: params.cursor.clone(),
        }
    }
}
// --
// ----
