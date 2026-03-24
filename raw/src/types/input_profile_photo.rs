/// This object describes a profile photo to set.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum InputProfilePhoto {
    #[serde(rename = "static")]
    Static {
        photo: String,
    },
    #[serde(rename = "animated")]
    Animated {
        animation: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        main_frame_timestamp: Option<f64>,
    },
}
