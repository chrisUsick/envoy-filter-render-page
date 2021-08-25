use proxy_wasm::types::Action;
use proxy_wasm::traits::HttpContext;
use proxy_wasm::traits::Context;
use proxy_wasm::traits::RootContext;
use proxy_wasm::types::ContextType;
use handlebars::Handlebars;
#[macro_use]
extern crate serde_json;
use serde_json::Value;
use log::info;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(proxy_wasm::types::LogLevel::Trace);
    proxy_wasm::set_root_context(|_context_id| -> Box<dyn RootContext> {
        Box::new(RenderPageRootContext{
            config: RenderPageConfig {
                template: "".to_string()
            }
        })
    });
}

struct RenderPageRootContext {
    config: RenderPageConfig
}

struct RenderPageHttpFilter {
    context_id: u32,
    config: RenderPageConfig
}

struct RenderPageConfig {
    template: String
}

impl Context for RenderPageRootContext {}

impl RootContext for RenderPageRootContext {
    fn on_vm_start(&mut self, _vm_configuration_size: usize) -> bool {
        info!("VM STARTED");
        true
    }
    fn on_configure(&mut self, _: usize) -> bool {
        info!("Reading config");
        // let config = self.get_configuration().unwrap();
        // info!("Found config: #{}", String::from_utf8(config).unwrap());
        if let Some(config_bytes) = self.get_configuration() {
            info!("Found config");
            let cfg: Value = serde_json::from_slice(config_bytes.as_slice()).unwrap();
            // TODO add some proper error handling here
            self.config.template = cfg.get("template").unwrap().as_str().unwrap().to_string();
        } else {
            info!("No config found");
        }
        true
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(RenderPageHttpFilter{ 
            context_id: context_id,
            config: RenderPageConfig {
                template: self.config.template.clone()
            }
        }))
    
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }
}

impl Context for RenderPageHttpFilter {}

impl HttpContext for RenderPageHttpFilter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        info!("In WASM : #{}", self.context_id);
        
        let reg = Handlebars::new();
        match reg.render_template(self.config.template.as_str(), &json!({"name": "foo"})) {
            Ok(body) => {
                info!("template rendered: {}", body);
                self.send_http_response(
                    200,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(body.as_bytes()),
        );
                Action::Pause
            }
            Err(_) => {
                self.send_http_response(
                    500,
                    vec![("Powered-By", "proxy-wasm")],
                    Some(b"Internal error\n")
                );
                Action::Pause
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
