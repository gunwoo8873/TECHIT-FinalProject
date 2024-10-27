use cursive::{event::Key};
use cursive::*;
use cursive::style::{BorderStyle, Palette};
use cursive::views::{Dialog, TextView};
use kube::{Client, api::{Api, ResourceExt, ListParams, PostParams}};
use k8s_openapi::api::core::v1::{Pod};
use std::error::Error;
use sysinfo::{Cpu, Networks, Disks};

pub fn system_info()
{}

//// Kubernetes Resource Client
#[tokio::main]
async fn kube_client() -> Result<(), Box<dyn Error>>
{
}

//// Resource Function
async fn get_pods() -> Result<Client, Box<dyn Error>>
{
}

pub fn pods()
{}

pub fn services()
{}

pub fn configmaps()
{}

pub fn nodes()
{}

pub fn networks()
{}

pub fn cpu_use()
{}

pub fn memory_use()
{}

pub fn gpu_use()
{}

//// Terminal UI Style
pub fn ui_style(c: &mut Cursive)
{
    c.load_toml(include_str!("../App.toml")).unwrap();
}

//// Help
pub fn key(c: &mut Cursive)
{
    let key = "Esc : Quit\n\
                     Tab : Menu";
    c.add_layer(Dialog::info(key));
}

pub fn repository(c: &mut Cursive)
{
    let github = "https://github.com/gunwoo8873/Test-Repo.git";
    c.add_layer(Dialog::info(github));
}

pub fn version(c: &mut Cursive)
{
    let version = "2024. 10. 21\n\
                        Builder: Choigunwoo\n\
                        Version: B0.0.1";
    c.add_layer(Dialog::info(version));
}

//// Key Press Events
pub fn key_handler(c: &mut Cursive)
{
    c.add_global_callback
    (
        Key::Tab,
        |c| {c.select_menubar()}
    );

    c.add_global_callback
    (
        Key::Esc,
        |c| {c.add_layer(
            Dialog::new()
                .title("Quit")
                .content(TextView::new("Are you sure?"))
                .button("Yes", yes)
                .button("No", no)
        )}
    );
}

//// Exit Select Button
pub fn yes(c: &mut Cursive)
{
    c.quit();
}

pub fn no(c: &mut Cursive)
{
    c.pop_layer();
}