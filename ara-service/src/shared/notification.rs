use failure::Error;
use ara_model::core::NewNotification;

pub trait NotificationService {
    fn send(notification: NewNotification) -> Result<(), Error>;
}

pub struct DbNotificationService {}
