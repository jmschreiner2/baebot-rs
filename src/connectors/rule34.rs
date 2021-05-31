use serde::Deserialize;
use serde_xml_rs::from_reader;
use super::base::{ConnectorError, TagResult, PictureResult};
use htmlescape::decode_html;
use log::*;
use rand::Rng;
use regex::Regex;

#[derive(Debug, Deserialize)]
struct AutocompleteResponse {
    label: String,
    value: String
}

pub async fn find_tags(url: &String, search: &String) -> Result<Vec<TagResult>, ConnectorError> {
    info!("Rule34 connector called.");
    info!("Searching for tag {}", search);
    let url = url::Url::parse(format!("{}/autocomplete.php", url).as_str())?;

    info!("Sending request to {}", url);
    let response = reqwest::Client::new()
        .get(url)
        .query(&[("q", search)])
        .send()
        .await?;

    let test = response.text().await?;

    debug!("{}", test);

    let body: Vec<AutocompleteResponse> = serde_json::from_str(test.as_str()).unwrap();//response.json().await?;
    let expression = match Regex::new(r"\(\d+\)$") {
        Ok(regex) => regex,
        Err(why) => {
            error!("Failee to initialize regex expression: {}", why);
            return Err(ConnectorError::Unknown);
        }
    };

    let body: Vec<(String, u32)> = body.iter()
        .map(|post| {
            let count = match expression.find(post.label.clone().as_str()) {
                Some(s) => {
                    let mut mat = s.as_str()[1..]
                        .to_string();

                    mat.pop();
                    mat.parse().unwrap_or(1)
                },
                None => 1
            };

            (post.value.clone(), count) 
        })
        .collect();

    let max_posts = body.iter()
        .map(|(_post, count)| count)
        .max()
        .unwrap_or(&999999);
    
    Ok(body.iter()
        .map(|(tag, count)| TagResult {
            tag: match decode_html(tag.as_str())
            {
                Ok(sanitized) => sanitized,
                Err(why) => {
                    warn!("Failed to sanitized result: {:?}", why);
                    tag.to_string()
                }
            }, 
            affinity: *count as f64 / *max_posts as f64
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct Post {
    file_url: String
}

#[derive(Debug, Deserialize)]
struct PostList {
    count: u32,
    
    #[serde(rename = "post", default)]
    posts: Vec<Post>
}


pub async fn get_random_picture(url: &String, tags: &Vec<String>, sfw_only: bool) -> Result<PictureResult, ConnectorError> {
    if sfw_only {
        info!("SFW filter not supported for Rule34");
        return Err(ConnectorError::SfwNotSupported)
    }

    let tags = tags.join(" ");

    info!("Rule34 connector called.");
    info!("Getting number of pictures for tag {}", tags);
    let url = url::Url::parse(format!("{}/index.php", url).as_str())?;

    info!("Sending request to {}", url);
    let response = reqwest::Client::new()
        .get(url.clone())
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
        .get(url)
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
        Some(ret) => Ok(PictureResult::new(ret.file_url.clone(), ret.file_url.clone())),
        None => {
            warn!("No posts found in response. Could be bad logic.");
            Err(ConnectorError::PictureNotFound)
        }
    }
}
