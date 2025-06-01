use rdev::{Event, EventType, Key};

pub fn is_super_key(event: &Event) -> bool {
    matches!(event.event_type, EventType::KeyPress(Key::MetaLeft) | EventType::KeyPress(Key::MetaRight))
}

pub fn is_key(event: &Event, key: Key) -> bool {
    matches!(event.event_type, EventType::KeyPress(k) if k == key)
}
