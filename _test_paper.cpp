#include <iostream>

using namespace std;

int main() {
    long long n, k, t;
    char *ans;

    cin >> t;
    while (cin >> n >> k) {
        // cout << "n: " << n << ", k2: " << k * k;
        if ((k * k > n) || (n % 2 != k % 2))
            cout << "NO" << endl;
        else
            cout << "YES" << endl;
    }
}