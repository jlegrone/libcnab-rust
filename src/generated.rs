#[derive(Serialize, Deserialize)]
struct AdditionalProperties {
  #[serde(rename = "type")]
  _type: String,
  properties: Properties9,
}

#[derive(Serialize, Deserialize)]
struct AdditionalProperties1 {
  allOf: Vec<Items2>,
}

#[derive(Serialize, Deserialize)]
struct Custom {
  #[serde(rename = "$comment")]
  _comment: String,
}

#[derive(Serialize, Deserialize)]
struct Definitions {
  invocationImage: Items1,
  image: Items1,
  credential: Platform,
  parameter: Items1,
  output: Items1,
}

#[derive(Serialize, Deserialize)]
struct Images {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  additionalProperties: AdditionalProperties,
}

#[derive(Serialize, Deserialize)]
struct Items {
  #[serde(rename = "type")]
  _type: String,
}

#[derive(Serialize, Deserialize)]
struct Items1 {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  properties: Properties10,
  required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Items2 {
  #[serde(rename = "$ref")]
  _ref: String,
}

#[derive(Serialize, Deserialize)]
struct Keywords {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  items: Items,
}

#[derive(Serialize, Deserialize)]
struct Parameters {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  properties: Properties3,
  additionalProperties: AdditionalProperties1,
}

#[derive(Serialize, Deserialize)]
struct Platform {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  properties: Properties7,
}

#[derive(Serialize, Deserialize)]
struct Properties {
  schemaVersion: SchemaVersion,
  name: SchemaVersion,
  version: Version,
  description: SchemaVersion,
  keywords: Keywords,
  maintainers: Keywords,
  license: SchemaVersion,
  invocationImages: Keywords,
  images: Images,
  credentials: Images,
  actions: Images,
  custom: Custom,
  parameters: Parameters,
  outputs: Parameters,
}

#[derive(Serialize, Deserialize)]
struct Properties1 {
  name: SchemaVersion,
  email: SchemaVersion,
  url: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct Properties10 {
  applyTo: Keywords,
  path: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct Properties2 {
  modifies: SchemaVersion,
  description: SchemaVersion,
  stateless: Stateless,
}

#[derive(Serialize, Deserialize)]
struct Properties3 {
  required: Items2,
}

#[derive(Serialize, Deserialize)]
struct Properties4 {
  image: SchemaVersion,
  originalImage: SchemaVersion,
  imageType: Stateless,
  digest: SchemaVersion,
  size: SchemaVersion,
  platform: Platform,
  mediaType: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct Properties5 {
  architecture: SchemaVersion,
  os: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct Properties6 {
  description: SchemaVersion,
  image: SchemaVersion,
  originalImage: SchemaVersion,
  imageType: Stateless,
  digest: SchemaVersion,
  size: SchemaVersion,
  platform: Platform,
  mediaType: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct Properties7 {
  path: SchemaVersion,
  env: SchemaVersion,
  description: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct Properties8 {
  destination: AdditionalProperties,
  #[serde(rename = "apply-to")]
  _applyTo: Keywords,
}

#[derive(Serialize, Deserialize)]
struct Properties9 {
  path: SchemaVersion,
  env: SchemaVersion,
}

#[derive(Serialize, Deserialize)]
struct RootInterface {
  #[serde(rename = "$schema")]
  _schema: String,
  #[serde(rename = "$id")]
  _id: String,
  title: String,
  description: String,
  #[serde(rename = "type")]
  _type: String,
  properties: Properties,
  required: Vec<String>,
  definitions: Definitions,
  additionalProperties: bool,
}

#[derive(Serialize, Deserialize)]
struct SchemaVersion {
  description: String,
  #[serde(rename = "type")]
  _type: String,
}

#[derive(Serialize, Deserialize)]
struct Stateless {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  default: String,
}

#[derive(Serialize, Deserialize)]
struct Version {
  description: String,
  #[serde(rename = "type")]
  _type: String,
  pattern: String,
}
