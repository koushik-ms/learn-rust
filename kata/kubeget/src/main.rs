// use k8s_openapi::api::core::v1::Pod;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{Client, Api};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let client = Client::try_default().await?;
    let cm_api: Api<ConfigMap> = Api::namespaced(client, "kube-system");
    let cm = cm_api.get("coredns").await?;
    if let Some(data) = cm.data {
        println!("Obtained Data: {:?}", data);
    }
    Ok(())
}
