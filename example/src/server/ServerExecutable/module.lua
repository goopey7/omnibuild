local module = {
	name = "ServerExecutable",
	type = "exe",
	dependencies = { "DynamicLibModule", "StaticLibModule" },
	include_dirs = { "public" },
}

ob.add_module(module)
