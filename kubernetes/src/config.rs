use cursive::{
    *,
    views::*,
};
use std::{
    sync::atomic::{AtomicUsize, Ordering},
    error::Error,
};
use kube::{
    Client,
    api::{Api, ResourceExt, ListParams, PostParams},
};
use k8s_openapi::api::core::v1::{Pod, ConfigMap, Service, Node};

//// Quit
pub fn yes(c: &mut Cursive) {
    c.quit();
}

pub fn no(c: &mut Cursive) {
    c.pop_layer();
}

pub fn menu(c: &mut Cursive) {
    c.select_menubar();
}

//// Kube Client
#[tokio::main]
pub async fn get_pod(c: &mut Cursive) -> Result<(), Box<dyn Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let lp = ListParams::default();
    Ok(())
}

//// Resource
pub fn pod(c: &mut Cursive) {
    let pod = Pod::default();
    c.add_layer(Dialog::info(
        format!("{:#?}", pod)
    ));
}

pub fn configmap(c: &mut Cursive) {
    let configmap = ConfigMap::default();
    c.add_layer(Dialog::info(
        format!("{:#?}", configmap)
    ));
}

pub fn service(c: &mut Cursive) {
    let service = Service::default();
    c.add_layer(Dialog::info(
        format!("{:#?}", service)
    ));
}

pub fn node(c: &mut Cursive) {
    let node = Node::default();
    c.add_layer(Dialog::info(
        format!("{:#?}", node)
    ));
}

//// Help
pub fn key(c: &mut Cursive) {
    let key = "Esc : Quit\n\
                     Tab : Menu";
    c.add_layer(Dialog::info(key));
}

pub fn repository(c: &mut Cursive) {
    let github = "https://github.com/gunwoo8873/Test-Repo.git";
    c.add_layer(Dialog::info(github));
}

pub fn version(c: &mut Cursive) {
    let version = "2024. 10. 21\n\
                        Builder: Choigunwoo\n\
                        Version: B0.0.1";
    c.add_layer(Dialog::info(version));
}