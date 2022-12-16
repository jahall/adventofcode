"Handy holder for network values"
struct Network
    flow_rates::Dict{String, Int64}
    neighbours::Dict{String, Vector{String}}
    num_non_zero::Int64

    Network(flow_rates, neighbours) = new(
        flow_rates,
        neighbours,
        length(filter(x -> x > 0, [rate for (_, rate) in flow_rates])),
    )
end

"Utility function to load the network"
function get_network()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day16_test.txt")
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

"Find optimal"
function find_optimal(
    network::Network,
    location::String,
    minute::Int64,
    total_pressure::Int64,
    pressure::Int64,
    turned_on::Set{String},
)
    println(minute)
    if minute == 11
        return total_pressure
    end
    # do nothing
    optimal = total_pressure + pressure * (31 - minute)
    if length(turned_on) == network.num_non_zero
        return optimal
    end
    # turn this on if 
    if !in(location, turned_on) && network.flow_rates[location] > 0
        this_opt = find_optimal(
            network,
            location,
            minute + 1,
            total_pressure + pressure,
            pressure + network.flow_rates[location],
            union(turned_on, Set([location])),
        )
        if this_opt > optimal
            optimal = this_opt
        end
    end
    # move to another node
    for next in network.neighbours[location]
        this_opt = find_optimal(
            network,
            location,
            minute + 1,
            total_pressure + pressure,
            pressure,
            turned_on,
        )
        if this_opt > optimal
            optimal = this_opt
        end
    end
    optimal
end

"Part 1"
function part1()
    network = get_network()
    total_pressure = find_optimal(
        network,
        "AA",
        1,
        0,
        0,
        Set{String}(),
    )
    println("PART 1: $total_pressure")
end

"Part 2"
function part2()
    println("PART 2:")
end

# x hours
part1()
part2()
