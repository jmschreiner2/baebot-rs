pub mod discord;

use serde::Deserialize;
use std::default::Default;

#[derive(Debug, Deserialize, Default)]
pub struct RedditAuth
{
    pub client_id: String,
    pub client_secret: String,
    pub user_agent: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Default)]
pub struct DiscordAuth
{
    pub token: String
}

#[derive(Debug, Deserialize, Default)]
pub struct AuthConfig
{
    pub discord: DiscordAuth,
    pub reddit: RedditAuth
}

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig
{
    pub meme_subreddits: Vec<String>,
    pub picture_types: Vec<String>
}
