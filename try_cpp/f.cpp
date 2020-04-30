#include <bits/stdc++.h>
#define MAX 1000000009
#define For(i, a, b) for (ll i = (a); i < (b); i++)
#define ll long long
#define x first
#define y second
using namespace std;

// Not Original author
int main() {
    ios::sync_with_stdio(0);
    cin.tie(0);
    int n, q;
    cin >> n >> q;

    int a[n + 3], prev = 1;
    a[0] = 1000005;
    a[n + 1] = 1000005;

    set<int> s;
    s.insert(1);
    s.insert(n + 1);

    for (int i = 1; i <= n; i++) {
        int x;
        cin >> x;
        a[i] = x;
        if (x % prev) s.insert(i);
        prev = x;
    }

    while (q--) {
        int ty;
        cin >> ty;
        if (ty == 1) {
            int i, val;
            cin >> i >> val;
            if (val % a[i - 1])
                s.insert(i);
            else if (s.find(i) != s.end())
                s.erase(i);

            if (a[i + 1] % val)
                s.insert(i + 1);
            else if (s.find(i + 1) != s.end())
                s.erase(i + 1);

            a[i] = val;
        } else {
            int i;
            cin >> i;
            auto x = s.upper_bound(i);
            x--;
            cout << *x << '\n';
        }
    }

    return 0;
}