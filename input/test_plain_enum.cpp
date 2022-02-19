#include <better_enums/better_enums.hpp>

#ifndef TEST_VALUES
#define NUM_VALUES 1
#define TEST_VALUES E0
#endif
enum class Test0 : int { TEST_VALUES, N_Values };
enum class Test1 : int { TEST_VALUES, N_Values };
enum class Test2 : int { TEST_VALUES, N_Values };
enum class Test3 : int { TEST_VALUES, N_Values };
enum class Test4 : int { TEST_VALUES, N_Values };
enum class Test5 : int { TEST_VALUES, N_Values };
enum class Test6 : int { TEST_VALUES, N_Values };
enum class Test7 : int { TEST_VALUES, N_Values };
enum class Test8 : int { TEST_VALUES, N_Values };
enum class Test9 : int { TEST_VALUES, N_Values };


static_assert(NUM_VALUES == int(Test0::N_Values));
static_assert(NUM_VALUES == int(Test1::N_Values));
static_assert(NUM_VALUES == int(Test2::N_Values));
static_assert(NUM_VALUES == int(Test3::N_Values));
static_assert(NUM_VALUES == int(Test4::N_Values));
static_assert(NUM_VALUES == int(Test5::N_Values));
static_assert(NUM_VALUES == int(Test6::N_Values));
static_assert(NUM_VALUES == int(Test7::N_Values));
static_assert(NUM_VALUES == int(Test8::N_Values));
static_assert(NUM_VALUES == int(Test9::N_Values));

int main() {}
