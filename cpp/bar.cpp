#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

struct SumUpOperator {
    inline void operator()(int ele) {
        static long long sum = 0;
        sum += ele;
    }
};

// int main() {
//     vector<int> vInt;
//     const int SIZE_VECTOR = 10000000;
//     for (int i = 0; i < SIZE_VECTOR; ++i) {
//         vInt.push_back(i);
//     }
//     for (int i = 0; i < 100; ++i) {
//         for_each(vInt.begin(), vInt.end(), SumUpOperator());
//     }
//     // cout << SumUpOperator::sum;
//     return 0;
// }

int main() {
    vector<int> vInt;
    const int SIZE_VECTOR = 10000000;
    for (int i = 0; i < SIZE_VECTOR; ++i) {
        vInt.push_back(i);
    }
    for (int i = 0; i < 100; ++i) {
        for_each(vInt.begin(), vInt.end(), [](int ele) {
            static long long sum = 0;
            sum += ele;
        });
    }

    return 0;
}