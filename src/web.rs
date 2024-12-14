use std::error::Error;
use tokio;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use vader_sentiment::SentimentIntensityAnalyzer;

/// the struct to format our search result
///
/// # fields
/// - 'title': the title of the website
/// - 'url': the url of the website
/// - 'snippet': the summary of the content of the website
/// - 'frequency': the frequency of the keyword that we are searching
/// - 'update_time': the latest update time of the website
/// - 'result_list': a vector that stores tuples of (keyword context, sentiment analysis)
#[derive(Debug)]
pub struct ReportEntry{
    title: String,
    url: String,
    snippet: String,
    frequency: usize,
    update_time: String,
    pub(crate) results_list: Vec<(String, String)>
}

/// doing single keyword search on a single webpage
///
/// # parameters
/// - 'url': the url of the website that we will search
/// - 'keyword': the keyword used to search, could be multiple words seperated with ' '
///
///  # return value
/// a struct that has everything we need or an error
pub async fn single_webpage_single_keyword(url: &str, keyword: &str) -> Result<ReportEntry, Box<dyn Error>> {
    // let the thread pool take care of the blocking code
    tokio::task::block_in_place(|| -> Result<ReportEntry, Box<dyn Error>> {
        // the website's source code
        let response = get(url)?.text()?;
        // parsing HTML source code
        let html = Html::parse_document(&response);

        // define the variables to store the values we need
        let mut title = String::from("");
        let mut snippet = String::from("");
        let mut frequency = 0;
        let mut update_time = String::from("");
        let mut results_list: Vec<(String, String)> = vec![];

        // define the sentiment analyzer
        let analyzer = SentimentIntensityAnalyzer::new();

        // create a selector that locates the <body> label
        if let Ok(body_selector) = Selector::parse("body") {
            // iterate through the what's inside <body>
            if let Some(body) = html.select(&body_selector).next() {
                // obtain the text content of <body>
                let body_text = body.text().collect::<Vec<_>>();

                // search for the keyword
                for element in body_text {
                    if element.contains(keyword) {
                        frequency += 1;
                        results_list.push((String::from(element), format!("{:?}", analyzer.polarity_scores(element))));
                    }
                }
            } else { println!("cannot find the <body> element"); }
        }else { println!("failed to create a selector"); }

        // if keyword is not found in the given website, simply return
        if frequency == 0 {
            return Ok(ReportEntry{
                title: "".to_string(),
                url: "".to_string(),
                snippet: "".to_string(),
                frequency: 0,
                update_time: "".to_string(),
                results_list: vec![]
            });
        }

        // extract <title>
        if let Ok(title_selector) = Selector::parse("title"){
            if let Some(x) = html.select(&title_selector).next().map(|element| element.text().collect::<String>()){
                title = x;
            }
        }

        // extract the content attribute of <meta name="description"> as snippet
        if let Ok(description_selector) = Selector::parse(r#"meta[name="description"]"#){
            if let Some(x) = html.select(&description_selector).next().and_then(|element| element.value().attr("content")){
                snippet = String::from(x);
            }
        }

        // extract the updated_time from <meta property="article:modified_time">
        if let Ok(update_time_selector) = Selector::parse(r#"meta[name="last-modified"]"#){
            if let Some(x) = html.select(&update_time_selector).next().and_then(|element| element.value().attr("content")){
                update_time = String::from(x);
            }
        }

        Ok(ReportEntry{
            title,
            url: String::from(url),
            snippet,
            frequency,
            update_time,
            results_list
        })
    })
}

pub async fn web_main() {

}
