#include <iostream>

using namespace std;

// 程序的主函数
int main() {
    int* num;
    int nums[10];
    // *num = 0;
    nums[0] = 0;
    cout << "pnum: " << num << endl;
    cout << "&nums[0]: " << &nums[0] << " nums: " << nums;
}