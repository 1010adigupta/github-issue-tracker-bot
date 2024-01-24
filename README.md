# GitHub Issue Alert Bot 🤖 🦀

## Overview
GitHub Issue Alert Bot is an automated tool designed to monitor issues in specified open-source repositories on GitHub and send Telegram notifications when new issues with specified labels are created. This bot is ideal for developers and teams who want to stay updated on specific types of issues in their projects or in other open-source repositories they are interested in.

## Features
- **Issue Monitoring**: Automatically monitors GitHub repositories for new issues.
- **Customizable Labels**: Sends notifications for issues with specified labels.
- **Telegram Notifications**: Receive real-time alerts directly on Telegram.
- **Multiple Repository Support**: Can be configured to monitor multiple repositories.

## Getting Started

### Prerequisites
- A Telegram bot token and your Telegram chat ID.
- Rust and Cargo installed on your system.

### Installation and Setup
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/1010adigupta/github-issue-tracker-bot.git
   cd github-issue-alert-bot

2. **Update your telgram bot token and chat id src/main.rs**
3. **Build and run the server**
   ```bash
   cargo build --release
   cargo run

### Track Repositories
Select the public repository you want to monitor and send curl request to bot server to track this respository. This repository info will stored in the global state for future tracking.
Add as many repsitories as you want to monitor!
Example:
   ```bash
   curl -X POST http://127.0.0.1:8081/input \ 
     -H "Content-Type: application/json" \
     -d '{"owner":"1010adigupta", "repo":"portfolio", "label":"good first issue"}'
