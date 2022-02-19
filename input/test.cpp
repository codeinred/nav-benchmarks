#include <better_enums/better_enums.hpp>

#ifndef TEST_VALUES
#define NUM_VALUES 1
#define TEST_VALUES E0
#endif
declenum(Test0, int, TEST_VALUES);
declenum(Test1, int, TEST_VALUES);
declenum(Test2, int, TEST_VALUES);
declenum(Test3, int, TEST_VALUES);
declenum(Test4, int, TEST_VALUES);
declenum(Test5, int, TEST_VALUES);
declenum(Test6, int, TEST_VALUES);
declenum(Test7, int, TEST_VALUES);
declenum(Test8, int, TEST_VALUES);
declenum(Test9, int, TEST_VALUES);


static_assert(NUM_VALUES == meta::enum_traits<Test0>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test1>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test2>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test3>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test4>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test5>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test6>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test7>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test8>::count);
static_assert(NUM_VALUES == meta::enum_traits<Test9>::count);

int main() {}
