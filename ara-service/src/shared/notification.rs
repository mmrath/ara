use ara_model::core::NewNotification;
use failure::Error;

pub trait NotificationService {
    fn send(notification: NewNotification) -> Result<(), Error>;
}

pub struct DbNotificationService {}
