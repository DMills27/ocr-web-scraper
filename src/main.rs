use std::fs;
use std::process::Command;
use headless_chrome::{Browser, protocol::cdp::Page};

// fn extract_between_http_and_dot(url: &str) -> str {
//     let start = url.find("http://")?;
//     let end = url[start + 7..].find('.')? + start + 7;
//     &url[start + 7..end]
// }
 
fn main() -> Result<(), Box<dyn std::error::Error>> {

    // A vector of targeted websites to scrape from
    let websites = vec![
        String::from("https://edgerealtyja.com"),
        String::from("https://www.jacars.net/cars"),
        String::from("https://hilofoodstoresja.loccloud.net/xstore/index.html#desktop=pos.desktopproduct"),
    ];

    for website in &websites {
        println!("{}", website);
        // Open a new instance of Chrome
        let browser = Browser::default()?;
    
        // Chrome always opens with one tab open, so
        // you just get that initial tab.
        let tab = browser.new_tab()?;
    
        // tab.print_to_pdf({landscape =  Some<true>
        //     display_header_footer = Some<true>
        //     §print_background = Some<true>});
        // Navigate to the website and wait for it to
        // finish loading
        tab.navigate_to(website)?;
        // tab.set_default_timeout(std::time::Duration::from_secs(5));
        // tab.wait_until_navigated()?;
        tab.wait_for_element("#img");
    
        // Screenshot the page to a JPEG file and return
        // the bytes for that PNG
        let jpeg_data = tab.capture_screenshot(
            Page::CaptureScreenshotFormatOption::Jpeg,
            None,
            None,
            true)?;
        // Save the screenshot to disc
        // let website_name = extract_between_http_and_dot(website);

        let slice = &website[8..16];
        println!("{}", slice);

        fs::write(slice.to_owned() + ".jpeg", jpeg_data)?;
    
        Command::new("ocrs")
        .arg(slice.to_owned() + ".jpeg")
        .arg("-o")
        .arg(slice.to_owned() + ".txt")
        .spawn()
        .expect("ocrs command failed to start");
    }

    Ok(())
}