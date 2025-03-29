#include <cstdint>
#include <test.h>
#include <runtime_sl.h>
#include <print>

int main()
{
	uint32_t i = test();
	std::println("{}", i);

	RuntimeStaticLib();
	OtherRuntimeStaticLib();

	return 0;
}
