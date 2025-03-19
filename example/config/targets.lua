local client_target = {
	name = "client",
	module_directories = {
		"src/runtime",
		"src/client",
	}
}
ob.add_target(client_target)

local server_target = {
	name = "server",
	module_directories = {
		"src/runtime",
		"src/server",
	},
	definitions = {
	  IS_SERVER = 1,
	},
}
ob.add_target(server_target)

local editor_target = {
	name = "editor",
	module_directories = {
		"src/runtime",
		"src/editor",
		"src/client",
		"src/server",
	}
}
ob.add_target(editor_target)
