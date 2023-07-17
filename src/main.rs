
use std::{path::{Path, PathBuf}, fs};

use reqwest::{Client, RequestBuilder, Method, IntoUrl, multipart::Form};
use serde::{Deserialize, Serialize};

use clap::{Parser, Subcommand};

type TypicalResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
struct Profile {
    /// Sets text of the status
    #[serde(rename = "status_text", skip_serializing_if = "Option::is_none")]
    #[arg(long, short)]
    text: Option<String>,

    /// Sets emoji of the status (provide as :emoji:)
    #[serde(rename = "status_emoji", skip_serializing_if = "Option::is_none")]
    #[arg(short, long)]
    emoji: Option<String>,
    /// Sets expiration of the status as a Unix timestamp
    #[serde(rename = "status_expiration", skip_serializing_if = "Option::is_none")]
    #[arg(long, short = 'x')]
    expiration: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
struct SetToken {

    /// Path to image file
    #[arg(index = 1)]
    token: String,

}


#[derive(Debug, Serialize, Deserialize, Clone, Parser)]
struct Photo {

    /// Path to image file
    #[arg(index = 1)]
    image: String,

    /// X coordinate of top-left corner of crop box
    #[arg(long, short = 'x')]
    crop_x: Option<u64>,

    /// Y coordinate of top-left corner of crop box
    #[arg(long, short = 'y')]
    crop_y: Option<u64>,

    /// Width/height of crop box (always square)
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
    /// Sets token
    SetToken(SetToken),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Optional name to operate on
    // name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

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
    config: Config,
}

impl App {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn token(&self) -> &str {
        &self.config.token
    }

    fn _env() -> Self {
        let config = Config::new(std::env::var("SLACK_TOKEN").expect("Expected a token in the environment"));
        Self::new(config)
    }

    fn from_config(config: Config) -> Self {
        Self::new(config)
    }

    fn from_config_path(path: Option<&PathBuf>) -> TypicalResult<Self> {
        let config = Config::from_file(path)?;
        Ok(Self::from_config(config))
    }

    fn bearer(&self) -> String {
        format!("Bearer {}", self.token())
    }

    fn client<U: IntoUrl>(&self, method: Method, url: U) -> RequestBuilder {
        Client::new().request(method, url).header("Authorization", self.bearer())
    }

    async fn _get_user(&self) -> TypicalResult<String> {
        let resp = self.client(Method::GET, "https://slack.com/api/users.profile.get")
        .send().await?;

        Ok(resp.text().await?)
    }

    async fn set_status(&self, profile: &Profile) -> TypicalResult<()> {
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

    async fn set_photo(&self, photo: &Photo) -> TypicalResult<()> {
        let path: &Path = Path::new(&photo.image);
        let filename = String::from(path.file_name().unwrap().to_str().unwrap());
        let extension = String::from(path.extension().unwrap().to_str().unwrap());

        let file = fs::read(path).expect("File not found");

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

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    token: String,
}

impl Config {
    fn new(token: String) -> Self {
        Self { token }
    }

    fn from_file(path: Option<&PathBuf>) -> TypicalResult<Self> {
        let path = path.cloned().unwrap_or(Self::default_location()?);
        let config = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config)?;
        Ok(config)
    }

    fn default_location() -> TypicalResult<PathBuf> {
        let home = dirs::home_dir().unwrap();
        let config = home.canonicalize()?.join(".config/slack_update/config.toml");
        Ok(config)
    }

    fn write_config(&self, path: Option<&PathBuf>) -> TypicalResult<()> {
        let path = path.cloned().unwrap_or(Self::default_location()?);
        let config_dir = path.parent().unwrap();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }

        let config = toml::to_string(&self)?;
        fs::write(path, config)?;
        Ok(())
    }
}


#[tokio::main]
async fn main() -> TypicalResult<()> {
    let c = Cli::parse();
    
    if let Some(command) = c.command {
        match command {
            Commands::Status(p) => {
                App::from_config_path(None).expect("Token is not set. Start the binary with set-token").set_status(&p).await?;
            },
            Commands::Photo(p) => {
                App::from_config_path(None).expect("Token is not set. Start the binary with set-token").set_photo(&p).await?;
            },
            Commands::SetToken(t) => {
                let config = Config::new(t.token);
                config.write_config(c.config.as_ref())?;
            }
        }
    }

    Ok(())
}