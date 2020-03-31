class Sudoku:

    _valid_clause = frozenset('123456789')

    @staticmethod
    def _valid(nums):
        return set(nums) == Sudoku._valid_clause

    def __init__(self, grid):
        self.grid = grid

    def valid(self):
        grid = self.grid
        _valid = Sudoku._valid

        for g in (grid, zip(*grid)):
            for l in g:
                if not _valid(set(l)):
                    return False
        for bi in range(3):
            for bj in range(3):
                block = {grid[bi*3+k][3*bj+n] for k in range(3)
                         for n in range(3)}
                if not _valid(block):
                    return False
        return True

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
            ans = self._solve(counter+1)
            if ans:
                return ans

        self.grid[i][j] = '.'


    def solve(self):
        self._solve(0)
        return self.grid
