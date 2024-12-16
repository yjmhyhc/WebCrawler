use std::error::Error;
use tokio;
use reqwest;
use scraper::{Html, Selector};
use vader_sentiment::SentimentIntensityAnalyzer;
use std::sync::{Arc, Mutex};
use futures::future::join_all;

/// the struct to format our search result
///
/// # fields
/// - 'title': the title of the website
/// - 'url': the url of the website
/// - 'snippet': the summary of the content of the website
/// - 'frequency': the frequency of the keyword that we are searching
/// - 'update_time': the latest update time of the website
/// - 'result_list': a vector that stores tuples of (keyword context, sentiment analysis)
#[derive(Clone, Debug)]
pub struct ReportEntry{
    title: String,
    url: String,
    snippet: String,
    frequency: usize,
    update_time: String,
    results_list: Vec<(String, String)>
}

impl ReportEntry {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn snippet(&self) -> &str {
        &self.snippet
    }
    pub fn frequency(&self) -> usize {
        self.frequency
    }
    pub fn update_time(&self) -> &str {
        &self.update_time
    }
    pub fn results_list(&self) -> &Vec<(String, String)> {
        &self.results_list
    }
}

/// doing keyword search on a single webpage
///
/// # parameters
/// - 'url': the url of the website that we will search
/// - 'keyword': the keyword used to search, could be multiple words seperated with ' '
///
///  # return value
/// a struct that has everything we need or an error
pub async fn single_webpage_search(keyword: String, url: &str) -> Result<ReportEntry, Box<dyn Error>> {

        // the website's source code
        let response = reqwest::get(url).await?.text().await?;
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
                let body_text = body.text().collect::<Vec<&str>>();

                // search for the keyword
                for element in body_text {
                    if let Some(pos) = element.find(&keyword[..]) {
                        frequency += 1;
                        let mut extracted_text = String::from(element);
                        // if the element (the text node) is too long, we only take a slice containing the keyword
                        if extracted_text.chars().count() > 70{
                            extracted_text = String::from(&extracted_text[pos..]);
                            if extracted_text.chars().count() > 70{
                                extracted_text = extracted_text.chars().take(70).collect::<String>();
                            }
                        }
                        results_list.push((extracted_text, format!("{:?}", analyzer.polarity_scores(element))));
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
}


/// parsing the input parameters and create asynchronous tasks to send requests
///
/// # parameters
/// - 'keyword': the keyword that we want to search
/// - 'websites': the websites that we search on
///
///  # return value
/// a vector of struct that contains search results from multiple websites
pub async fn web_main(keyword: &str, websites: &str) -> Result<Vec<ReportEntry>, Box<dyn Error>> {
    let mut websites_vec: Vec<String> = vec![];
    if websites == "" {
        // if the user left the websites field blank intentionally, then we need to google the keyword for some websites
        // construct the search url
        let url = format!(
            "https://www.google.com/search?q={}&num=100",
            keyword.replace(" ", "+").replace("/","+")
        );

        // send a request
        let response = reqwest::get(&url).await?.text().await?;

        // parsing html
        let html = Html::parse_document(&response);
        // locate all "a" elements containing the urls
        if let Ok(selector) = Selector::parse("div.egMi0.kCrYT > a"){
            for a_element in html.select(&selector){
                // locate the href attribute of "a" and take a slice of it
                if let Some(href) = a_element.value().attr("href") {
                    if let Some(start) = href.find("?q=") {
                        // start from ?q=
                        let sub_str = &href[start + 3..];

                        // end at &
                        if let Some(end) = sub_str.find('&') {
                            websites_vec.push(String::from(&sub_str[..end]));
                        }
                    }
                }
            }
        }
    } else {
        // parsing the input parameters
        websites_vec = websites.split(' ').map(|s| String::from(s)).collect();
    }

    // define a vector that contains the result we want to return
    let result_vec: Arc<Mutex<Vec<ReportEntry>>> = Arc::new(Mutex::new(vec![]));

    // define a vector to store all async tasks
    let mut tasks = vec![];

    for a_website in websites_vec{
        let vec_clone = Arc::clone(&result_vec);
        let mut kwd_for_pattern_matching = String::from(keyword);
        // if the user want to separate the context from the actual item, a '/' will appear
        if let Some(pos) = keyword.find('/'){
            kwd_for_pattern_matching = String::from(&keyword[pos..]);
        }
        tasks.push(
            tokio::spawn(async move {
                if let Ok(report_entry) = single_webpage_search(kwd_for_pattern_matching, &a_website[..]).await{
                    //if the ReportEntry is not empty, then it is valid
                    if report_entry.frequency() != 0{
                        if let Ok(mut vec) = vec_clone.lock(){
                            vec.push(report_entry);
                        };
                    }
                }
            })
        );
    }

    // wait until all tasks are finished
    let _ = join_all(tasks).await;

    let vec_clone = Arc::clone(&result_vec);
    if let Ok(vec) = vec_clone.lock(){
        return Ok(vec.clone());
    }
    return Ok(vec![]);
}
