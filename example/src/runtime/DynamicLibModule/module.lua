local module = {
	name = "RuntimeDynamicLib",
	type = "dylib",
	dependencies = {},
	include_dirs = { "public" },
}

ob.add_module(module)
