mod web;

use cursive::views::{Dialog, EditView, LinearLayout, TextView, Button, ScrollView, Panel};
use cursive::{Cursive, CursiveExt};
use cursive::traits::{Nameable, Resizable};

#[tokio::main]
async fn main() {

    let url = "https://en.wikipedia.org/wiki/Search_engine";
    let keyword = "Google";
    if let Ok(a) = web::single_webpage_single_keyword(url, keyword).await{
        println!("{:?}",a);
        if let Some(x) = a.results_list.get(0) {
            println!("{}", x.1);
        }
    }

    // let mut siv = Cursive::default();
    //
    // // the form to contain input information
    // let form = LinearLayout::vertical()
    //     .child(
    //         LinearLayout::vertical()
    //             .child(TextView::new("Keywords:").fixed_width(12))
    //             .child(Panel::new(EditView::new().with_name("Keywords").fixed_width(70))),
    //     )
    //     .child(
    //         LinearLayout::vertical()
    //             .child(TextView::new("Websites:").fixed_width(12))
    //             .child(Panel::new(EditView::new().with_name("Websites").fixed_width(70))),
    //     )
    //     .child(
    //         LinearLayout::vertical()
    //             .child(TextView::new("Iterations:").fixed_width(12))
    //             .child(Panel::new(EditView::new().with_name("Iterations").fixed_width(70))),
    //     )
    //     .child(
    //         Button::new("Craw !", |s| {
    //             // obtain the information in the input boxes
    //             let keywords = s
    //                 .call_on_name("Keywords", |view: &mut EditView| view.get_content())
    //                 .unwrap();
    //             let websites = s
    //                 .call_on_name("Websites", |view: &mut EditView| view.get_content())
    //                 .unwrap();
    //             let iterations = s
    //                 .call_on_name("Iterations", |view: &mut EditView| view.get_content())
    //                 .unwrap();
    //
    //             // format the output
    //             let output = format!("Keywords: {}\nWebsites: {}\nIterations: {}", keywords, websites, iterations);
    //
    //             // update the output box
    //             s.call_on_name("output", |view: &mut TextView| {
    //                 view.set_content(output);
    //             });
    //         }),
    //     );
    //
    // // the output box with a export button
    // let output = LinearLayout::vertical()
    //     .child(TextView::new("Found results:"))
    //     .child(
    //         Panel::new(
    //             ScrollView::new(
    //                 TextView::new("").with_name("output")
    //             ).scroll_x(true).scroll_y(true)
    //         )
    //     )
    //     .child(Button::new("Export", |_s|{}));
    //
    // let layout = LinearLayout::vertical()
    //     .child(form)
    //     .child(output);
    //
    // siv.add_layer(Dialog::around(layout).title("WebCrawler"));
    // siv.run();
}
