local common_config = {
  cpp_standard = 23,
  warnings_as_errors = true,
  warnings = {
    "All",
    "Extra",
    "Pedantic",
    "Conversion",
    "Shadow",
    "OldStyleCast",
    "FloatEqual",
    "ExtraSemi",
    "NonVirtualDtor",
    "OverloadedVirtual",
    "StrictNullSentinel",
    "ZeroAsNullPointerConstant",
  },
}

local debug_config = common_config
ob.print("cpp standard: " .. debug_config.cpp_standard)
debug_config.name = "debug"
debug_config.debug_info = true
debug_config.optimization = "None"
ob.add_config(debug_config)

local release_config = common_config
release_config.name = "release"
release_config.debug_info = false
release_config.optimization = "Speed"
ob.add_config(release_config)
