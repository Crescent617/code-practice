/***g++ -o libpycall.so -shared -fPIC py_call.cpp*/
#include <bits/stdc++.h>
using namespace std;

#define MY_DLL __attribute__((visibility("default")))

#define loop(i, start, end) for (int i = start; i < (end); i++)
#define rev_loop(i, start, end) for (int i = end - 1; i >= start; i--)
#define iter(it, iterable) for (auto &it : iterable)

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

template <typename T>
void chmax(T &a, const T b) {
  if (a < b) a = b;
}

template <typename T>
void chmin(T &a, const T b) {
  if (a > b) a = b;
}

void print() { cout << "empty" << endl; }

//展开函数
template <class T, class... Args>
void print(T head, Args... rest) {
  cout << head << ' ';
  print(rest...);
}

using Int = long long;
using vI = vector<Int>;
using vi = vector<int>;
const char newl = '\n';

extern "C"{
  int add(int a, int b){
    return a + b;
  }
}