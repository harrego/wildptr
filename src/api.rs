use reqwest;

use serde::{Deserialize, Serialize};
//use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct WildpointerRFC {
  pub id: String,
  pub status: String,
  pub date: String,
  pub info_provided: String,
  pub last_updated: Option<String>,
  pub body: Option<String>
}

#[derive(Serialize, Deserialize)]
struct WildpointerData {
  rfcs: Vec<WildpointerRFC>
}

pub fn all() -> Vec<WildpointerRFC> {
  let body = reqwest::blocking::get("https://local.harry.city/wildpointer/data").unwrap()
    .text().unwrap();
  let rfcs: WildpointerData = serde_json::from_str(&body).unwrap();
  return rfcs.rfcs;
}
