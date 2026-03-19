use crate::types::*;

/// This object represents a chat background.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBackground {
    #[serde(rename = "type")]
    pub type_: BackgroundType,
}

/// This object describes the type of a background.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundType {
    #[serde(rename = "fill")]
    Fill {
        fill: BackgroundFill,
        dark_theme_dimming: Integer,
    },
    #[serde(rename = "wallpaper")]
    Wallpaper {
        document: Document,
        dark_theme_dimming: Integer,
        is_blurred: Option<bool>,
        is_moving: Option<bool>,
    },
    #[serde(rename = "pattern")]
    Pattern {
        document: Document,
        fill: BackgroundFill,
        intensity: Integer,
        is_inverted: Option<bool>,
        is_moving: Option<bool>,
    },
    #[serde(rename = "chat_theme")]
    ChatTheme {
        theme_name: String,
    },
}

/// This object describes the way a background is filled based on the selected colors.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundFill {
    #[serde(rename = "solid")]
    Solid { color: Integer },
    #[serde(rename = "gradient")]
    Gradient {
        top_color: Integer,
        bottom_color: Integer,
        rotation_angle: Integer,
    },
    #[serde(rename = "freeform_gradient")]
    FreeformGradient { colors: Vec<Integer> },
}
