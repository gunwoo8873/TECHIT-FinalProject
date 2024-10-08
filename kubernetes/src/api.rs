use kube::{
    Api,
    Client,
    api::{ListParams},
};
use k8s_openapi::api::core::v1::{
    Pod, Service, ConfigMap,
};
use std::{
    error::Error,
};

async fn create_clinet() -> Result<Client, Box<dyn Error>> {
    let config = kube::Config::from_kubeconfig(
        &kube::config::KubeConfigOptions::default()
    ).await?;

    let client = Client::try_from(config)?;

    Ok(client)
}
#[tokio::main]
pub async fn get_pods() -> Result<(), Box<dyn Error>> {
    let client = create_clinet().await?;
    let lp = ListParams::default();

    let pods: Api<Pod> = Api::default_namespaced(client);
    for p in pods.list(&lp).await? {
        println!("Found Pod: {}", p.metadata.name.unwrap_or_default());
    }

    Ok(())
}

#[tokio::main]
pub async fn get_services() -> Result<(), Box<dyn Error>> {
    let client = create_clinet().await?;
    let lp = ListParams::default();

    let services: Api<Service> = Api::default_namespaced(client);
    for s in services.list(&lp).await? {
        println!("Found Service: {}", s.metadata.name.unwrap_or_default());
    }

    Ok(())
}

#[tokio::main]
pub async fn get_configmaps() -> Result<(), Box<dyn Error>> {
    let client = create_clinet().await?;
    let lp = ListParams::default();

    let configmaps: Api<ConfigMap> = Api::default_namespaced(client);
    for c in configmaps.list(&lp).await? {
        println!("Found ConfigMap: {}", c.metadata.name.unwrap_or_default());
    }

    Ok(())
}