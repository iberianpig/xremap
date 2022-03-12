use crate::client::Client;
use zbus::Connection;

pub struct GnomeClient {
    connection: Option<Connection>,
}

impl GnomeClient {
    pub fn new() -> GnomeClient {
        GnomeClient { connection: None }
    }

    fn connect(&mut self) {
        match Connection::new_session() {
            Ok(connection) => self.connection = Some(connection),
            Err(e) => println!("GnomeClient#connect() failed: {}", e),
        }
    }
}

impl Client for GnomeClient {
    fn supported(&mut self) -> bool {
        self.connect();
        self.connection.is_some()
    }

    fn current_application(&mut self) -> Option<String> {
        use serde::{Deserialize, Serialize};
        #[derive(Serialize, Deserialize)]
        struct ActiveWindow {
            #[serde(default)]
            wm_class: String,
            #[serde(default)]
            title: String,
            #[serde(default)]
            focus: bool,
        }

        self.connect();
        let connection = match &mut self.connection {
            Some(connection) => connection,
            None => return None,
        };

        if let Ok(message) = connection.call_method(
            Some("org.gnome.Shell"),
            "/dev/iberianpig/Appmatcher",
            Some("dev.iberianpig.Appmatcher"),
            "ActiveWindow",
            &(),
        ) {
            if let Ok(json) = message.body::<String>() {
                let w: ActiveWindow = serde_json::from_str(&json).unwrap();
                return Some(w.wm_class);
            }
        }
        None
    }
}
