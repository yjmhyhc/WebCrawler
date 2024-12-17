# A web crawler with data analysis

## Developer Information

name: Haocheng Yang

student number: 1010373301

email address: hc.yang@mail.utoronto.ca

## Motivation

### Why a web crawler is worth developing?

What drives people to develop web crawlers and give up the convenience supplied by the text search feature of a web browser? In my opinion, web crawlers wins the match for the following reasons.

#### 1. Automating Data Collection

The internet contains an immense amount of information spread across countless websites. Manually collecting this data is time-consuming, inefficient, and prone to human error. A web crawler automates this process, enabling the extraction of large datasets with minimal human intervention. This automation is critical for applications like search engines, market research, and trend analysis.

#### 2. Enabling Analysis

Web crawlers are also stronger in data analysis. In economical field, businesses can leverage web crawlers to monitor competitors, track pricing, and gather customer reviews. For example, e-commerce platforms use crawlers to compare product prices, while marketing firms analyze public sentiment through reviews and comments. Developing a web crawler offers a competitive advantage in today's data-driven economy. 

In academia, researchers often need large datasets for studies in fields like natural language processing, social network analysis, or epidemiology. A web crawler can collect relevant data from online sources, such as academic publications, blogs, and forums, and do things like the map-reduce technique proposed by Google's research team, enabling groundbreaking research and innovation.

In short, data itself is not that valuable, the true treasure is what it reveals to us.

### Why do I want to build a web crawler?

#### 1. Search for game characters

I believe every boy loves video games, I am not an exception. Sometimes I try hard to get the new information of my beloved games (like Street Fighter), but most of the time I get garbage information. This is because the information that I need may occur on different websites, and the time it occur is arbitrary, not to mention I have to spend a few minutes scrolling down a certain website to find out where it is. As a human, I cannot monitor certain websites 24-7, and the worst thing is, I myself may not know where the information is. I hope someone could do it for me and hand in the report, and that is where a web crawler comes in.

#### 2. Learning to build a web crawler is beneficial

Let's begin from the highest layer, the front end UI of a web crawler. Writing these code is all about thinking how to guide users of my web crawler to quickly get their hands on it. I communicate with users through dialog boxes, text boxes and buttons, so it is crucial that I make these things as precise and clear as possible. I learnt a lot from the perspective of a UI designer.

Then it comes down to the lower level part of my project, the code is included in a module called web. I familiarize myself with the HTML elements, DOM tree and so on because I have to know where the information that we care is stored. I also need to learn to use the asynchronous way of dealing with problems, so that users don't have to wait long just because my program is single-threaded. During the process of repeating "cargo build" and "cargo run"(which is the debugging process), I also have a deeper understanding of Rust programming language itself.

Overall, the hands-on experience to design a project methodically is perhaps the most valuable for me, I think it really prepares me with the ability to deal with real-world problems.

## Objectives

At the beginning of this project, I did set some objectives, some of them are accomplished some of them are not, I can say that I did my best, but there are still things to be refined and I have a lot to learn.

### 1. Basic craw

The web crawler should be able to search for a keyword (which can be multiple words separated with ' ') on the websites that we instruct.

This goal is accomplished. In fact, I extended my program to support separating the actual item from the context. For example, you can try to input "Street Fighter/Ken" to indicate that you want information about the character Ken in the game Street Fighter, not "Ken" in any other place.

### 2. Website generation

The web crawler should be able to generate some websites to craw if the user only provides the keyword.

This goal is also accomplished, my program will use Google to search for the keyword, and take the first 100 websites to craw.

### 3. Iterative craw

This means that the web crawler should be able to "click" the hyper links on the websites it craws, and keep on crawling on newly opened websites.

Unfortunately, this is not a feature of my web crawler, as there are so many hyper links on a website, it is hard to distinguish which ones are linked to useful websites, without a mechanism to find out, crawling iteratively may generate a lot of garbage information, so I gave up on this feature. But there is certainly a chance to enforce this type of mechanism.

### 4. Asynchronous craw

The web crawler should be able to do asynchronous search to eliminate waiting time and fully utilize the hardware.

For each website, there will be a asynchronous task to handle the search of it, this is to maximize the utility of hardware resources and eliminate the time users wait for the result. This feature is realized in my program.

### 5. Data analysis

The web crawler should be able to conduct word frequency and sentiment analysis.

My web crawler can do both word frequency and sentiment analysis. But unfortunately, I am not capable enough to further utilize these results to generate some valuable insight.

###  6. Text user interface

The web crawler provides a user-friendly text interface, with "craw" and "export" buttons, also text boxes to input keywords and websites and also a text box to show the result.

My web crawler uses Cursive to quickly build the text user interface, it may not appear beautiful, but it is robust enough for some mis-operations.

## Features

### 1. Basic craw

The web crawler will search all text nodes in a given website, find those text nodes containing the keyword, and show the result in a user-friendly way, if one text node is extremely long, it will be truncated to 70 characters, this is also a consideration to improve the user's experience when he/she reads the result.

### 2. Website generation

