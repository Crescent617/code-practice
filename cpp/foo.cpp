#include <bits/stdc++.h>

using namespace std;

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

struct Foo {
    char a;
    int b;
} foo;

class Shape {
   public:
    int id;
};
class Dot : public Shape {};
class Circle : public Dot {};

class Exporter {
   public:
    void exp(const Shape &s) { print("Shape"); };
    void exp(const Dot &d) { print("Dot"); };
    void exp(const Circle &c) { print("Circle"); };
};

class App {
   public:
    void export_(const Shape &s) {
        auto e = make_unique<Exporter>();
        e->exp(s);
    }
};

enum Week { Sun = 7, Mon = 1, Tue, Wed, Thu, Fri, Sat };

void printWeek(Week w) { print(w); }

// Not Original author
int main() {
    // char s1[] = "hhhhhhhhhhhhhhhhhhhhhhhhhhhhh";
    // auto s2 = String(s1);
    // print("size:", sizeof(foo), sizeof(foo.a), sizeof(foo.b));
    auto c = Circle();
    auto app = App();
    app.export_(c);

    printWeek(Week::Tue);
    print(sum(array<int>{1,23,4}));
}