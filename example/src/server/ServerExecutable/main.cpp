#include <cstdint>
#include <test.h>
#include <runtime_sl.h>
#include <print>
#include <fmt/printf.h>

int main()
{
	uint32_t i = test();
	std::println("{}", i);

	RuntimeStaticLib();
	OtherRuntimeStaticLib();

	fmt::println("hello from fmt!");

	return 0;
}
