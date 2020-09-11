#include <iostream>
#include <memory>
#include <string>

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

void print() { cout << "empty" << endl; }

//展开函数
template <class T, class... Args>
void print(T head, Args... rest) {
    cout << head << ' ';
    print(rest...);
}

// Not Original author
int main() {
    auto tr = make_unique<TreeNode>(10);
    char buffer[200], s[] = "computer", c = 'l';
    int i = 35, j;
    float fp = 1.7320534;
    // 格式化并打印各种数据到buffer
    j = sprintf(buffer, "   String:    %s\n", s);        // C4996
    j += sprintf(buffer + j, "   Character: %c\n", c);   // C4996
    j += sprintf(buffer + j, "   Integer:   %d\n", i);   // C4996
    j += sprintf(buffer + j, "   Real:      %f\n", fp);  // C4996

    printf("Output:\n%s\ncharacter count = %d\n", buffer, j);
    string str = "fdsfas";
    return 0;
}