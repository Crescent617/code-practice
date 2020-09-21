#include <bits/stdc++.h>

#include <iostream>
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

#define MAX 1000000009
#define For(i, a, b) for (ll i = (a); i < (b); i++)
#define ll long long
#define x first
#define y second
using namespace std;

template <class F>
struct y_combinator {
    F f;
    template <class... Args>
    auto operator()(Args &&... args) const {
        return f(*this, std::forward<Args>(args)...);
    }
};

template <class F>
inline auto Y(F &&f) {
    return y_combinator<F>{std::forward<F>(f)};
}

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

template <typename T, template <typename _T, typename Allocator = allocator<_T>>
                      class Container>
T sum(Container<T> items) {
    T res = 0;
    for (auto &it : items) {
        res += it;
    }
    return res;
}

template <typename First, typename... Rest>
struct Sum {
    enum { value = Sum<First>::value + Sum<Rest...>::value };
};

template <typename Last>
struct Sum<Last> {
    enum { value = sizeof(Last) };
};

void print() { cout << '\n'; }

//展开函数
template <class T, class... Args>
void print(T head, Args... rest) {
    cout << head << ' ';
    print(rest...);
}

template <class T>
struct PairHash {
    int operator()(pair<T, T> p) {
        return hash<T>(p.first) ^ (hash<T>(p.second << 1));
    }
};

template <class T = int>
struct Pair {
    T first;
    T second;
    bool operator==(const Pair<T> &other) const {
        return other.first == first && other.second == second;
    }
};

namespace std {
template <typename T>
struct hash<Pair<T>> {
    std::size_t operator()(const Pair<T> &p) const {
        auto h = std::hash<T>();
        return h(p.first) ^ (h(p.second) << 1);
    }
};
}  // namespace std

template <typename T, template <class U> typename Container>
class XCls {
   private:
    Container<T> c;
};

// Not Original author
int main() {
    string s = "abcdefg";
    vector<int> vec = {1, 2, 3, 4, 5};
    cout << s.substr(0, 2) << ' ';
    vec.erase(vec.begin() + 1);
    set<int> ss;
    unordered_set<int> sss;
    using pr = pair<const int, int>;
    map<int, int> m;
    using m_it = map<int, int>::iterator;

    for (size_t i = 0; i < 10; i++) {
        m.insert({i, 100 - i});
    }


    // sort(m.begin(), m.end(),
    //      [](m_it &p1, m_it &p2) { return (*p1).second > (*p2).second; });
    for (auto p : m) {
        print(p.first, p.second);
    }
}