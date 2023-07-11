use std::collections:: BTreeMap;

use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
use cursive::theme::load_toml;

pub fn launch_gui(marks: BTreeMap<String, String>) {
    // creating element
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    select.add_all_str(marks.clone().into_keys());

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(move |s, alias: &str| {
        let path = marks.get(alias).unwrap();
        s.quit();
        println!("{path}");
    } );

    // Let's override the `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    let mut siv = Cursive::default();
    siv.set_theme(load_toml(include_str!("./resources/theme.toml")).unwrap());

    // adding interactive options
    siv.add_layer(
        Dialog::around(select.scrollable().fixed_size((20, 10))).title("Your Clips"),
    );

    siv.run();
}
