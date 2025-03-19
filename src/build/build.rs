use crate::build::build_state::BUILD_STATE;
use crate::cli::Cli;
pub fn configure(lua: &mlua::Lua, args: &Cli) {
    let build_state = BUILD_STATE.try_read().expect("failed to get read on build_state");
    assert!(build_state.project.is_some(), "project must be set!");
    assert!(!build_state.modules.is_empty(),"no modules have been added!");
    assert!(!build_state.targets.is_empty(),"no targets have been added!");
    assert!(!build_state.configs.is_empty(),"no configs have been added!");

    // generate makefile, visual studio solution etc.
}
