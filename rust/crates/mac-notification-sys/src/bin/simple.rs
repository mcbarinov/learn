use mac_notification_sys::{Notification, send_notification};

fn main() {
    send_notification(
        "NOW",
        None,
        "Without subtitle",
        Some(Notification::new().sound("Blow")),
    )
    .unwrap();
}
