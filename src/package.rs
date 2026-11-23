use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Package {
    #[serde(rename = "package_pname")]
    pub name: String,

    #[serde(rename = "package_attr_name")]
    pub attr_name: String,

    #[serde(rename = "package_attr_set")]
    pub attr_set: String,

    #[serde(rename = "package_description")]
    pub description: String,

    #[serde(rename = "package_pversion")]
    pub version: String,

    #[serde(rename = "package_outputs")]
    pub outputs: Vec<String>,

    #[serde(rename = "package_programs")]
    pub programs: Vec<String>,

    #[serde(rename = "package_homepage")]
    pub homepage: Vec<String>,

    #[serde(rename = "package_platforms")]
    pub platforms: Vec<String>,

    #[serde(rename = "package_position")]
    pub position: String,
}
