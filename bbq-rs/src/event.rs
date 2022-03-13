
// 业务发给界面
#[derive(Debug, Clone)]
pub enum CoreEvent {
    Test(String)
}

// 界面发给业务
#[derive(Debug, Clone)]
pub enum UiEvent {
    Test(String)
}
