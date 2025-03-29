local module = {
	name = "ServerExecutable",
	type = "exe",
	dependencies = { "RuntimeStaticLib", "RuntimeDynamicLib" },
	include_dirs = { "public" },
}

ob.add_module(module)
