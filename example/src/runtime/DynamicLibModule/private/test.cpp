#include "test.h"
#include <cstdint>
#include <print>

extern "C" __attribute__((visibility("default"))) uint32_t test()
{
	std::println("aw yeah");
	return 42;
}

