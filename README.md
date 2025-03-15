# LostFilm Episode Tracker

A simple Rust application that monitors LostFilm.download for new episodes of your favorite TV shows and sends notifications to Telegram when they become available.

## Features

- Monitors LostFilm.download for new episode uploads
- Focuses on a specific TV series (currently configured for Invincible)
- Sends instant Telegram notifications when new episodes are detected
- Runs for a configurable time period (default: 1 minute with 5-second check intervals)
- Can be scheduled as a cron job for regular checking

## Prerequisites

- Rust and Cargo installed
- A Telegram bot token (can be obtained from [@BotFather](https://t.me/botfather))
- Your Telegram chat ID

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/yourusername/lostfilm-episode-tracker.git
   cd lostfilm-episode-tracker
   ```

2. Create a `.env` file in the project root with your Telegram credentials:

   ```
   TELEGRAM_BOT_TOKEN=your_telegram_bot_token
   TELEGRAM_CHAT_ID=your_telegram_chat_id
   ```

3. Build the application:
   ```bash
   cargo build --release
   ```

## Usage

Run the tracker manually:

```bash
cargo run --release
```

The application will check for new episodes every 5 seconds for a total duration of 1 minute.

### Scheduled Checking

For automated checking, you can set up a cron job (Linux/macOS) or Task Scheduler (Windows).

Example cron job that runs every hour:

```bash
0 * * * * cd /path/to/wait-for-new-lostfilm-invincible-episode && ./target/release/wait-for-new-lostfilm-invincible-episode
```

## Configuration

To track a different TV series or modify the behavior:

1. Edit the URL and HTML selector in the source code to target different shows
2. Adjust the check interval and total duration as needed
3. Rebuild the application

## How It Works

1. The application requests the LostFilm series page
2. It searches for a specific table using the provided CSS selector
3. It examines the first row of the table
4. If the first row does not have the "not-available" class, it means a new episode is available
5. A notification is sent to the configured Telegram chat

## Dependencies

- reqwest - HTTP client
- scraper - HTML parsing
- teloxide - Telegram Bot API
- tokio - Asynchronous runtime
- dotenv - Environment variable management

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
