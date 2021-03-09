class Solution(object):
    def slidingPuzzle(self, board):
        R, C = len(board), len(board[0])
        start = tuple(itertools.chain(*board))
        queue = collections.deque([(start, start.index(0), 0)])
        seen = {start}

        target = tuple(range(1, R*C) + [0])

        while queue:
            board, posn, depth = queue.popleft()
            if board == target: return depth
            for d in (-1, 1, -C, C):
                nei = posn + d
                if abs(nei//C - posn//C) + abs(nei%C - posn%C) != 1:
                    continue
                if 0 <= nei < R*C:
                    newboard = list(board)
                    newboard[posn], newboard[nei] = newboard[nei], newboard[posn]
                    newt = tuple(newboard)
                    if newt not in seen:
                        seen.add(newt)
                        queue.append((newt, nei, depth+1))

        return -1

