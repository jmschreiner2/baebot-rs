use serde::Deserialize;
use super::base::{ConnectorError, TagResult, PictureResult};
use htmlescape::decode_html;
use log::*;

#[derive(Debug, Deserialize)]
struct AutocompleteResponse {
    label: String,
    value: String,
    post_count: u32
}

const SFW_TAG: &str = "rating:safe";

pub async fn find_tags(url: &String, search: &String) -> Result<Vec<TagResult>, ConnectorError> {
    info!("Danbooru connector called with base url: {}", url);
    info!("Searching for tag {}", search);
    let url = url::Url::parse(format!("{}/autocomplete.json", url).as_str())?;

    info!("Sending request to {}", url);
    let response: Vec<AutocompleteResponse> = reqwest::Client::new()
        .get(url)
        .query(&[("search[query]", search.as_str()),
                ("search[type]", "tag_query"),
                ("limit", "10")])
        .send()
        .await?
        .json()
        .await?;

    let total_posts: u32 = response.iter().map(|post| post.post_count).sum();

    Ok(response.iter()
        .map(|r| TagResult {
            tag: match decode_html(r.value.as_str())
            {
                Ok(sanitized) => sanitized,
                Err(why) => {
                    warn!("Failed to sanitized result: {:?}", why);
                    r.value.clone()
                }
            }, 
            affinity: r.post_count as f64 / total_posts as f64
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct Post {
    id: u64,
    large_file_url: String
}

pub async fn get_random_picture(url: &String, tags: &Vec<String>, sfw_only: bool) -> Result<PictureResult, ConnectorError> {
    info!("Danbooru connector called with base url: {}", url);

    let tags = if sfw_only {
        let mut tags = tags.clone();
        tags.push(SFW_TAG.to_string());
        tags.join(" ")
    }
    else {
        tags.join(" ")
    };

    info!("Getting number of pictures for tag {}", tags);
    info!("Requesting random post");
    let parsed_url = url::Url::parse(format!("{}/posts.json", url).as_str())?;
    let response: Vec<Post> = reqwest::Client::new()
        .get(parsed_url)
        .query(&[("tags", tags.as_str()), 
            ("random", "true"), 
            ("limit", "1")])
        .send()
        .await?
        .json()
        .await?;

    match response.first() {
        Some(ret) => {
            let source = format!("{}/posts/{}", url, ret.id);
            Ok(PictureResult::new(ret.large_file_url.clone(), source))
        },
        None => {
            warn!("No posts found in response. Could be bad logic.");
            Err(ConnectorError::PictureNotFound)
        }
    }
}
