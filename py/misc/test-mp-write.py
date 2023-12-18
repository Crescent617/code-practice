from pathlib import Path
from concurrent.futures import ProcessPoolExecutor
import pandas as pd

out_dir = Path("output")
out_dir.mkdir(exist_ok=True)

with open(out_dir / "test.txt", "w") as f:
    f.write("test")

df = pd.DataFrame({"a": [1, 2, 3], "b": [4, 5, 6]})

with ProcessPoolExecutor() as executor:

    def write_test(df, i):
        df.to_csv(out_dir / f"test_{i}.csv")

    df.groupby("a").apply(lambda x: executor.submit(write_test, x, x.name))
