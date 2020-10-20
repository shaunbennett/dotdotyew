use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::FetchService;

const BASE_URL: &str = "http://localhost:8000";

#[derive(Serialize, Deserialize, Debug)]
pub struct PollChoice {
    pub id: i32,
    pub poll_id: usize,
    pub details: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollMetadata {
    pub id: i32,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct VoteSubmission {
    pub voter: String,
    pub choices: HashMap<i32, i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vote {
    pub id: i32,
    pub poll_id: i32,
    pub choice_id: i32,
    pub dots: i32,
    pub voter: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollResults {
    pub poll: PollMetadata,
    pub choices: Vec<PollChoice>,
    pub votes: Vec<Vote>,
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

pub fn vote<S, C, M, F>(
    poll_id: &str,
    voter: S,
    choices: HashMap<i32, i32>,
    link: &ComponentLink<C>,
    callback: F,
) -> FetchTask
where
    S: Into<String>,
    C: Component,
    M: Into<C::Message>,
    F: Fn(Response<Nothing>) -> M + 'static,
{
    let vote = VoteSubmission {
        voter: voter.into(),
        choices,
    };

    let post_request = Request::post(format!("{}/api/v1/polls/{}/vote", BASE_URL, poll_id))
        .body(Json(&vote))
        .unwrap();
    let callback = link.callback(callback);
    FetchService::fetch(post_request, callback).unwrap()
}

pub fn get_results<C, M, F>(id: &str, link: &ComponentLink<C>, callback: F) -> FetchTask
    where
        C: Component,
        M: Into<C::Message>,
        F: Fn(Response<Json<Result<PollResults, Error>>>) -> M + 'static,
{
    let get_request = Request::get(format!("{}/api/v1/polls/{}/results", BASE_URL, id))
        .body(Nothing)
        .unwrap();
    let callback = link.callback(callback);
    FetchService::fetch(get_request, callback).unwrap()
}
