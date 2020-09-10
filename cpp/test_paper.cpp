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
  auto operator()(Args &&... args) const {
    return f(*this, std::forward<Args>(args)...);
  }
};

template <class F>
inline auto Y(F &&f) {
  return y_combinator<F>{std::forward<F>(f)};
}

class GrayCode {
 public:
  vector<string> getGray(int n) {
    // write code here
    if (n == 0) return {""};

    vector<string> result;
    for (auto &ch : {"0", "1"}) {
      for (auto &s : getGray(n - 1)) {
        result.push_back(ch + s);
      }
    }
    reverse(result.begin() + result.size() / 2, result.end());
    return result;
  }
};