#include <better_enums/better_enums.hpp>

#ifndef TEST_VALUES
#define NUM_VALUES 1
#define TEST_VALUES E0
#endif
declenum(TestEnum, int, TEST_VALUES);

static_assert(NUM_VALUES == meta::enum_traits<TestEnum>::count);
int main() {}
