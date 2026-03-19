/// This object represents an inline keyboard button that copies specified text to the clipboard.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct CopyTextButton {
    pub text: String,
}
