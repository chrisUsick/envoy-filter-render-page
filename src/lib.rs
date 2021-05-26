use proxy_wasm::types::Action;
use proxy_wasm::traits::HttpContext;
use proxy_wasm::traits::Context;
use log::trace;

struct RenderPageFilter {
    context_id: u32,
    template: String
}

impl Context for RenderPageFilter {}

impl HttpContext for RenderPageFilter {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        trace!("In WASM : #{}", self.context_id);
        self.send_http_response(
            403,
            vec![("Powered-By", "proxy-wasm")],
            Some(b"Access forbidden.\n"),
        );
        Action::Pause
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
