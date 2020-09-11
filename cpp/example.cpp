#include <bits/stdc++.h>
using namespace std;
template <typename T1, typename T2>
inline void chmin(T1& a, T2 b) {
    if (a > b) a = b;
}
template <typename T1, typename T2>
inline void chmax(T1& a, T2 b) {
    if (a < b) a = b;
}
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

// INSERT ABOVE HERE
signed main() {
    cin.tie(0);
    ios::sync_with_stdio(0);

    int n;
    cin >> n;
    vector<int> cs(n);
    for (int i = 0; i < n; i++) cin >> cs[i], cs[i]--;

    vector<vector<int>> G(n);
    for (int i = 1; i < n; i++) {
        int a, b;
        cin >> a >> b;
        a--;
        b--;
        G[a].emplace_back(b);
        G[b].emplace_back(a);
    }

    using ll = long long;
    ll all = 0;
    vector<ll> cnt(n, 0), sub(n, 0);

    vector<ll> ans(n, (ll)n * (n - 1) / 2);
    Y([&](auto dfs, int v, int p) -> int {
        sub[v] = 1;
        ll bfr = cnt[cs[v]];
        for (int u : G[v]) {
            if (u == p) continue;
            ll pre = all - cnt[cs[v]];
            sub[v] += dfs(u, v);
            ll nxt = all - cnt[cs[v]];
            ans[cs[v]] -= (nxt - pre) * (nxt - pre - 1) / 2;
        }
        cnt[cs[v]] = bfr + sub[v];
        all++;
        return sub[v];
    })
    (0, -1);

    for (int c = 0; c < n; c++) {
        ll pre = 0, nxt = n - cnt[c];
        ans[c] -= (nxt - pre) * (nxt - pre - 1) / 2;
    }
    for (int c : cs) ans[c]++;

    for (ll a : ans) cout << a << newl;
    return 0;
}
