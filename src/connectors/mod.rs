pub mod base;
mod rule34;
mod gelbooru;
mod danbooru;

use base::{ConnectorError, PictureResult, MIN_AFFINITY};
use super::settings::CoomSettings;
use std::sync::Arc;
use std::collections::HashMap;
use log::*;
use base::TagResult;
use futures::*;
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    static ref SOURCES: Arc<Vec<SourceType>> = Arc::new(vec![
        SourceType::Rule34(String::from("https://rule34.xxx")),
        SourceType::Danbooru(String::from("https://danbooru.donmai.us")),
        SourceType::Danbooru(String::from("https://safebooru.donmai.us")),
        SourceType::Gelbooru(String::from("https://gelbooru.com"))
    ]);
}

const MAX_RETRIES: usize = 3;

enum SourceType {
    Rule34(String),
    Danbooru(String),
    Gelbooru(String)
}

async fn dispatch_get_picture(source: &SourceType, tags: &Vec<String>, sfw_only: bool) -> Result<PictureResult, ConnectorError> {
    match source {
        SourceType::Rule34(ref base_url) => rule34::get_random_picture(base_url, tags, sfw_only).await,
        SourceType::Danbooru(ref base_url) => danbooru::get_random_picture(base_url, tags, sfw_only).await,
        SourceType::Gelbooru(ref base_url) => gelbooru::get_random_picture(base_url, tags, sfw_only).await
    }
}

async fn dispatch_search_tag(source: &SourceType, tag: &String) -> Result<Vec<TagResult>, ConnectorError> {
    match source {
        SourceType::Rule34(ref base_url) => rule34::find_tags(base_url, tag).await,
        SourceType::Danbooru(ref base_url) => danbooru::find_tags(base_url, tag).await,
        SourceType::Gelbooru(ref base_url) => gelbooru::find_tags(base_url, tag).await
    }
}

pub async fn get_random_picture(tags: Vec<String>, sfw_only: bool) -> Option<PictureResult> {
    info!("Attempting to get a random source.");
    let source_count = (*SOURCES).len();
    let source_index = rand::thread_rng().gen_range(0..source_count);

    let mut retry_count: usize = 0;
    loop {
        if retry_count == MAX_RETRIES {
            error!("Failed to get a picture with tag: {:?}", tags);
            return None;
        }

        let source = match (*SOURCES).get((source_index + retry_count) % source_count) {
            Some(s) => s,
            None => {
                error!("Generated a random source that doesn't exist!");
                return None;
            }
        };

        let res = match dispatch_get_picture(source, &tags, sfw_only).await {
            Ok(url) => Some(url),
            Err(why) => {
                warn!("Source failed: {}", why);
                retry_count += 1;
                None
            }
        };

        if res.is_some() {
            return res;
        }
    }
}

pub async fn get_real_picture(tags: Vec<String>, sfw_only: bool) -> Option<PictureResult> {
    info!("Attempt to get a random real picture.");
    match dispatch_get_picture(&SourceType::Rule34(String::from("https://realbooru.com")), &tags, sfw_only).await {
        Ok(res) => Some(res),
        Err(why) => {
            warn!("Source failed: {:?}", why);
            None
        }
    }
}

pub async fn search_tag(tag: String) -> Vec<TagResult>
{
    let min_affinity = match CoomSettings::new() {
        Ok(config) => config.min_affinity,
        Err(why) => {
            info!("No configured minimum affinity: {:?}", why);
            MIN_AFFINITY
        }
    };

    let source_count = (*SOURCES).len() as f64;
    info!("Calling all connectors to search for tag {}", tag);
    let results = {
        let mut futures = Vec::new();
        for source in (*SOURCES).iter()
        {
            futures.push(dispatch_search_tag(source, &tag));
        }
        
        future::join_all(futures)
    }.await;

    info!("Organizing results");
    let mut tags = HashMap::<String, f64>::new();

    results
        .iter()
        .for_each(|r| match r {
            Ok(res) => {
                res.iter()
                    .for_each(|tag| {
                        let c = match tags.get(&tag.tag) {
                            Some(curr_count) => curr_count + (tag.affinity / source_count),
                            None => (tag.affinity / source_count)
                        };
                        tags.insert(tag.tag.clone(), c);
                    });
            },
            Err(why) => warn!("Error to get tags: {:?}", why)
        });

    info!("Sorting results");
    let mut tags: Vec<(&String, &f64)> = tags.iter().collect();
    tags.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut ret = Vec::new();
    tags.iter()
        .for_each(|(tag, affinity)| ret.push(TagResult {
            tag: tag.to_string(),
            affinity: *affinity.clone()
        }));

    ret.into_iter()
        .filter(|tag| tag.affinity >= min_affinity)
        .collect()
}

pub async fn search_real_tag(tag: String) -> Vec<TagResult> {
    let min_affinity = match CoomSettings::new() {
        Ok(config) => config.min_affinity,
        Err(why) => {
            info!("No configured minimum affinity: {:?}", why);
            MIN_AFFINITY
        }
    };

    match dispatch_search_tag(&SourceType::Rule34(String::from("https://realbooru.com")), &tag).await {
        Ok(res) => res,
        Err(why) => {
            warn!("Error to get tags: {:?}", why);
            Vec::new()
        }
    }.into_iter()
        .filter(|tag| tag.affinity >= min_affinity)
        .collect()
}

