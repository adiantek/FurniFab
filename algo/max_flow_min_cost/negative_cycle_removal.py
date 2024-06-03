def read_graph(file):
    
    graph = dict()
    list_of_edges = []
    nodes = [i + 1 for i in range(len(file))]

    for line in file:
        line_split = line.split()
        edges = dict()
        for i in range(1, len(line_split)):
            edge_split = line_split[i].split(':')
            max_capacity_and_cost = edge_split[1].split(',')
            edges[int(edge_split[0])] = [int(max_capacity_and_cost[0]), 0, int(max_capacity_and_cost[1])]
            list_of_edges.append((int((line_split[0])[:-1]), int(edge_split[0])))
        for i in range(1, len(nodes) + 1):
            if i not in edges.keys():
                edges[i] = [0, 0, 0]
        graph[int((line_split[0])[:-1])] = edges

    return graph, list_of_edges, nodes

def bellman_ford(graph, list_of_edges, nodes, number_of_all_nodes, starting_node):
    cost = [float("inf")]
    predecessor = [None]
    negative_cycle = []
    reached_nodes = []

    for _ in range(number_of_all_nodes):
        cost.append(float("inf"))
        predecessor.append(None)
    # cost.append(float("inf"))
    # predecessor.append(None)
    cost[0] = 0
    start_node = min(nodes)
    cost[starting_node] = 0
    reached_nodes.append(starting_node)

    for i in range(len(nodes) - 1):
        for (u, v) in list_of_edges:
            _, _, weight = graph[u][v]
            if cost[u] + weight < cost[v]:
                cost[v] = cost[u] + weight
                predecessor[v] = u
                reached_nodes.append(v)

    if (any(predecessor)):
        for (u, v) in list_of_edges:
            if cost[u] + weight < cost[v]:
    #             negative cycle was found
                predecessor[v] = u
                visited = [False for i in range(number_of_all_nodes + 1)]
                visited[v] = True
                while not(visited[u]):
                    visited[u] = True
                    u = predecessor[u]
                    if u == None:
                        return negative_cycle, reached_nodes
                negative_cycle.append(u)
                v = predecessor[u]
                while v != u:
                    negative_cycle.append(v)
                    v = predecessor[v]
                return negative_cycle, reached_nodes
    return negative_cycle, reached_nodes

def check_all_cohesion_components(graph, list_of_edges, nodes):
    number_of_all_nodes = len(nodes)
    non_checked_nodes = nodes
    negative_cycle = []
    i = 1
    while negative_cycle == [] and i <= len(non_checked_nodes):
        negative_cycle, reached_nodes = bellman_ford(graph, list_of_edges, non_checked_nodes, number_of_all_nodes, i)
        non_checked_nodes = [x for x in non_checked_nodes if x not in reached_nodes]
        i += 1
    return negative_cycle

def find_minimum_edge_augment(residual_graph, negative_cycle):
    # initialize augment as infinity
    augment = float("inf")

    # get sublist with edges only from the cycle
    negative_cycle_edges = [(negative_cycle[i], negative_cycle[i+1]) for i in range(len(negative_cycle) - 1)]
    negative_cycle_edges.append((negative_cycle[-1], negative_cycle[0]))

    # loop through the edges to get minimal augment
    for (start_node, end_node) in negative_cycle_edges:
        # edge parameters: (maximal_capacity, used_capacity, cost)
        maximal_capacity, used_capacity, cost = residual_graph[start_node][end_node]
        edge_augment = maximal_capacity - used_capacity
        if edge_augment < augment:
            augment = edge_augment

    return augment

