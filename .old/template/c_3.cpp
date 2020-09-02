#include <bits/stdc++.h>
using namespace std;

#define for_i(start, end) for (int i = start; i < (end); i++)
#define for_j(start, end) for (int j = start; j < (end); j++)
#define for_k(start, end) for (int k = start; k < (end); k++)

using Int = long long;
const char newl = '\n';

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

int main() {
    cin.tie(NULL);
    ios::sync_with_stdio(false);
}
