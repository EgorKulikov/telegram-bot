use crate::types::*;

/// This object represents a checklist.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Checklist {
    pub title: String,
    #[serde(default)]
    pub tasks: Vec<ChecklistTask>,
    #[serde(default)]
    pub others_can_add_tasks: bool,
    #[serde(default)]
    pub others_can_mark_tasks_as_done: bool,
}

/// This object represents a task in a checklist.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChecklistTask {
    pub id: Integer,
    pub text: String,
    pub completed_by_user: Option<User>,
    pub completion_date: Option<Integer>,
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

// The `checklist_message` field (an optional full Message) is intentionally
// not modeled on the two service-message types below: nested messages can
// take shapes the Message deserializer rejects, and a required-but-absent
// field here once wedged getUpdates entirely.

/// This object represents a service message about checklist tasks completed.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChecklistTasksDone {
    #[serde(default)]
    pub marked_as_done_task_ids: Vec<Integer>,
    #[serde(default)]
    pub marked_as_not_done_task_ids: Vec<Integer>,
}

/// This object represents a service message about checklist tasks added.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChecklistTasksAdded {
    #[serde(default)]
    pub tasks: Vec<ChecklistTask>,
}
