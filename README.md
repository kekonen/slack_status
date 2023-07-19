# Slack Update 🕺💃

A tool for updating certain things in your Slack Profile:
* Status (emoji, text, expiration)
* Profile pic

## Setup
To start using this tool you first need to create an app and connect it to your workspace

### Creating the app
1. https://api.slack.com/apps
2. Create New App
3. Choose "from scratch"
4. Give it a name and choose the workspace
5. In the next window find and press "OAuth & Permissions"
6. Follow "Scopes" > "User Token Scopes" > press "Add OAuth Scope"
7. Add "users.profile:write" scope
8. Then, within the same page find "OAuth Tokens for Your Workspace" > "Install to Workspace" > Install
9. This will provide you with a Token that looks like "xoxp-...". Save it for the next step

### Install the cli
Install the binary
```bash
cargo install slack_update
```

Set up your token from the previous step. It will be stored into `~/.config/slack_update/config.toml`
```bash
slack_update set-token "xoxp-..."
```

### Usage
Set emoji, text and expiration (as a unix timestamp)
```bash
slack_update status -e ":crazy:" -t gotcha! -x 1337
```

Set image (with cropping)
```bash
slack_update photo -w 150 ./bubble_gum.png 
```

Get further help with
```bash
slack_update -h
```