local debug_config = {
	name = "debug",
	optimization_level = 0,
	warnings_as_errors = true,
}
ob.add_config(debug_config)

local release_config = debug_config
release_config.name = "release"
release_config.optimization_level = 4
ob.add_config(release_config)
