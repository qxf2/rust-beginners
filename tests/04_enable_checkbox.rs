/*
Learn to select a checkbox using Selenium in Rust
DISCLAIMER: This code is aimed at Rust BEGINNERS

AUTHOR: Ajitava Deb

SCOPE:
1) Launch Chrome Driver
2) Navigate to Qxf2 Tutorial page
3) Find the Checkbox element in the Example form and enable it
4) Close the browser
*/

use thirtyfour::prelude::*;

#[tokio::test]
async fn enable_checkbox() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();

    //Create an instance of WebDriver
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    //KEY POINT: The driver.goto method will navigate to a page given by the URL
    driver.goto("http://qxf2.com/selenium-tutorial-main").await?;

    //Maximize the browser window
    driver.maximize_window().await?;

    //KEY POINT: Locate the checkbox and click on it
    let checkbox = driver.find(By::XPath("//input[@type='checkbox']")).await?;
    checkbox.click().await?;

    //Close the browser window
    driver.quit().await?;

    Ok(())
}