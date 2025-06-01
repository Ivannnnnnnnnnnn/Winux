use rdev::{listen, Event};

pub async fn start_key_listener<F: Fn(String) + Send + 'static>(callback: F) {
    tokio::spawn(async move {
        let _ = listen(move |event: Event| {
            if let Some(key) = event.name {
                callback(key);
            }
        });
    });
}
