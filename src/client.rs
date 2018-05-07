use xrl;
use xrl::{ClientResult, ViewId};
use serde_json::Value;
use futures::future::ok;

pub trait Client {
    fn notify(&mut self, method: &str, params: Value) -> ClientResult<()>;
    fn request(&mut self, method: &str, params: Value) -> ClientResult<Value>;
    fn edit(&mut self, view_id: ViewId, method: &str, params: Option<Value>) -> ClientResult<()>;
    fn scroll(&mut self, view_id: ViewId, first_line: u64, last_line: u64) -> ClientResult<()>;
    fn left(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn left_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn right(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn right_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn up(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn up_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn down(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn down_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn delete(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn backspace(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn del(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn page_up(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn page_up_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn page_down(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn page_down_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn line_start(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn line_start_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn line_end(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn line_end_sel(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn insert_newline(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn f1(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn f2(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn char(&mut self, view_id: ViewId, ch: char) -> ClientResult<()>;
    fn click(&mut self, view_id: ViewId, line: u64, column: u64) -> ClientResult<()>;
    fn drag(&mut self, view_id: ViewId, line: u64, column: u64) -> ClientResult<()>;
    fn new_view(&mut self, file_path: Option<String>) -> ClientResult<ViewId>;
    fn close_view(&mut self, view_id: ViewId) -> ClientResult<()>;
    fn save(&mut self, view_id: ViewId, file_path: &str) -> ClientResult<()>;
    fn set_theme(&mut self, theme: &str) -> ClientResult<()>;
    fn start_plugin(&mut self, view_id: ViewId, name: &str) -> ClientResult<()>;
    fn stop_plugin(&mut self, view_id: ViewId, name: &str) -> ClientResult<()>;
    fn notify_plugin(
        &mut self,
        view_id: ViewId,
        plugin: &str,
        method: &str,
        params: Value,
    ) -> ClientResult<()>;
}

#[derive(Clone)]
pub struct XrlClient {
    client: xrl::Client,
}

impl XrlClient {
    pub fn new(client: xrl::Client) -> Self {
        XrlClient { client: client }
    }
}

impl Client for XrlClient {
    fn notify(&mut self, method: &str, params: Value) -> ClientResult<()> {
        self.client.notify(method, params)
    }
    fn request(&mut self, method: &str, params: Value) -> ClientResult<Value> {
        self.client.request(method, params)
    }
    fn edit(&mut self, view_id: ViewId, method: &str, params: Option<Value>) -> ClientResult<()> {
        self.client.edit(view_id, method, params)
    }
    fn scroll(&mut self, view_id: ViewId, first_line: u64, last_line: u64) -> ClientResult<()> {
        self.client.scroll(view_id, first_line, last_line)
    }
    fn left(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.left(view_id)
    }
    fn left_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.left_sel(view_id)
    }
    fn right(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.right(view_id)
    }
    fn right_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.right_sel(view_id)
    }
    fn up(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.up(view_id)
    }
    fn up_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.up_sel(view_id)
    }
    fn down(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.down(view_id)
    }
    fn down_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.down_sel(view_id)
    }
    fn delete(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.delete(view_id)
    }
    fn backspace(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.backspace(view_id)
    }
    fn del(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.del(view_id)
    }
    fn page_up(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.page_up(view_id)
    }
    fn page_up_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.page_up_sel(view_id)
    }
    fn page_down(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.page_down(view_id)
    }
    fn page_down_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.page_down_sel(view_id)
    }
    fn line_start(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.line_start(view_id)
    }
    fn line_start_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.line_start_sel(view_id)
    }
    fn line_end(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.line_end(view_id)
    }
    fn line_end_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.line_end_sel(view_id)
    }
    fn insert_newline(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.insert_newline(view_id)
    }
    fn f1(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.f1(view_id)
    }
    fn f2(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.f2(view_id)
    }
    fn char(&mut self, view_id: ViewId, ch: char) -> ClientResult<()> {
        self.client.char(view_id, ch)
    }
    fn click(&mut self, view_id: ViewId, line: u64, column: u64) -> ClientResult<()> {
        self.client.click(view_id, line, column)
    }
    fn drag(&mut self, view_id: ViewId, line: u64, column: u64) -> ClientResult<()> {
        self.client.drag(view_id, line, column)
    }
    fn new_view(&mut self, file_path: Option<String>) -> ClientResult<ViewId> {
        self.client.new_view(file_path)
    }
    fn close_view(&mut self, view_id: ViewId) -> ClientResult<()> {
        self.client.close_view(view_id)
    }
    fn save(&mut self, view_id: ViewId, file_path: &str) -> ClientResult<()> {
        self.client.save(view_id, file_path)
    }
    fn set_theme(&mut self, theme: &str) -> ClientResult<()> {
        self.client.set_theme(theme)
    }
    fn start_plugin(&mut self, view_id: ViewId, name: &str) -> ClientResult<()> {
        self.client.start_plugin(view_id, name)
    }
    fn stop_plugin(&mut self, view_id: ViewId, name: &str) -> ClientResult<()> {
        self.client.stop_plugin(view_id, name)
    }
    fn notify_plugin(
        &mut self,
        view_id: ViewId,
        plugin: &str,
        method: &str,
        params: Value,
    ) -> ClientResult<()> {
        self.client.notify_plugin(view_id, plugin, method, &params)
    }
}

#[derive(Clone)]
pub struct DummyClient;

impl DummyClient {
    pub fn new() -> Self {
        DummyClient {}
    }
}

impl Client for DummyClient {
    fn notify(&mut self, method: &str, params: Value) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn request(&mut self, method: &str, params: Value) -> ClientResult<Value> {
        Box::new(ok(json!({})))
    }
    fn edit(&mut self, view_id: ViewId, method: &str, params: Option<Value>) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn scroll(&mut self, view_id: ViewId, first_line: u64, last_line: u64) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn left(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn left_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn right(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn right_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn up(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn up_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn down(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn down_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn delete(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn backspace(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn del(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_up(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_up_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_down(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_down_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn line_start(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn line_start_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn line_end(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn line_end_sel(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn insert_newline(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn f1(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn f2(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn char(&mut self, view_id: ViewId, ch: char) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn click(&mut self, view_id: ViewId, line: u64, column: u64) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn drag(&mut self, view_id: ViewId, line: u64, column: u64) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn new_view(&mut self, file_path: Option<String>) -> ClientResult<ViewId> {
        Box::new(ok(ViewId(0)))
    }
    fn close_view(&mut self, view_id: ViewId) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn save(&mut self, view_id: ViewId, file_path: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn set_theme(&mut self, theme: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn start_plugin(&mut self, view_id: ViewId, name: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn stop_plugin(&mut self, view_id: ViewId, name: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn notify_plugin(
        &mut self,
        view_id: ViewId,
        plugin: &str,
        method: &str,
        params: Value,
    ) -> ClientResult<()> {
        Box::new(ok(()))
    }
}
