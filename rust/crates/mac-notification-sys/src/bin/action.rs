use mac_notification_sys::{MainButton, Notification, send_notification};

fn main() {
    let response = send_notification(
        "Single Action",
        None,
        "ok?",
        Some(
            Notification::new()
                .main_button(MainButton::SingleAction("Ok"))
                .sound("Blow"),
        ),
    )
    .unwrap();
    dbg!(response);
}
