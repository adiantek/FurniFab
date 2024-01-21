def read_input(file_name):
    file = open(file_name)
    f = file.readlines()
    graph = dict()
    list_of_edges = []
    nodes = [i + 1 for i in range(len(f))]
    for line in f:
        line_split = line.split()
        edges = dict()
        for i in range(1, len(line_split)):
            edge_split = line_split[i].split(':')
            edges[int(edge_split[0])] = (int(edge_split[1]))
            list_of_edges.append((int((line_split[0])[:-1]), int(edge_split[0])))
        for i in range(1, len(nodes) + 1):
            if i not in edges.keys():
                edges[i] = 0
        graph[int((line_split[0])[:-1])] = edges
    file.close()
    return graph, list_of_edges, nodes

def find_path(s, t, nodes, list_of_edges, graph):
    visited_nodes = [1] + [0] * (len(nodes) - 1)
    nodes_to_be_checked = [s]
    parents = [-1] * (len(nodes) + 1)
    temp = 1
    while len(nodes_to_be_checked) > 0:
        current_node = nodes_to_be_checked.pop(0)
        adjacent_nodes = [y for (x, y) in list_of_edges if x == current_node]
        availible_flow = graph[current_node]
        for adjacent_node in adjacent_nodes:
            availible_flow_of_adjacent_node = availible_flow[adjacent_node]
            if not visited_nodes[adjacent_node - 1] and availible_flow_of_adjacent_node > 0:
                nodes_to_be_checked.append(adjacent_node)
                visited_nodes[adjacent_node - 1] = 1
                parents[adjacent_node] = current_node
                if adjacent_node == t:
                    return parents
            temp += 1
    return -1

def ford_fulkerson(s, t, nodes, list_of_edges, graph):
    max_flow = 0
    parents = find_path(s, t, nodes, list_of_edges, graph)
    while parents != -1:
        path_flow = float('inf')
        current_node = t
        while current_node != s:
            parent = parents[current_node]
            availible_flow_of_parent = graph[parent]
            availible_flow_to_current_node = availible_flow_of_parent[current_node]
            path_flow = min(path_flow, availible_flow_to_current_node)
            current_node = parent
        max_flow += path_flow

        current_node = t
        while current_node != s:
            parent = parents[current_node]
            graph[parent][current_node] -= path_flow
            graph[current_node][parent] += path_flow
            current_node = parent

        parents = find_path(s, t, nodes, list_of_edges, graph)
    return max_flow

if __name__ == '__main__':
    graph, list_of_edges, nodes = read_input("file_mag.txt")
    s = min(nodes)
    t = max(nodes)
    print(graph)
    print(list_of_edges)

    max_flow = ford_fulkerson(s, t, nodes, list_of_edges, graph)
    print('Maksymalny przepływ na zadanym grafie: ', max_flow)
