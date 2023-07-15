
use reqwest::{Client, RequestBuilder, Method, IntoUrl};
use serde::{Deserialize, Serialize};

use clap::Parser;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Response {
    ok: bool,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
struct Profile {
    #[serde(rename = "status_text")]
    #[arg(long, short)]
    text: Option<String>,
    #[serde(rename = "status_emoji")]
    #[arg(short, long)]
    emoji: Option<String>,
    #[serde(rename = "status_expiration")]
    #[arg(long, short = 'x')]
    expiration: Option<u64>,
}

impl Profile {
    fn _new(text: Option<String>,
        emoji: Option<String>,
        expiration: Option<u64>) -> Profile {
        Profile {
            text,
            emoji,
            expiration,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUser {
    profile: Profile,
}

struct App {
    token: String,
}

impl App {
    fn new(token: String) -> Self {
        Self { token }
    }

    fn env() -> Self {
        Self::new(std::env::var("SLACK_TOKEN").expect("Expected a token in the environment"))
    }

    fn bearer(&self) -> String {
        format!("Bearer {}", self.token)
    }

    fn client<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        Client::new().request(method, url).header("Authorization", self.bearer())
    }

    async fn _get_user(&self) -> Result<String, Box<dyn std::error::Error>> {
        let resp = self.client(Method::GET, "https://slack.com/api/users.profile.get")
        .send().await?;

        return Ok(resp.text().await?)
    }

    async fn set(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        let u = UpdateUser{profile: profile.clone()};
        let resp = self.client(Method::POST, "https://slack.com/api/users.profile.set")
        .json(&u)
        .send().await?;

        let r = resp.json::<Response>().await?;

        if !r.ok {
            Err(r.error.unwrap_or("".to_string()))?;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let app = App::env();
    let p = Profile::parse();
    app.set(&p).await.unwrap();
}