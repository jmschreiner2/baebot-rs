use serde::Deserialize;
use super::base::{ConnectorError, TagResult, PictureResult};
use serde_xml_rs::from_reader;
use htmlescape::decode_html;
use log::*;
use rand::Rng;

const SFW_TAG: &str = "rating:safe";

#[derive(Debug, Deserialize)]
struct TagSearchResult {
    count: String,
    tag: String
}

pub async fn find_tags(url: &String, search: &String) -> Result<Vec<TagResult>, ConnectorError> {
    info!("Starting gentoo connect with base url: {}", url);
    let url = url::Url::parse(format!("{}/index.php", url).as_str())?;

    let response: Vec<TagSearchResult> = reqwest::Client::new()
        .get(url)
        .query(&[("page", "dapi"),
            ("q", "index"),
            ("s", "tag"),
            ("name_pattern", format!("%{}%", search).as_str()),
            ("order", "DESC"),
            ("orderby", "count"),
            ("limit", "10"),
            ("json", "1")
        ])
        .send()
        .await?
        .json()
        .await?;

    let max_posts = response
        .iter()
        .map(|res| res.count.parse::<u64>().unwrap_or(1))
        .max()
        .unwrap_or(999999) as f64;

    Ok(response.iter()
        .map(|r| TagResult {
            tag: match decode_html(r.tag.as_str())
            {
                Ok(sanitized) => sanitized,
                Err(why) => {
                    warn!("Failed to sanitized result: {:?}", why);
                    r.tag.clone()
                }
            },
            affinity: r.count.parse::<f64>().unwrap_or(1.0) / max_posts
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct Post {
    id: u64,
    file_url: String
}

#[derive(Debug, Deserialize)]
struct PostList {
    count: u32,
    
    #[serde(rename = "post", default)]
    posts: Vec<Post>
}

pub async fn get_random_picture(url: &String, tags: &Vec<String>, sfw_only: bool) -> Result<PictureResult, ConnectorError> {
    info!("Genbooru connector called.");
    let tags = if sfw_only {
        let mut tags = tags.clone();
        tags.push(SFW_TAG.to_string());
        tags.join(" ")
    }
    else {
        tags.join(" ")
    };

    info!("Getting number of pictures for tag {}", tags);
    let url_parsed = url::Url::parse(format!("{}/index.php", url).as_str())?;

    info!("Sending request to {}", url_parsed);
    let response = reqwest::Client::new()
        .get(url_parsed.clone())
        .query(&[("page", "dapi"), 
            ("s", "post"), 
            ("q", "index"),
            ("tags", &tags),
            ("limit", "1")])
        .send()
        .await?;

    let body_str = response.text().await?;

    debug!("{}", body_str);
    info!("Generating random number to get picture");
    let picture_index = {
        let body: PostList = from_reader(body_str.as_bytes())?;
        let mut rng = rand::thread_rng();
        rng.gen_range(0..body.count)
    }.to_string();

    info!("Requesting post {}", picture_index);
    let response = reqwest::Client::new()
        .get(url_parsed)
        .query(&[("page", "dapi"), 
            ("s", "post"), 
            ("q", "index"),
            ("tags", &tags),
            ("limit", "1"),
            ("pid", &picture_index)])
        .send()
        .await?;

    let body_str = response.text().await?;

    debug!("{}", body_str);
    let body: PostList = from_reader(body_str.as_bytes())?;
    match body.posts.first() {
        Some(ret) => {
            let source = format!("{}/index.php?page=post&s=view&id={}", url, ret.id);
            Ok(PictureResult::new(ret.file_url.clone(), source))
        },
        None => {
            warn!("No posts found in response. Could be bad logic.");
            Err(ConnectorError::PictureNotFound)
        }
    }
}
