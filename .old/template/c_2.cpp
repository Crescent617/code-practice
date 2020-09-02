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

struct Print {
    char sep = ' ';
    char end = '\n';

    template <class... Args>
    decltype(auto) operator()(const Args&... args) const {
        this->_print(args...);
    }
    template <class T>
    auto _print(const T& t) const {
        cout << t << end;
    }
    template <class T, class... Args>
    decltype(auto) _print(const T& t, const Args&... rest) const {
        cout << t << sep;
        return this->_print(rest...);
    }

    template <class... Args>
    decltype(auto) operator[](const Args&... args) const {
        this->_print_it(args...);
    }
    template <class T>
    auto _print_it(const T& t) const {
        for (auto x : t) cout << x << sep;
        cout << end;
    }
} print;

using Int = long long;
using vI = vector<Int>;
using vi = vector<int>;
const char newl = '\n';

int main() {
    vi v(10, 1);
    print[v];

    return 0;
}