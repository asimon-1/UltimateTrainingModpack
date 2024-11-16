use skyline::nn::ui2d::ResColor;

use training_mod_sync::*;

pub static NOTIFICATIONS_QUEUE: RwLock<Vec<Notification>> = RwLock::new(vec![]);

#[derive(Clone)]
pub struct Notification {
    pub header: String,
    pub message: String,
    length: u32,
    pub color: ResColor,
    has_drawn: bool,
}

impl Notification {
    pub fn new(header: String, message: String, length: u32, color: ResColor) -> Notification {
        Notification {
            header,
            message,
            length,
            color,
            has_drawn: false,
        }
    }

    pub fn set_drawn(&mut self) {
        self.has_drawn = true;
    }

    pub fn has_drawn(&mut self) -> bool {
        self.has_drawn
    }

    pub fn tick(&mut self) {
        self.length -= 1;
    }

    pub fn has_completed(&self) -> bool {
        self.length <= 1
    }
}

pub fn notification(header: String, message: String, len: u32) {
    let mut queue_lock = lock_write(&NOTIFICATIONS_QUEUE);
    (*queue_lock).push(Notification::new(
        header,
        message,
        len,
        ResColor {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        },
    ));
    drop(queue_lock);
}

pub fn color_notification(header: String, message: String, len: u32, color: ResColor) {
    let mut queue_lock = lock_write(&NOTIFICATIONS_QUEUE);
    (*queue_lock).push(Notification::new(header, message, len, color));
    drop(queue_lock);
}

pub fn clear_notification(header: &'static str) {
    if (*lock_read(&NOTIFICATIONS_QUEUE)).is_empty() {
        // Before acquiring an exclusive write lock, check if there are even any notifications to clear out
        return;
    }
    let mut queue_lock = lock_write(&NOTIFICATIONS_QUEUE);
    (*queue_lock).retain(|notif| notif.header != header);
    drop(queue_lock);
}

pub fn clear_all_notifications() {
    if (*lock_read(&NOTIFICATIONS_QUEUE)).is_empty() {
        // Before acquiring an exclusive write lock, check if there are even any notifications to clear out
        return;
    }
    let mut queue_lock = lock_write(&NOTIFICATIONS_QUEUE);
    (*queue_lock).clear();
    drop(queue_lock);
}
