#include <bits/stdc++.h>

using namespace std;

int n, m, lognm;
vector<string> c, s;
vector<vector<int>> used, nxt;

void getnext(int x, int y, int &nx, int &ny) {
    if (s[x][y] == 'U') nx = x - 1, ny = y;
    if (s[x][y] == 'R') nx = x, ny = y + 1;
    if (s[x][y] == 'D') nx = x + 1, ny = y;
    if (s[x][y] == 'L') nx = x, ny = y - 1;
}

void dfs(int x, int y) {
    used[x][y] = 1;
    int nx = -1, ny = -1;
    getnext(x, y, nx, ny);
    assert(0 <= nx && nx < n && 0 <= ny && ny < m);
    int v = x * m + y, to = nx * m + ny;
    if (!used[nx][ny]) dfs(nx, ny);
    nxt[v][0] = to;
}

int main() {
#ifdef _DEBUG
    freopen("input.txt", "r", stdin);
//	freopen("output.txt", "w", stdout);
#endif

    int t;
    cin >> t;
    while (t--) {
        cin >> n >> m;
        lognm = 0;
        int nm = n * m;
        while ((1 << lognm) <= nm) ++lognm;
        c = s = vector<string>(n);
        for (auto &it : c) cin >> it;
        for (auto &it : s) cin >> it;

        used = vector<vector<int>>(n, vector<int>(m));
        nxt = vector<vector<int>>(n * m, vector<int>(lognm));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (!used[i][j]) dfs(i, j);
            }
        }
        for (int deg = 1; deg < lognm; ++deg) {
            for (int i = 0; i < n; ++i) {
                for (int j = 0; j < m; ++j) {
                    int id = i * m + j;
                    nxt[id][deg] = nxt[nxt[id][deg - 1]][deg - 1];
                }
            }
        }

        vector<vector<int>> black, white;
        black = white = vector<vector<int>>(n, vector<int>(m));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                int v = i * m + j, to = v;
                for (int deg = lognm - 1; deg >= 0; --deg) {
                    if ((nm >> deg) & 1) to = nxt[to][deg];
                }
                if (c[i][j] == '0') {
                    ++black[to / m][to % m];
                    cout << "c[i][j]: " << c[i][j] << "\ni: " << to / m
                         << " j: " << to % m << endl;
                } else
                    ++white[to / m][to % m];
            }
        }
        for (auto &it : c) cout << it << endl;
        cout << "c[1][0]: " << c[1][0] << endl;

        int all = 0, good = 0;
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < m; ++j) {
                if (black[i][j]) {
                    ++all;
                    ++good;
                } else if (white[i][j]) {
                    ++all;
                }
            }
        }
        cout << all << " " << good << endl;
    }

    return 0;
}