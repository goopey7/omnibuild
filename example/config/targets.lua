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
