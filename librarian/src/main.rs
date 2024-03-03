mod crds;

use futures::prelude::*;
use kube::{Client, api::Api, ResourceExt};
use kube::runtime::{watcher, WatchStreamExt};
use crate::crds::Game;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Booting Librarian Operator....");
    let client = Client::try_default().await?;
    let games = Api::<Game>::default_namespaced(client);

    let wc = watcher::Config::default();

    watcher(games, wc).applied_objects().default_backoff().try_for_each(
        |g| async move {
            println!("found {:?}", g.name_any());
            Ok(())
        }
    ).await?;

    Ok(())
}