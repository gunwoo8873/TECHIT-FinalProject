#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use {
    std::error::Error,
    kube::{
        api::{ObjectList, ListParams},
        Client,
        Api
    },
    k8s_openapi::{
        api::core::v1::Pod
    },
    tokio,
};

#[tokio::main]
async fn kube_run() -> Result<(), Box<dyn Error>> {
    let client = kube::Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let lp = ListParams::default();
    let pl = pods.list(&lp).await?;

    println!("{:?}", pl);
    Ok(())
}

slint::include_modules!();
fn gui_run() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    ui.set_counter(0);

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;
    Ok(())
}

fn main() {
    gui_run().unwrap();
}