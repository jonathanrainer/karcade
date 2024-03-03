use kube::CustomResource;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use serde_json;

#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
group = "karcade.io",
version = "v1",
kind = "Game",
namespaced,
doc = "A Custom Resource to indicate the presence of a Game running in the cluster",
derive = "PartialEq",
singular = "game",
plural = "games",
shortname = "gm",
schema = "disabled"
)]
#[serde(rename_all = "camelCase")]
pub struct GameSpec {
    rom_path: String
}