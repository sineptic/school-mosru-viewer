use std::time::Duration;

use chromiumoxide::{Browser, BrowserConfig, Page};
use futures::StreamExt;

async fn new_school_mosru_page(browser: &Browser) -> anyhow::Result<Page> {
    let page = browser.new_page("https://school.mos.ru").await?;
    tokio::time::sleep(Duration::from_secs(4)).await;
    if let Ok(login_button) = page.find_element(".style_btn__3lIWs").await {
        let login = std::env::var("MOSRU_LOGIN")?;
        let password = std::env::var("MOSRU_PASSWORD")?;

        login_button.click().await?;

        tokio::time::sleep(Duration::from_secs(1)).await;
        let url = page.url().await.unwrap().unwrap();
        if url.starts_with("https://login.mos.ru") {
            println!("login required");
            tokio::time::sleep(Duration::from_secs(2)).await;
            page.find_element("#login")
                .await
                .unwrap()
                .click()
                .await?
                .type_str(login)
                .await?;
            page.find_element("#password")
                .await
                .unwrap()
                .click()
                .await?
                .type_str(password)
                .await?;
            page.find_element("#bind").await.unwrap().click().await?;
            tokio::time::sleep(Duration::from_secs(4)).await;
        }
    }
    tokio::time::sleep(Duration::from_secs(10)).await;
    tokio::time::sleep(LOAD_TIME).await;
    Ok(page)
}

const LOAD_TIME: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build().unwrap()).await?;

    let handler = tokio::spawn(async move {
        while handler.next().await.is_some() {
            //
        }
    });

    let page = new_school_mosru_page(&browser).await?;
    println!("page loaded");

    // marks at https://school.mos.ru/api/family/web/v1/subject_marks?student_id=*****
    //
    //
    handler.await?;
    browser.close().await?;

    Ok(())
}
