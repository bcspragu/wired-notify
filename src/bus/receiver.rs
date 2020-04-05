use std::collections::HashMap;
use std::sync::mpsc::Sender;

use dbus::arg;
use dbus::tree;
use crate::bus::dbus::DBusNotification;

use super::dbus_codegen::{ OrgFreedesktopNotifications };

#[derive(Copy, Clone, Default, Debug)]
pub struct BusNotification;
impl OrgFreedesktopNotifications for BusNotification {
    //type Err = dbus::tree::MethodErr;
    fn close_notification(&self, _id: u32) -> Result<(), tree::MethodErr> {
        Ok(())
    }

    fn get_capabilities(&self) -> Result<Vec<String>, tree::MethodErr> {
        let capabilities: Vec<String> = vec![
            "actions".to_string(),
            "body".to_string(),
            "body-hyperlinks".to_string(),
            "body-markup".to_string(),
            "icon-static".to_string(),
            "sound".to_string(),
            "persistence".to_string(),
            "action-icons".to_string(),
        ];

        Ok(capabilities)
    }

    // TODO: fill out this.
    fn get_server_information(&self) -> Result<(String, String, String, String), tree::MethodErr> {
        Ok((
            "dummy".to_string(),
            "dummy".to_string(),
            "dummy".to_string(),
            "dummy".to_string(),
        ))
    }

    fn notify(
        &self,
        sender: Sender<DBusNotification>,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        _actions: Vec<&str>,
        _hints: HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
        expire_timeout: i32,
        ) -> Result<u32, tree::MethodErr> {
        let notification = DBusNotification::new(
            app_name.to_owned(),
            replaces_id,
            app_icon.to_owned(),
            summary.to_owned(),
            body.to_owned(),
            expire_timeout
        );

        sender.send(notification).unwrap();

        Ok(0 as u32)
    }
}
