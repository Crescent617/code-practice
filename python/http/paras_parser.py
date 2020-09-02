import re

def parse(s):
    res = re.sub(r'(\w+)', r"'\g<1>'", s)
    res = re.sub(r'(?=\n)(?!,)', ',', res)
    return res