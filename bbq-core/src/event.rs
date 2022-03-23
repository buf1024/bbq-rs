
// 业务发给界面
#[derive(Debug, Clone)]
pub enum TraderEvent {
    Test(String)
}

// 界面发给业务
#[derive(Debug, Clone)]
pub enum CtrlEvent {
    Test(String)
}
