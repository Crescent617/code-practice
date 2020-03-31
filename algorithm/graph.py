# directed graph
# def isCyclic(n, graph):
#     # Code here
#     visited_num = 0
#     indegree = {i: 0 for i in range(n)}
#     for vert in graph:
#         for dest in graph[vert]:
#             indegree[dest] += 1
#     stack = []
#     for vert, i in indegree.items():
#         if not i:
#             stack.append(vert)
#     while stack:
#         vert = stack.pop()
#         visited_num += 1
#         for dest in graph[vert]:
#             indegree[dest] -= 1
#             if not indegree[dest]:
#                 stack.append(dest)

#     return visited_num == n

# def isCyclic(n, graph):
#     # Code here
#     status = {v: 'unvisited' for v in range(n)}
#     for v in range(n):
#         if status[v] == 'unvisited':
#             if dfs(v, graph, status):
#                 return True
#     return False

# def dfs(vert, graph, status):
#     status[vert] = 'visiting'

#     for dest in graph[vert]:
#         if status[dest] == 'visiting':
#             return True
#         if status[dest] == 'unvisited' and dfs(dest, graph, status):
#             return True

#     status[vert] = 'visited'
#     return False

# # undirected graph
# from collections import defaultdict

# def isCyclic(g,n):
#     '''
#     :param g: given adjacency list representation of graph
#     :param n: no of nodes in graph
#     :return:  boolean (whether a cycle exits or not)
#     '''
#     status = {v: 'unvisited' for v in range(n)}
#     for v in range(n):
#         if status[v] == 'unvisited':
#             if dfs(v, g, status):
#                 return 1
#     return 0

# def dfs(vert, graph, status, parent=None):
#     status[vert] = 'visited'

#     for dest in graph[vert]:
#         if status[dest] == 'unvisited':
#             if dfs(dest, graph, status, vert):
#                 return True
#         elif dest != parent:
#             return True
#     return False

# def isCyclic1(g,n):
#     '''
#     :param g: given adjacency list representation of graph
#     :param n: no of nodes in graph
#     :return:  boolean (whether a cycle exits or not)
#     '''
#     # Mark all the vertices as not visited 
#     visited =[False]*n
#     # Call the recursive helper function to detect cycle in different 
#     #DFS trees 
#     for i in range(n): 
#         if visited[i] ==False: #Don't recur for u if it is already visited 
#             if _cyclic(i,visited, g, -1)== True: 
#                 return True
      
#     return False
    
    
# def _cyclic(v,visited, graph, parent): 
#     #Mark the current node as visited  
#     visited[v]= True
#     #Recur for all the vertices adjacent to this vertex 
#     for i in graph[v]: 
#         # If the node is not visited then recurse on it 
#         if  visited[i]==False :  
#             if(_cyclic(i,visited,graph, v)): 
#                 return True
#         # If an adjacent vertex is visited and not parent of current vertex, 
#         # then there is a cycle 
#         elif  parent!=i: 
#             return True
      
#     return False
       
