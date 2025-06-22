use git2::{ObjectType, Repository};

use crate::{
    build_state,
    lua::{
        config::module_config::ModuleType, git_utils::fetch_package_files,
        package_info::PackageInfo,
    },
};

use super::config::module_config::ModuleConfig;

pub fn is_windows() -> bool {
    cfg!(windows)
}

pub fn is_unix() -> bool {
    cfg!(unix)
}

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn print(str: String) {
    println!("{}", str);
}

pub fn set_project(project: super::config::project_config::ProjectConfig) {
    build_state!().project = Some(project);
}

pub fn add_module(module: super::config::module_config::ModuleConfig) {
    build_state!().modules.push(module);
}

pub fn add_config(config: super::config::build_config::BuildConfig) {
    build_state!().configs.push(config);
}

pub fn add_target(target: super::config::target_config::TargetConfig) {
    build_state!().targets.push(target);
}

pub fn cmd(cmd: String, args: Vec<String>) {
    std::process::Command::new(cmd)
        .args(&args[0..])
        .status()
        .unwrap();
}

pub fn add_package(package: super::config::package_config::PackageConfig) {
    let target = {
        let build_state = build_state!().clone();
        let targets = build_state.targets;
        targets
            .iter()
            .find(|target| target.name == build_state.args.build_target)
            .expect("target not found")
            .clone()
    };

    println!("{}", &package.binary);
    println!("{}", &target.output_dir.to_str().unwrap());
    std::fs::copy(
        package.binary,
        build_state!()
            .working_directory
            .join(&target.output_dir)
            .join(format!(
                "lib{}.{}",
                package.name,
                if package.r#type == ModuleType::Dylib {
                    "so"
                } else {
                    "a"
                }
            )),
    )
    .unwrap();

    let module = ModuleConfig {
        name: package.name,
        r#type: package.r#type,
        include_dirs: package.include_dirs,
        dependencies: vec![],
        path: None,
    };

    build_state!().modules.push(module);
}

pub fn use_repo(url: String) {
    build_state!().repo_url = url;
}

pub fn use_package(lua: &mlua::Lua, name: String, version: String) {
    let repo_url = { build_state!().repo_url.clone() };
    let (build_lua, info_json) = fetch_package_files(&repo_url, &name);

    let package_info = serde_json::from_str::<PackageInfo>(&info_json).unwrap();

    let repo =
        Repository::clone(&package_info.git, format!(".packages/{}", name)).unwrap_or_else(|_| {
            // If clone fails, try to open existing repo
            let repo = Repository::open(format!(".packages/{}", name)).unwrap();

            // Fetch latest changes from remote
            {
                let mut remote = repo.find_remote("origin").unwrap();
                let refspecs = remote.fetch_refspecs().unwrap();
                let refspecs: Vec<&str> = refspecs.iter().flatten().collect();

                // Perform the fetch
                remote.fetch(&refspecs, None, None).unwrap();
            }
            repo
        });

    // Find the tag reference
    let tag_ref_name = format!("refs/tags/{}", version);
    let tag_ref = repo.find_reference(&tag_ref_name).unwrap();

    // Resolve the tag to get the target commit
    let target_oid = tag_ref.target().unwrap();
    let target_object = repo.find_object(target_oid, None).unwrap();

    // Handle both annotated tags and lightweight tags
    let commit = match target_object.kind() {
        Some(ObjectType::Tag) => {
            // Annotated tag - need to peel to get the commit
            let tag = target_object.as_tag().unwrap();
            let commit_oid = tag.target_id();
            repo.find_commit(commit_oid).unwrap()
        }
        Some(ObjectType::Commit) => {
            // Lightweight tag - directly points to commit
            target_object.as_commit().unwrap().clone()
        }
        _ => panic!(
            "failed to checkout package version {}\n{}",
            version, "Tag does not point to a commit"
        ),
    };

    // Create a detached HEAD checkout
    let tree = commit.tree().unwrap();

    // Checkout the tree
    repo.checkout_tree(tree.as_object(), None).unwrap();

    // Set HEAD to the commit (detached HEAD state)
    repo.set_head_detached(commit.id()).unwrap();

    println!(
        "Successfully checked out tag '{}' at commit {}",
        version,
        commit.id()
    );

    let old_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(old_dir.join(format!(".packages/{}", name))).unwrap();
    lua.load(build_lua).exec().unwrap();
    std::env::set_current_dir(old_dir).unwrap();
}
