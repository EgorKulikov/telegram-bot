use std::fs::File;
use std::io::prelude::*;

use telegram_bot_raw::types::message::MessageKind;
use telegram_bot_raw::types::update::{Update, UpdateKind};

macro_rules! make_test {
    ($asset: ident, $test: expr) => {
        #[test]
        fn $asset() {
            let data = {
                let filename = format!("tests/update_assets/{}.json", stringify!($asset));
                let mut data = Vec::new();
                let mut file = File::open(filename).unwrap();
                file.read_to_end(&mut data).unwrap();
                data
            };
            let update = serde_json::from_slice::<Update>(&data).unwrap();
            $test(update)
        }
    };
}

make_test!(migrate_from_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateFromChatId { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

make_test!(migrate_to_chat_id, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::MigrateToChatId { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

make_test!(inline_query, |update: Update| {
    if let UpdateKind::InlineQuery(_query) = update.kind {
        return ();
    }

    assert!(false)
});

make_test!(regression_test_208, |update: Update| {
    if let UpdateKind::CallbackQuery(_query) = update.kind {
        return ();
    }

    assert!(false)
});

make_test!(checklist_tasks_added, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::ChecklistTasksAdded { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

make_test!(checklist_tasks_done, |update: Update| {
    if let UpdateKind::Message(message) = update.kind {
        if let MessageKind::ChecklistTasksDone { .. } = message.kind {
            return ();
        }
    }
    assert!(false)
});

#[test]
fn unparseable_update_falls_back_to_error_kind() {
    let data = br#"{"update_id":42,"some_future_update_type":{"foo":1}}"#;
    let update = serde_json::from_slice::<Update>(data).unwrap();
    assert_eq!(update.id, 42);
    match update.kind {
        UpdateKind::Error(_) | UpdateKind::Unknown => (),
        other => panic!("expected Error/Unknown kind, got {:?}", other),
    }
}
