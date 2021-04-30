pub mod print_command {
    use crate::model::file_tree::FileEntry;
    use crate::rpc::client::RpcOperations;
    use crate::AvailableThemes;
    use druid::Selector;

    pub const REBUILD_MENUS: Selector = Selector::new("print.rebuild-menus");
    pub const OPEN: Selector = Selector::new("print.open-project");
    pub const SET_FILE: Selector<FileEntry> = Selector::new("print.open-file");
    // todo: add reload dir
    pub const RELOAD_DIR: Selector = Selector::new("print.reload-dir");
}
