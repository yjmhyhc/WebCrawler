mod web;

use cursive::views::{Dialog, EditView, LinearLayout, TextView, Button, ScrollView, Panel};
use cursive::{Cursive, CursiveExt};
use cursive::traits::{Nameable, Resizable};
use cursive::theme::{BaseColor, Color};
use crate::web::ReportEntry;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {

    let mut siv = Cursive::default();

    // the form to contain input information
    let form = LinearLayout::vertical()
        .child(
            LinearLayout::vertical()
                .child(TextView::new("Keyword:(seperated with ' ', case sensitive)").fixed_width(30))
                .child(TextView::new("you can also separate the context from the actual item using '/', \
                for example, \"Street Fighter/Ken\" means you want to find content related to the character named Ken in game Street Fighter, \
                not Ken in any other place. This is useful if you want sentences containing \"Ken\", \
                rather than sentences containing all three words \"Street Fighter Ken\"").style(Color::Dark(BaseColor::Blue)).fixed_width(70))
                .child(Panel::new(EditView::new().with_name("Keyword").fixed_width(70))),
        )
        .child(
            LinearLayout::vertical()
                .child(TextView::new("Websites:(seperated with ' ')").fixed_width(30))
                .child(Panel::new(EditView::new().with_name("Websites").fixed_width(70))),
        )
        .child(
            Button::new("Craw !", |s| {
                //disable the EditViews and Buttons first to avoid error
                s.call_on_name("Keyword", |view: &mut EditView| {
                    view.set_enabled(false);
                });
                s.call_on_name("Websites", |view: &mut EditView| {
                    view.set_enabled(false);
                });
                s.call_on_name("Craw", |view: &mut Button| {
                    view.set_enabled(false);
                });
                s.call_on_name("Export", |view: &mut Button| {
                    view.set_enabled(false);
                });

                // obtain the information in the input boxes
                let keyword = s
                    .call_on_name("Keyword", |view: &mut EditView| view.get_content())
                    .unwrap();
                let websites = s
                    .call_on_name("Websites", |view: &mut EditView| view.get_content())
                    .unwrap();
                // use sink to update the UI thread safely
                let sink = s.cb_sink().clone();

                // create an async task, as Cursive's event handling is synchronous based
                tokio::spawn(async move{
                    // format the result or handle the error if search fails
                    if let Ok(result_vec) = web::web_main(&keyword[..], &websites[..]).await{
                        let result_string = format_the_result(result_vec);
                        // update the output box
                        sink.send(Box::new(|s| {
                            s.call_on_name("output", |view: &mut TextView| {
                                view.set_content(result_string);
                            }).unwrap();
                            s.add_layer(
                                Dialog::new()
                                    .title("Information")
                                    .content(TextView::new("craw complete!"))
                                    .button("Ok", |s| {
                                        //re-enable the EditViews and Buttons
                                        s.call_on_name("Keyword", |view: &mut EditView| {
                                            view.set_enabled(true);
                                        });
                                        s.call_on_name("Websites", |view: &mut EditView| {
                                            view.set_enabled(true);
                                        });
                                        s.call_on_name("Craw", |view: &mut Button| {
                                            view.set_enabled(true);
                                        });
                                        s.call_on_name("Export", |view: &mut Button| {
                                            view.set_enabled(true);
                                        });
                                        s.pop_layer(); // remove the dialog box
                                    }),
                            );
                        })).unwrap();
                    }else {
                        // pop a message box to inform the user of an error
                        sink.send(Box::new(|s| {
                            s.add_layer(
                                Dialog::new()
                                    .title("Error")
                                    .content(TextView::new("sorry, craw failed due to network issue"))
                                    .button("Ok", |s| {
                                        //re-enable the EditViews and Buttons
                                        s.call_on_name("Keyword", |view: &mut EditView| {
                                            view.set_enabled(true);
                                        });
                                        s.call_on_name("Websites", |view: &mut EditView| {
                                            view.set_enabled(true);
                                        });
                                        s.call_on_name("Craw", |view: &mut Button| {
                                            view.set_enabled(true);
                                        });
                                        s.call_on_name("Export", |view: &mut Button| {
                                            view.set_enabled(true);
                                        });
                                        s.pop_layer(); // remove the dialog box
                                    }),
                            );
                        })).unwrap();
                    }
                });
            }).with_name("Craw"),
        );

    // the output box with a export button
    let output = LinearLayout::vertical()
        .child(TextView::new("Found results:"))
        .child(
            Panel::new(
                ScrollView::new(
                    TextView::new("").with_name("output")
                ).scroll_x(true).scroll_y(true)
            )
        )
        .child(Button::new("Export", |s|{
            //disable the EditViews and Buttons first to avoid error
            s.call_on_name("Keyword", |view: &mut EditView| {
                view.set_enabled(false);
            });
            s.call_on_name("Websites", |view: &mut EditView| {
                view.set_enabled(false);
            });
            s.call_on_name("Craw", |view: &mut Button| {
                view.set_enabled(false);
            });
            s.call_on_name("Export", |view: &mut Button| {
                view.set_enabled(false);
            });

            //export to a file
            if let Ok(mut file) = File::create("craw result.txt"){
                let output = s
                    .call_on_name("output", |view: &mut TextView| String::from(view.get_content().source()))
                    .unwrap();
                let _ = file.write_all(output.as_bytes());
            }

            //show a dialog box
            s.add_layer(
                Dialog::new()
                    .title("Information")
                    .content(TextView::new("export complete!"))
                    .button("Ok", |s| {
                        //re-enable the EditViews and Buttons
                        s.call_on_name("Keyword", |view: &mut EditView| {
                            view.set_enabled(true);
                        });
                        s.call_on_name("Websites", |view: &mut EditView| {
                            view.set_enabled(true);
                        });
                        s.call_on_name("Craw", |view: &mut Button| {
                            view.set_enabled(true);
                        });
                        s.call_on_name("Export", |view: &mut Button| {
                            view.set_enabled(true);
                        });
                        s.pop_layer(); // remove the dialog box
                    }),
            );
        }).with_name("Export"));

    let layout = LinearLayout::vertical()
        .child(form)
        .child(output);

    siv.add_layer(Dialog::around(layout).title("WebCrawler"));
    siv.run();
}

/// this function formats the result in a more readable way
///
/// # parameters
/// - 'result_vec': a Vec<ReportEntry> that is returned from web_main function
///
/// # return value
/// a string to be displayed in the Cursive text UI
fn format_the_result(result_vec: Vec<ReportEntry>) -> String{
    let mut result_string = String::from("");
    let mut idx = 1;
    for re in result_vec {
        result_string.push_str(&format!("❖  Webpage #{}\n", idx));
        result_string.push_str("-----------------------------------------Below-is-meta-information-------------------------------------------\n");
        result_string.push_str(&format!("➤  title: {}\n", re.title()));
        result_string.push_str(&format!("➤  url: {}\n", re.url()));
        result_string.push_str(&format!("➤  snippet: {}\n", re.snippet()));
        result_string.push_str(&format!("➤  frequency of the keyword: {}\n", re.frequency()));
        result_string.push_str(&format!("➤  latest updated time: {}\n", re.update_time()));
        result_string.push_str("------------------------------------------Below-are-search-results-------------------------------------------\n");

        // a counter to record the # of matches
        let mut index = 1;
        for a_match in re.results_list() {
            result_string.push_str(&format!("➤  {}. {}\n", index, a_match.0));
            result_string.push_str(&format!("➤  sentiment analysis: {}\n", a_match.1));
            index += 1;
        }
        result_string.push_str("\n\n");
        idx += 1;
    }
    return result_string;
}