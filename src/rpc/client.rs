use crate::rpc::message::{Message, Notification, Request, Response};
use crate::rpc::structs::{
    Alert, AvailableLanguages, AvailablePlugins, AvailableThemes, ConfigChanged, FindStatus,
    LanguageChanged, MeasureWidth, PluginStarted, PluginStopped, ReplaceStatus, ScrollTo, Style,
    ThemeChanged, Update, UpdateCmds,
};
use crossbeam_channel::unbounded;
use druid::Data;
use log::*;
use pipe::{pipe, PipeReader, PipeWriter};
use serde_json::{self, from_value, json, to_vec, Value};
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex};
use std::{fmt, thread};
use xi_core_lib::XiCore;
use xi_rpc::RpcLoop;

type XiSender = PipeWriter;
type XiReceiver = PipeReader;

pub trait Callback: Send {
    fn call(self: Box<Self>, result: Result<Value, Value>);
}

impl<F: FnOnce(Result<Value, Value>) + Send> Callback for F {
    fn call(self: Box<Self>, result: Result<Value, Value>) {
        (*self)(result)
    }
}

pub struct Client {
    sender: XiSender,
    pending_requests: Arc<Mutex<HashMap<u64, Box<dyn Callback>>>>,
    current_request_id: Cell<u64>,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(format!("current_request_id: {:?}", self.current_request_id).as_str())
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            pending_requests: self.pending_requests.clone(),
            current_request_id: self.current_request_id.clone(),
        }
    }
}

