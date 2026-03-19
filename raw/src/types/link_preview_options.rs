/// Describes the options used for link preview generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}
