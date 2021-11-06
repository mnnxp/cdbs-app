#[macro_export]
macro_rules! yewLog {
    ( $x:expr ) => {{
        use yew::services::ConsoleService;
        ConsoleService::info(format!("{:?}", $x).as_ref());
    }};
}
