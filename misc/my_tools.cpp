/***g++ -o libpycall.so -shared -fPIC py_call.cpp*/
#include <bits/stdc++.h>
using namespace std;

#define loop(i, start, end) for (int i = start; i < (end); i++)
#define rev_loop(i, start, end) for (int i = end - 1; i >= start; i--)
#define iter(it, iterable) for (auto& it : iterable)

template <class F>
struct y_combinator {
    F f;
    template <class... Args>
    decltype(auto) operator()(Args&&... args) const {
        return f(std::ref(*this), std::forward<Args>(args)...);
    }
};

template <class F>
inline decltype(auto) Y(F&& f) {
    return y_combinator<F>{std::forward<F>(f)};
}

template <typename T>
void chmax(T& a, const T b) {
    if (a < b) a = b;
}

template <typename T>
void chmin(T& a, const T b) {
    if (a > b) a = b;
}

struct PyPrint {
    char sep = ' ';

    template <class T>
    auto operator()(T&& t) const {
        cout << t << '\n';
    }

    template <class T, class... Args>
    decltype(auto) operator()(T&& t, Args&&... rest) const {
        cout << t << sep;
        return this->operator()(rest...);
    }

} print;

using Int = long long;
using vI = vector<Int>;
using vi = vector<int>;
const char newl = '\n';
int main() {}