use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use serde_json;
use serde::Deserialize;
use log::debug;
use crate::graphql_authorizer::GraphqlAuthorizer;

pub mod graphql_authorizer;

#[derive(Deserialize,Debug)]
struct Body {
    query: String,
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);

    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> {
        Box::new(
            PropertyAuthorizer {
                graphql_authorizer: GraphqlAuthorizer {
                    authorized_fields_config: vec![
                        "Kevin:name,age,email".to_string(),
                        "Matt:name".to_string()
                    ]
                },
                user: None
            }
        )
    });
}

struct PropertyAuthorizer {
    graphql_authorizer: GraphqlAuthorizer,
    user: Option<String>
}
impl Context for PropertyAuthorizer {}

impl HttpContext for PropertyAuthorizer {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        match self.get_http_request_header("user") {
            Some(user) => {
                self.user = Some(user);
            }
            _ => {}
        }
        Action::Continue
    }

    fn on_http_request_body(&mut self, _: usize, _: bool) -> Action {
        match &self.user {
            None => {
                debug!("Access denied -- no User header.");
                self.send_http_response(
                    403,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Access denied.\n"),
                );
                Action::Pause
            }
            Some(user) => {
                let body: Vec<u8> = match self.get_http_request_body(0, 1000000) {
                    Some(x) => x,
                    None => Vec::new()
                };

                if body.len() == 0 {
                    return Action::Continue;
                }

                let json: Body = match serde_json::from_slice(&body[..]) {
                    Ok(x) => x,
                    Err(e) => panic!("Couldn't parse request body {}", e),
                };

                let disallowed_fields = self.graphql_authorizer.get_unauthorized_fields(
                    user,
                    &json.query,
                );

                if disallowed_fields.len() > 0 {
                    debug!("User {} denied access to {}", user, disallowed_fields.join(","));
                    self.send_http_response(
                        403,
                        vec![("Powered-By", "proxy-wasm")],
                        Some(b"Access denied.\n"),
                    );
                    return Action::Pause;
                }
                Action::Continue
            }
        }
    }
}
