use std::collections::HashMap;

use libcommand::{impl_command, CommandClient, TrinityCommand};
use wit_log as log;
use wit_sync_request;

struct Component;

impl Component {
    fn get_quote(msg: &str) -> Option<String> {
        if !msg.starts_with("!horsejs") {
            return None;
        }

        const URL: &str = "https://javascript.horse/random.json";

        let resp = wit_sync_request::Request::get(URL)
            .header("Accept", "application/json")
            .run()
            .ok()?;

        if resp.status != wit_sync_request::ResponseStatus::Success {
            log::info!("request failed with non-success status code");
        }

        #[derive(serde::Deserialize)]
        struct Response {
            text: String,
        }

        serde_json::from_str::<Response>(&resp.body?)
            .ok()
            .map(|resp| resp.text)
    }
}

impl TrinityCommand for Component {
    fn init(_config: HashMap<String, String>) {
        let _ = log::set_boxed_logger(Box::new(crate::log::WitLog::new()));
        log::set_max_level(log::LevelFilter::Trace);
        log::trace!("Called the init() method \\o/");
    }

    fn on_help(_topic: Option<&str>) -> String {
        "Contextless twitter quotes about the JavaScript".to_owned()
    }

    fn on_msg(client: &mut CommandClient, content: &str) {
        if let Some(content) = Self::get_quote(&content) {
            client.respond(content);
        }
    }
}

impl_command!(Component);