def create_residual_graph(graph, list_of_edges, nodes):
    # create an empty residual graph and an empty list of it's edges
    residual_graph = dict()
    residual_list_of_edges = []

    # fill residual graph with empty edges
    for current_node in nodes:
        residual_graph[current_node] = dict()

    # add capacity information to the edges
    for current_node in nodes:
        edges_end_original = [y for (x, y) in list_of_edges if x == current_node]
        for end_node in edges_end_original:
            max_capacity, used_capacity, cost = graph[current_node][end_node]
            # add a reverse edge with capacity equal to flow from current_node to end_node
            if used_capacity > 0:
                residual_graph[end_node][current_node] = (used_capacity, 0, -cost)
                residual_list_of_edges.append((end_node, current_node))
            non_used_capacity = max_capacity - used_capacity
            # if capacity is not fully used, add a forward edge
            if non_used_capacity > 0:
                residual_graph[current_node][end_node] = (non_used_capacity, 0, cost)
                residual_list_of_edges.append((current_node, end_node))
    return residual_graph, residual_list_of_edges

def adjust_graph(graph, negative_cycle, list_of_edges, augment):
    # get sublist with edges only from the cycle
    negative_cycle_edges = [(negative_cycle[i], negative_cycle[i + 1]) for i in range(len(negative_cycle) - 1)]
    negative_cycle_edges.append((negative_cycle[-1], negative_cycle[0]))

    for negative_cycle_edge in negative_cycle_edges:
        if negative_cycle_edge in list_of_edges:
            maximum_capacity, used_capacity, cost = graph[negative_cycle_edge[0]][negative_cycle_edge[1]]
            graph[negative_cycle_edge[0]][negative_cycle_edge[1]] = [maximum_capacity, used_capacity + augment, cost]
        else:
            maximum_capacity, used_capacity, cost = graph[negative_cycle_edge[1]][negative_cycle_edge[0]]
            graph[negative_cycle_edge[1]][negative_cycle_edge[0]] = [maximum_capacity, used_capacity - augment, cost]

    return graph

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
            availible_flow_of_adjacent_node = availible_flow[adjacent_node][0] - availible_flow[adjacent_node][1]
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
            availible_flow_to_current_node = availible_flow_of_parent[current_node][0] - availible_flow_of_parent[current_node][1]
            path_flow = min(path_flow, availible_flow_to_current_node)
            current_node = parent
        max_flow += path_flow

        current_node = t
        while current_node != s:
            parent = parents[current_node]
            graph[parent][current_node][1] += path_flow
            graph[current_node][parent][0] += path_flow
            current_node = parent

        parents = find_path(s, t, nodes, list_of_edges, graph)
    return max_flow

def flow_cost(graph, list_of_edges):
    cost = 0
    for (u, v) in list_of_edges:
        _, used_capacity, edge_cost = graph[u][v]
        if used_capacity > 0:
            cost += used_capacity * edge_cost
    return cost

def start(file):
    graph, list_of_edges, nodes = read_graph(file)
    s = min(nodes)
    t = max(nodes)

    max_flow = ford_fulkerson(s, t, nodes, list_of_edges, graph)

    cost = flow_cost(graph, list_of_edges)

    residual_graph, residual_list_of_edges = create_residual_graph(graph, list_of_edges, nodes)
    residual_list_of_edges.sort()

    negative_cycle = check_all_cohesion_components(residual_graph, residual_list_of_edges, nodes)
    negative_cycle.reverse()
    while negative_cycle != []:
        minimum_augment = find_minimum_edge_augment(residual_graph, negative_cycle)

        graph = adjust_graph(graph, negative_cycle, list_of_edges, minimum_augment)

        last_cost = cost

        cost = flow_cost(graph, list_of_edges)

        if last_cost > cost:

            residual_graph, residual_list_of_edges = create_residual_graph(graph, list_of_edges, nodes)
            residual_list_of_edges.sort()

            negative_cycle = check_all_cohesion_components(residual_graph, residual_list_of_edges, nodes)
            negative_cycle.reverse()
        else:
            negative_cycle = []

    return graph, max_flow, cost

if __name__ == '__main__':

    file = open("max_flow_min_cost/file_negative_cycle_x1.txt")
    f = file.readlines()
    file.close()

    graph, maximum_flow, minimum_cost = start(f)