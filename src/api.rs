use anyhow::Error;
use serde::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;

const BASE_URL: &str = "http://localhost:8000";

#[derive(Serialize, Deserialize, Debug)]
pub struct PollChoice {
    pub id: usize,
    pub poll_id: usize,
    pub details: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollMetadata {
    pub id: usize,
    pub uuid: String,
    pub title: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub poll: PollMetadata,
    pub choices: Vec<PollChoice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePoll {
    pub title: String,
    pub choices: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePollResponse {
    pub poll: String,
}

pub fn get_poll<C, M, F>(id: &str, link: &ComponentLink<C>, callback: F) -> FetchTask
where
    C: Component,
    M: Into<C::Message>,
    F: Fn(Response<Json<Result<Poll, Error>>>) -> M + 'static,
{
    let get_request = Request::get(format!("{}/api/v1/polls/{}", BASE_URL, id))
        .body(Nothing)
        .unwrap();
    let callback = link.callback(callback);
    FetchService::fetch(get_request, callback).unwrap()
}

pub fn create_poll<S, C, M, F>(
    title: S,
    choices: &[String],
    link: &ComponentLink<C>,
    callback: F,
) -> FetchTask
where
    S: Into<String>,
    C: Component,
    M: Into<C::Message>,
    F: Fn(Response<Json<Result<CreatePollResponse, Error>>>) -> M + 'static,
{
    let poll = CreatePoll {
        title: title.into(),
        choices: choices.to_vec(),
    };

    let post_request = Request::put(format!("{}/api/v1/polls", BASE_URL))
        .body(Json(&poll))
        .unwrap();
    let callback = link.callback(callback);
    FetchService::fetch(post_request, callback).unwrap()
}
