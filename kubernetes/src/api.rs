use k8s_openapi::api::core::v1::{
    Pod, Service, ConfigMap, PodSpec, PodStatus, Container,
};
use kube::{
    Config,
    Client,
    Api,
    config::KubeConfigOptions,
    api::{ListParams, DeleteParams, Resource},
};
use std::error::Error;

#[tokio::main]
async fn kube_client() -> Result<(), Box<dyn Error>> {
    let config = Config::from_kubeconfig(
        &KubeConfigOptions::default()
    ).await?;
    let client = Client::try_from(config)?;

    Ok(())
}

enum Resources {
    Pod(Pod),
    PodSpec(PodSpec),
    PodStatus(PodStatus),
    Service(Service),
}

// #[tokio::main]
// pub async fn get_resource() -> Result<Vec<Resources>, Box<dyn Error>> {
//     let client = kube_client().await?;
//     let lp = ListParams::default();
//
//     Ok()
// }