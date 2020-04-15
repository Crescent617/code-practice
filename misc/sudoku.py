class Sudoku:

    def is_valid_digit(self, d, i, j):
        g = self.grid
        bi, bj = i // 3, j // 3
        return not (d in g[i]
                    or d in {g[r][j] for r in range(9)}
                    or d in {g[bi*3+k][3*bj+n] for k in range(3)
                             for n in range(3)})

    def _solve(self, counter=0):
        if counter > 80:
            return True

        i, j = counter // 9, counter % 9
        if self.grid[i][j] != '.':
            return self._solve(counter+1)

        for num in range(1, 10):
            d = str(num)
            if not self.is_valid_digit(d, i, j):
                continue
            self.grid[i][j] = d
            if self._solve(counter+1):
                return

        self.grid[i][j] = '.'

    def solve(self):
        self._solve(0)
        return self.grid
