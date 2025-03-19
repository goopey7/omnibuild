#include <cstdint>
#include <test.h>
#include <print>

int main()
{
	uint32_t i = test();
	std::println("{}", i);
	return 0;
}
