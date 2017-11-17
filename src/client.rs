use xrl;
use xrl::{ClientResult, ClientError};
use serde_json::Value;
use futures::future::{ok, err};

pub trait Client {
    fn notify(&mut self, method: &str, params: Value) -> ClientResult<()>;
    fn request(&mut self, method: &str, params: Value) -> ClientResult<Value>;
    fn scroll(
        &mut self,
        view_id: &str,
        first_line: u64,
        last_line: u64,
    ) -> ClientResult<()>;
    fn left(&mut self, view_id: &str) -> ClientResult<()>;
    fn left_sel(&mut self, view_id: &str) -> ClientResult<()>;
    fn right(&mut self, view_id: &str) -> ClientResult<()>;
    fn right_sel(&mut self, view_id: &str) -> ClientResult<()>;
    fn up(&mut self, view_id: &str) -> ClientResult<()>;
    fn up_sel(&mut self, view_id: &str) -> ClientResult<()>;
    fn down(&mut self, view_id: &str) -> ClientResult<()>;
    fn down_sel(&mut self, view_id: &str) -> ClientResult<()>;
    fn del(&mut self, view_id: &str) -> ClientResult<()>;
    fn page_up(&mut self, view_id: &str) -> ClientResult<()>;
    fn page_up_sel(&mut self, view_id: &str) -> ClientResult<()>;
    fn page_down(&mut self, view_id: &str) -> ClientResult<()>;
    fn page_down_sel(&mut self, view_id: &str) -> ClientResult<()>;
    fn insert_newline(&mut self, view_id: &str) -> ClientResult<()>;
    fn f1(&mut self, view_id: &str) -> ClientResult<()>;
    fn f2(&mut self, view_id: &str) -> ClientResult<()>;
    fn char(&mut self, view_id: &str, ch: char) -> ClientResult<()>;
    fn click(
        &mut self,
        view_id: &str,
        line: u64,
        column: u64,
    ) -> ClientResult<()>;
    fn drag(
        &mut self,
        view_id: &str,
        line: u64,
        column: u64,
    ) -> ClientResult<()>;
    fn new_view(&mut self, file_path: Option<String>) -> ClientResult<String>;
    fn close_view(&mut self, view_id: &str) -> ClientResult<()>;
    fn save(&mut self, view_id: &str, file_path: &str) -> ClientResult<()>;
    fn set_theme(&mut self, theme: &str) -> ClientResult<()>;
    fn start_plugin(&mut self, view_id: &str, name: &str) -> ClientResult<()>;
    fn stop_plugin(&mut self, view_id: &str, name: &str) -> ClientResult<()>;
    fn notify_plugin(
        &mut self,
        view_id: &str,
        plugin: &str,
        method: &str,
        params: Value,
    ) -> ClientResult<()>;
}

