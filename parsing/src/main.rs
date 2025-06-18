use std::time::Duration;

use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build().unwrap()).await?;

    let handler = tokio::spawn(async move {
        while handler.next().await.is_some() {
            //
        }
    });

    let page = browser.new_page("https://school.mos.ru").await?;
    tokio::time::sleep(Duration::from_millis(1000)).await;
    page.find_element(".style_btn__3lIWs")
        .await?
        .click()
        .await?;

    handler.await?;
    println!("Interactive mode");
    std::thread::sleep_ms(1_000_000);

    Ok(())
}
