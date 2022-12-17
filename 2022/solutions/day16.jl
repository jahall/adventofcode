using DataStructures

"Handy holder for network values"
struct Network
    flow_rates::Dict{String, Int64}
    neighbours::Dict{String, Vector{String}}
    useful_valves::Set{String}

    Network(flow_rates, neighbours) = new(
        flow_rates,
        neighbours,
        Set([name for (name, rate) in flow_rates if rate > 0]),
    )
end

"Current state of the network"
struct State
    minute::Int64
    pressure::Int64
    incremental::Int64
    history::String
    turned_on::Set{String}
    my_location::String
    elephant_location::String

    State() = new(0, 0, 0, "", Set{String}(), "AA", "AA")
    # assume the elephant stays put
    State(minute, pressure, incremental, history, turned_on, my_loc) = new(
        minute, pressure, incremental, history, turned_on, my_loc, "AA"
    )
    State(minute, pressure, incremental, history, turned_on, my_loc, elephant_loc) = new(
        minute, pressure, incremental, history, turned_on, my_loc, elephant_loc
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

"Part 1"
function part1()
    network = get_network()

    pq = PriorityQueue(Base.Order.Reverse)
    enqueue!(pq, State(), 0)
    best = 0
    while length(pq) > 0
        state = dequeue!(pq)
        # we've arrived!
        if state.minute == 31
            if state.pressure > best
                best = state.pressure
            end
            continue
        end
        # common next params
        next_minute = state.minute + 1
        next_pressure = state.pressure + state.incremental
        history = state.history * "/" * state.my_location
        # nothing more we can do now that everything is on...
        if length(state.turned_on) == length(network.useful_valves)
            end_pressure = state.pressure + state.incremental * (31 - state.minute)
            if end_pressure > best
                best = state.pressure
            end
            continue
        end
        # try turning this one on, if it is useful
        if !in(state.my_location, state.turned_on) && in(state.my_location, network.useful_valves)
            next = State(
                next_minute,
                next_pressure + network.flow_rates[state.my_location],
                state.incremental + network.flow_rates[state.my_location],
                history,
                union(state.turned_on, Set([state.my_location])),
                state.my_location,
            )
            enqueue!(pq, next, next.pressure)
        end
        # try moving to a neighbour
        for nbr in network.neighbours[state.my_location]
            next = State(
                next_minute,
                next_pressure,
                state.incremental,
                history,
                state.turned_on,
                nbr,
            )
            enqueue!(pq, next, next.pressure)
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
part2()
