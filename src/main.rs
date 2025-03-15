use reqwest::Client;
use scraper::{Html, Selector};
use std::{thread, time::Duration};
use std::error::Error;
use teloxide::prelude::*;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get Telegram bot token and chat ID from environment variables
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let chat_id = std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set");
    let chat_id = chat_id.parse::<i64>()?;
    let mut is_episode_available: bool = false;

    
    // Initialize Telegram bot
    let bot = Bot::new(bot_token);
    
    // Create HTTP client
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;
    
    println!("Starting to check for new episodes of Invincible...");
    
    // Check periodically for 1 minute
    // let start_time = std::time::Instant::now();
    while !is_episode_available {
        // Get the webpage
        let response = match client.get("https://www.lostfilm.download/series/Invincible/seasons").send().await {
            Ok(resp) => resp,
            Err(e) => {
                println!("Error fetching website: {}", e);
                thread::sleep(Duration::from_secs(10));
                continue;
            }
        };
        
        // Parse the HTML
        let html = match response.text().await {
            Ok(text) => Html::parse_document(&text),
            Err(e) => {
                println!("Error parsing HTML: {}", e);
                thread::sleep(Duration::from_secs(10));
                continue;
            }
        };
        
        // Find the table
        let table_selector = Selector::parse("table#season_series_601003999").unwrap();
        
        if let Some(table) = html.select(&table_selector).next() {
            // Find the first row
            let tr_selector = Selector::parse("tr").unwrap();
            if let Some(first_tr) = table.select(&tr_selector).next() {
                // Check if the first row has a "not-available" class
                let has_not_available_class = first_tr.value().attr("class").map_or(false, |class| {
                    class.contains("not-available")
                });
                
                if !has_not_available_class {
                    // New episode is available!
                    let message = "New episode of Invincible is now available on LostFilm! https://www.lostfilm.download/series/Invincible/seasons";
                    println!("{}", message);
                    is_episode_available = true;
                    
                    // Send notification to Telegram
                    match bot.send_message(ChatId(chat_id), message).await {
                        Ok(_) => println!("Notification sent to Telegram successfully!"),
                        Err(e) => println!("Failed to send Telegram notification: {}", e),
                    }
                    
                    // Exit the program after sending notification
                    break;
                } else {
                    println!("No new episode yet. Checking again in 10 seconds...");
                }
            }
        } else {
            println!("Could not find the specified table. Check if the selector is correct.");
        }
        
        // Wait 5 seconds before checking again
        thread::sleep(Duration::from_secs(5));
    }
    
    println!("Finished checking for new episodes.");
    Ok(())
}