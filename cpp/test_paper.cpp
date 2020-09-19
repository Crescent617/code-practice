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
auto Y(F &&f) {
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
namespace std {
template <>
class hash<std::pair<int, int>> {
   public:
    std::size_t operator()(const std::pair<int, int> &p) const {
        return std::hash<int>()(p.first) ^ std::hash<int>()(p.second);
    }
};
}  // namespace std

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};

class Solution {
   public:
    TreeNode *sortedArrayToBST(vector<int> &nums) {
        if (!nums.size()) return nullptr;

        function<TreeNode *(int, int)> dfs = [&](int i, int j) {
            int m = i + (j - i) / 2;
            auto root = new TreeNode(nums[m]);
            if (m > i) root->left = dfs(i, m);
            if (j > m) root->right = dfs(m, j);
            return root;
        };
        return dfs(0, nums.size());
    }
};

int main() {
    unordered_map<pair<int, int>, int> us;
    us[{1, 1}] = 1;
    return 0;
};