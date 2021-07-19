#include <algorithm>
#include <functional>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
   public:
    int numWays(int n, vector<vector<int>> &relation, int k) {
        unordered_map<int, vector<int>> rel;
        for (const auto v : relation) {
            if (rel.find(v[0]) == rel.end()) {
                rel.insert(v[0], {});
            }
            rel[v[0]].push_back(v[1]);
        }

        vector<int> cur(n, 0);
        cur[0] = 1;
        vector<int> nxt(n, 0);

        for (int i = 0; i < k; i++) {
            fill(nxt.begin(), nxt.end(), 0);
            for (int j = 0; j < n; j++) {
                if (rel.find(j) != rel.end()) {
                    for (int nxt_person : rel[j]) {
                        nxt[j] += cur[j];
                    }
                }
            }
            cur = nxt;
        }
        return cur[n - 1];
    }
};

int main() {
    vector<int> m(10, 0);
    for (auto i : m) {
        cout << i << endl;
    }
}