import networkx as nx

edges = [
    ("a", "b"),
    ("a", "c"),
    ("b", "d"),
    ("d", "a"),
    ("c", "d"),
    ("c", "e"),
    ("d", "e"),
]
dag = nx.DiGraph(edges)
s = nx.is_directed_acyclic_graph(dag)
print(s)
