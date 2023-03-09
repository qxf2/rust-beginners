/*
Learn to navigate to a URL using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Firefox Driver
2) Navigate to Qxf2 Tutorial page
3) Check the page title
4) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn navigate_to_url() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Check if the title of the page is proper
    assert_eq!(driver.title().await?, "Qxf2 Services: Selenium training main");  

    //Quit the browser window 
    driver.quit().await?;
    
    Ok(())
}