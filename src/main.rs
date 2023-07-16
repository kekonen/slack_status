
use std::{path::Path, fs};

use reqwest::{Client, RequestBuilder, Method, IntoUrl, multipart::Form};
use serde::{Deserialize, Serialize};

use clap::{Parser, Subcommand};

#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
struct Profile {
    #[serde(rename = "status_text", skip_serializing_if = "Option::is_none")]
    #[arg(long, short)]
    text: Option<String>,
    #[serde(rename = "status_emoji", skip_serializing_if = "Option::is_none")]
    #[arg(short, long)]
    emoji: Option<String>,
    #[serde(rename = "status_expiration", skip_serializing_if = "Option::is_none")]
    #[arg(long, short = 'x')]
    expiration: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
struct Photo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(index = 1)]
    image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long, short = 'x')]
    crop_x: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long, short = 'y')]
    crop_y: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[arg(long, short = 'w')]
    crop_w: Option<u64>,
}

impl Photo{
    fn update_form(&self, form: Form) -> Form {
        let mut form = form;
        if let Some(x) = &self.crop_x {
            form = form.text("crop_x", x.to_string());
        }
        if let Some(y) = &self.crop_y {
            form = form.text("crop_y", y.to_string());
        }
        if let Some(w) = &self.crop_w {
            form = form.text("crop_w", w.to_string());
        }

        form
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Sets status
    Status(Profile),
    /// Sets photo
    Photo(Photo),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Response {
    ok: bool,
    error: Option<String>,
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

    async fn set_status(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
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

    async fn set_photo(&self, photo: &Photo) -> Result<(), Box<dyn std::error::Error>> {
        let path = photo.image.as_ref().expect("Shopuld provide image").clone();
        let path: &Path = Path::new(&path);
        let filename = String::from(path.clone().file_name().unwrap().to_str().unwrap());
        let extension = String::from(path.clone().extension().unwrap().to_str().unwrap());

        let file = fs::read(path.clone()).expect("File not found");

        let image_part = reqwest::multipart::Part::bytes(file)
        .file_name(filename)
        .mime_str(&format!("image/{}", extension))
        .unwrap();

        let form = reqwest::multipart::Form::new().part("image", image_part);

        let form = photo.update_form(form);

        let resp = self.client(Method::POST, "https://slack.com/api/users.setPhoto")
        .multipart(form)
        .send()
        .await?;
        
        let r = resp.json::<Response>().await?;

        if !r.ok {
            Err(r.error.unwrap_or("".to_string()))?;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let app = App::env();
    let c = Cli::parse();
    if let Some(command) = c.command {
        match command {
            Commands::Status(p) => {
                app.set_status(&p).await?;
            },
            Commands::Photo(p) => {
                app.set_photo(&p).await?;
            }
        }
    }

    Ok(())
}