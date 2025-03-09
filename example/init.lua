local project = {
	project_name = "omnibuild_example",
	project_version = "0.0.1"
}
ob.set_project(project)

local debug_config = {
	name = "debug",
	optimization_level = 0,
	warnings_as_errors = true,
}
ob.add_config(debug_config)

local release_config = {
	name = "release",
	optimization_level = 4,
	warnings_as_errors = true,
}
ob.add_config(release_config)

local client_target = {
	name = "client",
	module_directories = {
		"runtime",
		"client",
	}
}
ob.add_target(client_target)

local server_target = {
	name = "server",
	module_directories = {
		"runtime",
		"server",
	}
}
ob.add_target(server_target)

local editor_target = {
	name = "editor",
	module_directories = {
		"runtime",
		"editor",
		"client",
		"server",
	}
}
ob.add_target(editor_target)
