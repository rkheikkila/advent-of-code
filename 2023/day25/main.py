import networkx as nx

import matplotlib.pyplot as plt

edges = []
lines = open("input.txt").readlines()
for line in lines:
    head, tails = line.split(": ")
    for node in tails.split():
        edges.append((head, node))

graph = nx.Graph(edges)
pos = nx.nx_agraph.graphviz_layout(graph)
nx.draw(graph, with_labels=True, pos=pos)
plt.savefig("path.png")

graph.remove_edges_from(nx.minimum_edge_cut(graph))

components = nx.number_connected_components(graph)
print(components)

result = 1
for c in nx.connected_components(graph):
    result *= len(c)

print(result)
