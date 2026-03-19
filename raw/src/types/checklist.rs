use crate::types::*;

/// This object represents a checklist.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Checklist {
    pub title: String,
    pub tasks: Vec<ChecklistTask>,
}

/// This object represents a task in a checklist.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChecklistTask {
    pub id: Integer,
    pub text: String,
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(default)]
    pub is_completed: bool,
}

/// This object represents a checklist to send.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputChecklist {
    pub title: String,
    pub tasks: Vec<InputChecklistTask>,
}

/// This object represents a task in a checklist to send.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InputChecklistTask {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
}

/// This object represents a service message about checklist tasks completed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChecklistTasksDone {
    pub checklist_message_id: Integer,
    pub tasks: Vec<ChecklistTask>,
}

/// This object represents a service message about checklist tasks added.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChecklistTasksAdded {
    pub checklist_message_id: Integer,
    pub tasks: Vec<ChecklistTask>,
}
