
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
  Quit,
  Resume,
  Suspend,
  RenderTick,
  Resize(u16, u16),
  ToggleShowLogger,
  EnterNormal,
  EnterSearch,
  EnterActionMenu,
  EnterProcessing,
  CancelTask,
  ToggleHelp,
  SetLogs { unit_name: String, logs: String },
  StartService(String),
  ScrollUp(u16),
  ScrollDown(u16),
  ScrollToTop,
  ScrollToBottom,
  Update,
  Noop,
}
