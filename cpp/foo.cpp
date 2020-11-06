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

class Solution {
   public:
    int shortestSubarray(vector<int> &A, int K) {
        auto check = [&](int len) -> bool {
            int tmp = 0;
            for (int i = 0; i < A.size(); i++) {
                tmp += A[i];
                if (i >= len) tmp -= A[i - len];
                if (tmp >= K) return true;
            }
            return false;
        };

        int l = 1, r = A.size();
        while (l < r - 1) {
            int m = (r + l) / 2;
            if (check(m))
                r = m;
            else
                l = m;
        }
        cout << check(14);
        return check(r) ? r : -1;
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

// Not Original author
int main() {
    xxg.Flag = 1;
    print(xxg.Bbits.Port_SigUdc);
}