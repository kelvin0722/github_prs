use mac_notification_sys::NotificationResponse;
use webbrowser;

pub fn handle_response(response: &NotificationResponse, action1: &str, action2: &str, url: &str) {
    match response {
        // Requires main_button to be a MainButton::SingleAction or MainButton::DropdownActions
        NotificationResponse::ActionButton(action_name) => {
            if action_name == action1 {
                webbrowser::open(url).unwrap();
            } else if action_name == action2 {
                println!("Clicked on Action 2")
            }
        }
        NotificationResponse::Click => println!("Clicked on the notification itself"),
        NotificationResponse::CloseButton(close_name) => println!(
            "Dismissed the notification with the close button called {}",
            close_name
        ),
        // Requires main_button to be a MainButton::Response
        NotificationResponse::Reply(response) => {
            println!("Replied to the notification with {}", response)
        }
        NotificationResponse::None => println!("No interaction with the notification occured"),
    };
}
