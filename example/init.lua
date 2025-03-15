ob.print("hello from lua")
require("config.project")
require("config.build_configs")
require("config.targets")

ob.build("src")
