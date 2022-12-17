using Combinatorics
using DataStructures

"Find shortest path - Djikastra"
function find_shortest_path(neighbours, start, target)
    pq = PriorityQueue()
    visited = Set([start])
    enqueue!(pq, (start, 0), 0)
    while length(pq) > 0
        (valve, path_length) = dequeue!(pq)
        if valve == target
            return path_length
        end
        for neighbour in neighbours[valve]
            if !in(neighbour, visited)
                push!(visited, neighbour)
                enqueue!(pq, (neighbour, path_length + 1), path_length + 1)
            end
        end
    end
    shortest
end

"Handy holder for network values"
struct Network
    flow_rates::Dict{String, Int64}
    neighbours::Dict{String, Vector{String}}
    useful_valves::Vector{String}
    shortest_paths::Dict{Tuple{String, String}, Int64}

    function Network(flow_rates, neighbours)
        useful_valves = sort([name for (name, rate) in flow_rates if rate > 0])
        shortest_paths = Dict([
            (v1, v2) => find_shortest_path(neighbours, v1, v2)
            for (v1, v2) in combinations(["AA", useful_valves...], 2)
        ])
        new(flow_rates, neighbours, useful_valves, shortest_paths)
    end
end

"Get the shortest path between two valves"
function shortest_path(network::Network, v1::String, v2::String)
    paths = network.shortest_paths
    haskey(paths, (v1, v2)) ? paths[(v1, v2)] : paths[(v2, v1)]
end

"Calc total pressure for a given order of visiting valves"
function calc_total_pressure(network::Network, visting_order::Vector{String}, time::Int64)
    prev_valve = "AA"
    pressure = 0
    elapsed = 0
    for valve in visting_order
        elapsed += shortest_path(network, prev_valve, valve) + 1
        remaining = time - elapsed
        if remaining <= 0
            break
        end
        pressure += network.flow_rates[valve] * remaining
        prev_valve = valve
    end
    pressure
end

"Utility function to load the network"
function get_network()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day16.txt")
    flow_rates::Dict{String, Int64} = Dict()
    neighbours::Dict{String, Vector{String}} = Dict()
    open(path) do file
        for line in readlines(file)
            parts = split(line, " ")
            name = parts[2]
            rate = parse(Int64, view(parts[5], 6:length(parts[5]) - 1))
            nbrs = [view(part, 1:2) for part in view(parts, 10:length(parts))]
            flow_rates[name] = rate
            neighbours[name] = nbrs
        end
    end
    Network(flow_rates, neighbours)
end

"Part 1"
function part1()
    network = get_network()
    best = 0
    for combo in permutations(network.useful_valves)
        pressure = calc_total_pressure(network, combo, 30)
        if pressure > best
            best = pressure
        end
    end
    println("PART 1: $best")
end

"Part 2"
function part2()
    network = get_network()
    println("PART 2:")
end

# 3 hours
part1()
#part2()
