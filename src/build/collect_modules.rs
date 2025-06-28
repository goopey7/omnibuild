use crate::build_state;

pub fn collect_modules(lua: &mlua::Lua) -> Result<(), mlua::Error>
{
    let build_state = build_state!().clone();
    let target = build_state.targets.iter().find(|target| target.name == build_state.args.build_target);
    let target = target.expect("target not found");

    target.module_directories.iter().for_each(|module_dir| {
        let entries = std::fs::read_dir(module_dir).expect("unable to read module directory");
        for entry in entries {
            if let Ok(entry) = entry {
                let module_file_read = std::fs::read_to_string(format!("{}/module.lua", entry.path().to_str().expect("path is invalid unicode"))).expect("no module.lua found!");
                lua.load(module_file_read).exec().expect("failed to execute module.lua");
                build_state!().modules.last_mut().expect(format!("module was not added! Missing call to ob.add_module() in {}", entry.path().to_str().unwrap_or("")).as_str()).path = Some(entry.path());
            }
        }
    });

    Ok(())
}
