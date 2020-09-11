import numpy as np

def _sum_log(n):
	if n == 0:
		return 0
	return np.sum(np.log(np.arange(1, n+1)))

sum_log = np.frompyfunc(_sum_log, 1, 1)

