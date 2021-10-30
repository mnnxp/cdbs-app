use yew::services::ConsoleService;

#[macro_export]
macro_rules! yewLog {
    ( $x:expr ) => {{
        ConsoleService::info(format!("{}", $x).as_ref());
    }};
}
