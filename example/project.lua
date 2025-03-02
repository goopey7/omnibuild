project_name = "omnibuild_example"
project_version = "0.0.1"
build_targets = { "client", "server", "editor" }
module_directories = {
	{ path = "runtime", targets = { "client", "server", "editor" } },
	{ path = "editor",  targets = { "editor" } },
}