impl Data for Client {
    fn same(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Default for Client {
    fn default() -> Self {
        let (mut _receiver, sender) = Client::start_xi_thread();
        Client {
            sender,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            current_request_id: Cell::new(0),
        }
    }
}

#[derive(Debug)]
pub enum RpcOperations {
    Update(Update),
    ScrollTo(ScrollTo),
    DefStyle(Style),
    AvailablePlugins(AvailablePlugins),
    UpdateCmds(UpdateCmds),
    PluginStarted(PluginStarted),
    PluginStopped(PluginStopped),
    ConfigChanged(ConfigChanged),
    ThemeChanged(ThemeChanged),
    Alert(Alert),
    AvailableThemes(AvailableThemes),
    FindStatus(FindStatus),
    ReplaceStatus(ReplaceStatus),
    AvailableLanguages(AvailableLanguages),
    LanguageChanged(LanguageChanged),
    MeasureWidth((u64, MeasureWidth)),
}

impl Client {
    pub fn new() -> (Client, crossbeam_channel::Receiver<RpcOperations>) {
        let (mut receiver, sender) = Client::start_xi_thread();
        let client = Client {
            sender,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            current_request_id: Cell::new(0),
        };

        let (rpc_sender, rpc_receiver) = unbounded();
        let pending_requests = client.pending_requests.clone();

        thread::spawn(move || {
            let mut buf = String::new();
            while receiver.read_line(&mut buf).is_ok() {
                let msg = match Message::decode(&buf) {
                    Ok(x) => x,
                    Err(err) => {
                        log::info!("buf: {}", buf);
                        panic!("{:?}", err);
                    }
                };
                log::info!("Received message from xi: {:?}", msg);
                match msg {
                    Message::Request(res) => {
                        let Request { method, params, id } = res;
                        let operation = match method.as_str() {
                            "measure_width" => RpcOperations::MeasureWidth((
                                id,
                                from_value::<MeasureWidth>(params).unwrap(),
                            )),
                            _ => {
                                unreachable!("Unknown method {}", method);
                            }
                        };
                        rpc_sender.send(operation).unwrap();
                    }
                    Message::Response(res) => {
                        let Response { id, result } = res;
                        if let Some(cb) = pending_requests.lock().unwrap().remove(&id) {
                            cb.call(result);
                        }
                    }
                    Message::Notification(res) => {
                        let Notification { method, params } = res;
                        let operation = Client::handle_notification(method, params);
                        if let Err(err) = rpc_sender.send(operation) {
                            log::error!("{}", err);
                        };
                    }
                }

                buf.clear();
            }
        });

        (client, rpc_receiver)
    }

    fn start_xi_thread() -> (XiReceiver, XiSender) {
        let (to_core_rx, to_core_tx) = pipe();
        let (from_core_rx, from_core_tx) = pipe();
        let mut state = XiCore::new();
        let mut rpc_looper = RpcLoop::new(from_core_tx);
        thread::spawn(move || rpc_looper.mainloop(|| to_core_rx, &mut state));
        (from_core_rx, to_core_tx)
    }

    pub fn client_started(
        &mut self,
        config_dir: Option<&String>,
        client_extras_dir: Option<&String>,
    ) {
        self.send_notification(
            "client_started",
            &json!({
                "config_dir": config_dir,
                "client_extras_dir": client_extras_dir,
            }),
        );
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.send_notification(
            "resize",
            &json!({
                    "width": width,
                    "height": height,
            }),
        );
    }

    pub fn modify_user_config_domain(&mut self, domain: &str, changes: &Value) {
        self.send_notification(
            "modify_user_config",
            &json!({
                "domain": domain,
                "changes": changes,
            }),
        )
    }

    pub fn send_notification(&mut self, method: &str, params: &Value) {
        let cmd = json!({
            "method": method,
            "params": params,
        });

        info!("Xi-CORE <-- {}", cmd);
        self.sender.write_all(&to_vec(&cmd).unwrap()).unwrap();
        self.sender.write_all(b"\n").unwrap();
        self.sender.flush().unwrap();
    }

    pub fn new_view<F>(&mut self, file_path: String, callback: F)
    where
        F: FnOnce(Result<Value, Value>) + Send + 'static,
    {
        self.send_request(
            "new_view",
            &json!({
                "file_path": file_path,
            }),
            callback,
        );
    }

    /// Calls the callback with the result (from a different thread).
    fn send_request<F>(&mut self, method: &str, params: &Value, callback: F)
    where
        F: FnOnce(Result<Value, Value>) + Send + 'static,
    {
        let cmd = json!({
            "method": method,
            "params": params,
            "id": self.current_request_id,
        });
        let id = { self.current_request_id.get() };
        info!("Xi-CORE <-- {}", cmd.clone());
        self.sender.write_all(&to_vec(&cmd).unwrap()).unwrap();
        self.sender.write_all(b"\n").unwrap();
        self.sender.flush().unwrap();
        self.pending_requests
            .lock()
            .unwrap()
            .insert(id, Box::new(callback));
        self.current_request_id.set(id + 1);
    }

    pub fn handle_notification(method: String, params: Value) -> RpcOperations {
        match method.as_str() {
            "update" => RpcOperations::Update(from_value::<Update>(params).unwrap()),
            "scroll_to" => RpcOperations::ScrollTo(from_value::<ScrollTo>(params).unwrap()),
            "def_style" => RpcOperations::DefStyle(from_value::<Style>(params).unwrap()),
            "available_plugins" => {
                RpcOperations::AvailablePlugins(from_value::<AvailablePlugins>(params).unwrap())
            }
            "plugin_started" => {
                RpcOperations::PluginStarted(from_value::<PluginStarted>(params).unwrap())
            }
            "plugin_stopped" => {
                RpcOperations::PluginStopped(from_value::<PluginStopped>(params).unwrap())
            }
            "update_cmds" => RpcOperations::UpdateCmds(from_value::<UpdateCmds>(params).unwrap()),
            "config_changed" => {
                RpcOperations::ConfigChanged(from_value::<ConfigChanged>(params).unwrap())
            }
            "theme_changed" => {
                RpcOperations::ThemeChanged(from_value::<ThemeChanged>(params).unwrap())
            }
            "alert" => RpcOperations::Alert(from_value::<Alert>(params).unwrap()),
            "available_themes" => {
                RpcOperations::AvailableThemes(from_value::<AvailableThemes>(params).unwrap())
            }
            "find_status" => RpcOperations::FindStatus(from_value::<FindStatus>(params).unwrap()),
            "replace_status" => {
                RpcOperations::ReplaceStatus(from_value::<ReplaceStatus>(params).unwrap())
            }
            "available_languages" => {
                RpcOperations::AvailableLanguages(from_value::<AvailableLanguages>(params).unwrap())
            }
            "language_changed" => {
                RpcOperations::LanguageChanged(from_value::<LanguageChanged>(params).unwrap())
            }
            _ => {
                unreachable!("Unknown method {}", method)
            }
        }
    }
}
