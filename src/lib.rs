use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(
            PropertyAuthorizer {
                context_id,
                user: None
            }
        )
    });
}

struct PropertyAuthorizer {
    context_id: u32,
    user: Option<String>,
}
impl Context for PropertyAuthorizer {}

impl HttpContext for PropertyAuthorizer {
    fn on_http_request_body(&mut self, _: usize, _: bool) -> Action {
        match &self.user {
            Some(x) => {
                trace!("User: {} for context_id {}", x, self.context_id);
                Action::Continue
            }
            None => {
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access forbidden.\n"),
                );
                Action::Pause
            }
        }
    }

    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_request_header("user") {
            Some(user) => {
                self.user = Some(user);
            }
            _ => {}
        }
        Action::Continue
    }
}
