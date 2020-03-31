/***g++ -o libpycall.so -shared -fPIC py_call.cpp*/

#include <iostream>
#include <unordered_map>
using namespace std;
typedef long long ll;

void swap(int &a, int &b) {
    int temp = a;
    a = b;
    b = temp;
}

extern "C" {
    void my_swap(int &a, int &b){
        return swap(a, b);
    }
}