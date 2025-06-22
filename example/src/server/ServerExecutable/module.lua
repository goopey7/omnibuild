local module = {
	name = "ServerExecutable",
	type = "exe",
	dependencies = { "RuntimeStaticLib", "RuntimeDynamicLib", "fmt" },
	include_dirs = { "public" },
}

ob.add_module(module)
