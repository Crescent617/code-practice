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

// Definition for Employee.
class Employee {
   public:
    int id;
    int importance;
    vector<int> subordinates;
};

enum IP { V4 };
enum Week { Sun = 7, Mon = 1, Tue, Wed, Thu, Fri, Sat };

void printWeek(Week w) { print(w); }

union _XXG_PortRunFLAG {
    struct _PortRunFLAG {
        ushort Port_SigUdc : 1;
        ushort Port_UdcOK : 1;
        ushort Port_AutoOrManual : 1;  // 1: Auto
        ushort Port_RunOrTest : 1;     // 1: Run

        ushort PWMState_50Hz : 1;    // 1: Disable
        ushort PWMState_Dab : 1;     // 1: Disable
        ushort PWMState_Module : 1;  // 1: Disable
        ushort Stop_AllPort : 1;     // 1: all port need shutdown
        ushort PhaseSquence_Error : 1;
        ushort Port_HighUdc : 1;
        ushort Reservedbit : 6;
    } Bbits;
    ushort Flag;
} xxg;

// Definition for a Node.
class Node {
   public:
    int val;
    vector<Node *> children;

    Node() {}

    Node(int _val) { val = _val; }

    Node(int _val, vector<Node *> _children) {
        val = _val;
        children = _children;
    }
};

class Solution {
   public:
    vector<int> postorder(Node *root) {
        vector<int> res;
        helper(root, res);
        return res;
    }

    void helper(Node *node, vector<int> &res) {
        if (!node) return;
        for (auto child : node->children) {
            helper(child, res);
        }
        res.push_back(node->val);
    }
};

// Not Original author
int main() {
    vector<int> v[10];
    // print(v);
}