impl Client for xrl::Client {
    fn notify(&mut self, method: &str, params: Value) -> ClientResult<()> {
        self.notify(method, params)
    }
    fn request(&mut self, method: &str, params: Value) -> ClientResult<Value> {
        self.request(method, params)
    }
    fn scroll(
        &mut self,
        view_id: &str,
        first_line: u64,
        last_line: u64,
    ) -> ClientResult<()> {
        self.scroll(view_id, first_line, last_line)
    }
    fn left(&mut self, view_id: &str) -> ClientResult<()> {
        self.left(view_id)
    }
    fn left_sel(&mut self, view_id: &str) -> ClientResult<()> {
        self.left_sel(view_id)
    }
    fn right(&mut self, view_id: &str) -> ClientResult<()> {
        self.right(view_id)
    }
    fn right_sel(&mut self, view_id: &str) -> ClientResult<()> {
        self.right_sel(view_id)
    }
    fn up(&mut self, view_id: &str) -> ClientResult<()> {
        self.up(view_id)
    }
    fn up_sel(&mut self, view_id: &str) -> ClientResult<()> {
        self.up_sel(view_id)
    }
    fn down(&mut self, view_id: &str) -> ClientResult<()> {
        self.down(view_id)
    }
    fn down_sel(&mut self, view_id: &str) -> ClientResult<()> {
        self.down_sel(view_id)
    }
    fn del(&mut self, view_id: &str) -> ClientResult<()> {
        self.del(view_id)
    }
    fn page_up(&mut self, view_id: &str) -> ClientResult<()> {
        self.page_up(view_id)
    }
    fn page_up_sel(&mut self, view_id: &str) -> ClientResult<()> {
        self.page_up_sel(view_id)
    }
    fn page_down(&mut self, view_id: &str) -> ClientResult<()> {
        self.page_down(view_id)
    }
    fn page_down_sel(&mut self, view_id: &str) -> ClientResult<()> {
        self.page_down_sel(view_id)
    }
    fn insert_newline(&mut self, view_id: &str) -> ClientResult<()> {
        self.insert_newline(view_id)
    }
    fn f1(&mut self, view_id: &str) -> ClientResult<()> {
        self.f1(view_id)
    }
    fn f2(&mut self, view_id: &str) -> ClientResult<()> {
        self.f2(view_id)
    }
    fn char(&mut self, view_id: &str, ch: char) -> ClientResult<()> {
        self.char(view_id, ch)
    }
    fn click(
        &mut self,
        view_id: &str,
        line: u64,
        column: u64,
    ) -> ClientResult<()> {
        self.click(view_id, line, column)
    }
    fn drag(
        &mut self,
        view_id: &str,
        line: u64,
        column: u64,
    ) -> ClientResult<()> {
        self.drag(view_id, line, column)
    }
    fn new_view(&mut self, file_path: Option<String>) -> ClientResult<String> {
        self.new_view(file_path)
    }
    fn close_view(&mut self, view_id: &str) -> ClientResult<()> {
        self.close_view(view_id)
    }
    fn save(&mut self, view_id: &str, file_path: &str) -> ClientResult<()> {
        self.save(view_id, file_path)
    }
    fn set_theme(&mut self, theme: &str) -> ClientResult<()> {
        self.set_theme(theme)
    }
    fn start_plugin(&mut self, view_id: &str, name: &str) -> ClientResult<()> {
        self.start_plugin(view_id, name)
    }
    fn stop_plugin(&mut self, view_id: &str, name: &str) -> ClientResult<()> {
        self.stop_plugin(view_id, name)
    }
    fn notify_plugin(
        &mut self,
        view_id: &str,
        plugin: &str,
        method: &str,
        params: Value,
    ) -> ClientResult<()> {
        self.notify_plugin(view_id, plugin, method, params)
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
    fn scroll(
        &mut self,
        view_id: &str,
        first_line: u64,
        last_line: u64,
    ) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn left(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn left_sel(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn right(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn right_sel(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn up(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn up_sel(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn down(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn down_sel(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn del(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_up(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_up_sel(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_down(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn page_down_sel(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn insert_newline(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn f1(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn f2(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn char(&mut self, view_id: &str, ch: char) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn click(
        &mut self,
        view_id: &str,
        line: u64,
        column: u64,
    ) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn drag(
        &mut self,
        view_id: &str,
        line: u64,
        column: u64,
    ) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn new_view(&mut self, file_path: Option<String>) -> ClientResult<String> {
        Box::new(ok((String::new())))
    }
    fn close_view(&mut self, view_id: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn save(&mut self, view_id: &str, file_path: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn set_theme(&mut self, theme: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn start_plugin(&mut self, view_id: &str, name: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn stop_plugin(&mut self, view_id: &str, name: &str) -> ClientResult<()> {
        Box::new(ok(()))
    }
    fn notify_plugin(
        &mut self,
        view_id: &str,
        plugin: &str,
        method: &str,
        params: Value,
    ) -> ClientResult<()> {
        Box::new(ok(()))
    }
}