If the user don't know which websites to craw, the program will use Google to search the keyword, and craw the first 100 websites. More concretely, we do "https://google?q=keyword&num=100", apparently this URL will not always include 100 search results, most of the time there will be fewer results, but according to my simple test, the number of websites is redundant enough.

### 3. Asynchronous craw

For each input website or Google found website, the web_main() in module web will create an asynchronous task for tokio runtime to schedule. This means all tasks are being conducted concurrently.

```rust
// take out a website from the vector
for a_website in websites_vec{
    ...
    // put the asynchronous task into the "tasks" vector
    tasks.push(
        tokio::spawn(async move {
            // search the keyword on the website that we just take
            if let Ok(report_entry) = single_webpage_search(kwd_for_pattern_matching, &a_website[..]).await{
                ...
            }
        })
    );
}
...
// the main thread will wait until all tasks are completed
let _ = join_all(tasks).await;
```

### 4. Data analysis

The web crawler will calculate how many times the keyword appears on a given website, and do sentiment analysis on the text nodes containing the keyword, a possible sentiment analysis looks like: ("pos" for positive, "neu" for neural, "neg" for negative)

```
Sentiment { pos: 0.59, neu: 0.41, neg: 0.0, compound: 0.8442 }
```

### 5. Text user interface

The text user interface looks like this:

![Text UI](https://github.com/yjmhyhc/WebCrawler/blob/master/img/UI.png "Text UI")

And the search results will be shown in this way:

![search results](https://github.com/yjmhyhc/WebCrawler/blob/master/img/search%20results.png "search results")

## User's guide

### 1. Build the project

In this GitHub repository,

1. click the "code" button
2. click the "Download Zip" button

![github user guide](https://github.com/yjmhyhc/WebCrawler/blob/master/img/github%20user%20guide.png "github user guide")

3. unzip the WebCrawler-master.zip

4. open your command line and change the directory to:

```
the_directory_where_you_put_the_unzipped_folder\WebCrawler-master\WebCrawler-master
```

In Linux (Ubuntu), this step can be done by:

a. press Ctrl + Alt + T to open the command line

b. input the following command and press Enter

```
cd the_directory_where_you_put_the_unzipped_folder/WebCrawler-master/WebCrawler-master
```

In Windows, it can be done by:

a. press Win + R

b. input "cmd" and press Enter

![windows cmd](https://github.com/yjmhyhc/WebCrawler/blob/master/img/windows%20cmd.png "windows cmd")

c. input the following command and press Enter

```
cd the_directory_where_you_put_the_unzipped_folder\WebCrawler-master\WebCrawler-master
```

5. input the following command and press Enter

```
cargo build
```

note: 

a. this command may take about 2 minutes to run, please be patient

b. if you have not installed Rust on your computer yet, please follow the instructions on: https://www.rust-lang.org/tools/install

### 2. Run the project

In the same directory, input the following command in your command line and press Enter

```
cargo run
```

Wait about 30 seconds, if you see the following text user interface, congratulations! You have already booted the web crawler

![Text UI](https://github.com/yjmhyhc/WebCrawler/blob/master/img/UI.png "Text UI")

### 3. Let's craw

![UI Instruction](https://github.com/yjmhyhc/WebCrawler/blob/master/img/UI%20Instruction.png "UI Instruction")

1. this is where you input the keyword you want to search, for example

```rust
Ken //this is a keyword
```

​		you need to notice that this keyword is case sensitive.

​		if you want to search multiple words, sure, just separate them with a ' ' like this

```
Ken Cammy Ryu//this keyword contains two words
```

​		you may also find that a lot of results are not relevant, try to separate the context from the actual item you want to craw!

​		for example,

```
Street Fighter/Ken
```

​		this command will craw the text containing "Ken", but it will only craw from those websites which are related to "Street Fighter Ken"

2. this is where you input the websites that you want to craw

​		for example,

```
https://google.com //this is a website to craw
```

​		without doubt, you can also craw multiple websites, just separate them with a ' ' like this

```
https://google.com https://youtube.com //craw two websites at a time
```

​		note: however, this input box is **not necessary**, if you don't know which websites to craw, just leave it blank, the web crawler will 		generate a few for you

3. this button starts the crawling process

​		after pressing this button, the textboxes and the buttons will be **disabled** to avoid errors, just wait a few seconds

4. this area shows the craw result, the content may not be complete if it is more than 3000 characters, but don't worry, **the content that you export will be complete**
5. this button exports the result to "craw result.txt" in the same directory that we run this project

## Lessons Learned and Concluding Remarks

1. the error of dangling reference seems to be the most frequent in my debugging process, this type of error occurs especially when there are closures in my code, as the closure may conduct some asynchronous tasks, it may live longer than the thread that calls the closure, so it is very important to:
   - a. use move keyword to move the ownership into the closure to avoid dangling reference
   - b. clone the data if we don't want to move the ownership into the closure
2. everything inside the async function should implement the "Send" trait, otherwise they cannot be send between threads safely
3. Rust is great, despite its long compile time, it almost do not fail at run time. But it is hard to learn Rust by oneself. I am very grateful having the opportunity to take 1724 Performant Software Systems with Rust this fall, and I also appreciate very much the great effort made by Professor. Li and his TAs, they have made all this possible, thank you!