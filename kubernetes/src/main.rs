use cursive::*;
use cursive::views::*;
use kubernetes::config::{*};

fn main()
{
    let mut c = default();
    key_handler(&mut c);
    ui_style(&mut c);
    menu(&mut c);
    c.run();
}

fn menu(c: &mut Cursive)
{
    c.menubar()
        //// Kubernetes Resource
        .add_subtree
        (
            "Resource",
            menu::Tree::new()
                //// Kubernetes Resource Information
                .leaf("Pod", move |c| { pods() })
                .leaf("Service", move |c| { services() })
                .leaf("ConfigMap", move |c| c.add_layer(Dialog::info("Content")))
                .leaf("Node", move |c| c.add_layer(Dialog::info("Content")))
                .leaf("Network", move |c| c.add_layer(Dialog::info("Content")))
                .delimiter()
                //// CPU, Memory, GPU Used Information
                .leaf("CPU", move |c| c.add_layer(Dialog::info("Content")))
                .leaf("Memory", move |c| c.add_layer(Dialog::info("Content")))
                .leaf("GPU", move |c| c.add_layer(Dialog::info("Content")))
        )
        //// UI Options
        .add_subtree
        (
            "Options",
            menu::Tree::new()
                .leaf("Theme", move |c| c.add_layer(Dialog::info("Content")))
                .leaf("Text", move |c| c.add_layer(Dialog::info("Content")))
        )
        //// TUI Help
        .add_subtree
        (
            "Help",
            menu::Tree::new()
                .leaf("Key", move |c| { key(c) })
                .leaf("Repository", move |c| {repository(c)})
                .leaf("Version", move |c| {version(c)})
                .leaf("Update", move |c| {})
        )
        .add_leaf
        (
            "Exit",
            |c| c.add_layer
            (
                Dialog::new()
                    .title("Quit")
                    .content(TextView::new("Are you sure?"))
                    .button("Yes", yes)
                    .button("No", no)
            )
        );
}