use std::fs;
use headless_chrome::{Browser, protocol::cdp::Page};
 
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a new instance of Chrome
    let browser = Browser::default()?;
 
    // Chrome always opens with one tab open, so
    // you just get that initial tab.
    let tab = browser.new_tab()?;

    // let tools = devtools(tab, true) 
 
    // Navigate to the website and wait for it to
    // finish loading
    tab.navigate_to("https://facebook.com/")?;
    tab.wait_until_navigated()?;
 
    // Screenshot the page to a PNG file and return
    // the bytes for that PNG
    // let png_data = tab.capture_screenshot(
    //     ScreenshotFormat::PNG,
    //     None,
    //     true)?;
    
    let jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true)?;
    // Save the screenshot to disc
    fs::write("screenshot-2.jpeg", jpeg_data)?;

    // // Save the bytes to a screenshot.png file
    // fs::write("screenshot.png", png_data)?;
 
    Ok(())
}