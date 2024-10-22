use cursive::{
    *,
    views::*,
    event::Key,
    // traits::*,
};
// use std::sync::atomic::{AtomicUsize, Ordering};
use kubernetes::config::*;

fn main() {
    let mut app = default();

    //// Global Theme Setup
    app.load_toml(include_str!("../App.toml")).unwrap();

    //// Global Callback Key Event
    app.add_global_callback(Key::Esc, |c| c.add_layer(
        Dialog::new()
            .title("Quit")
            .button("Yes", yes)
            .button("No", no)
    ));

    app.add_global_callback(Key::Tab, menu);

    //// Menu
    app.menubar()
        .add_subtree(
            "Resource",
            menu::Tree::new()
                .leaf("Pod", move |c| {pod(c)})
                .leaf("Service", move |c| {service(c)})
                .leaf("ConfigMap", move |c| {configmap(c)})
                .leaf("Network", move |c| {})
                .leaf("Node", move |c| {node(c)})
                .delimiter()
                .leaf("CPU", move |c| {})
                .leaf("Memory", move |c| {})
                .leaf("GPU", move |c| {})
        )
        .add_subtree(
            "Option",
            menu::Tree::new()
                .leaf("Color", move |c| {})
                .leaf("Text", move |c| {})
        )
        .add_subtree(
            "Help",
            menu::Tree::new()
                .leaf("Key", move |c| {key(c)})
                .leaf("Repository", move |c| {repository(c)})
                .leaf("Version", move |c| {version(c)})
        )
        .add_leaf("Monitoring", |c| {});
    app.run();
}