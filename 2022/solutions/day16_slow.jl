using Combinatorics

"Handy holder for network values"
struct Network
    flow_rates::Dict{String, Int64}
    neighbours::Dict{String, Vector{String}}
    valves::Vector{String}
    useful::Vector{String}

    Network(flow_rates, neighbours) = new(
        flow_rates,
        neighbours,
        sort([name for (name, _) in flow_rates]),
        sort([name for (name, rate) in flow_rates if rate > 0]),
    )
end

"Current state of the network"
struct State
    turned_on::Any
    my_location::String
    elephant_location::String

    # assume the elephant stays put
    State(turned_on, my_loc) = new(turned_on, my_loc, "AA")
    State(turned_on, my_loc, elephant_loc) = new(turned_on, my_loc, elephant_loc)
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

"Initialize the value function for me only"
function init_values_me_only(network::Network)
    values = Dict{State, Int64}(
        State(Tuple(subset), valve) => 0
        for valve in network.valves
        for subset in combinations(network.useful)
    )
    for valve in network.valves
        values[State((), valve)] = 0
    end
    values
end

"Initialize the value function including the elephant"
function init_values_with_elephant(network::Network)
    values = Dict{State, Int64}(
        State(Tuple(subset), my_valve, elephant_valve) => 0
        for my_valve in network.valves
        for elephant_valve in network.valves
        for subset in combinations(network.useful)
    )
    for my_valve in network.valves
        for elephant_valve in network.valves
            values[State((), my_valve, elephant_valve)] = 0
        end
    end
    values
end

"Pre-calculate and store pressure increases"
function calc_pressure_increases(network::Network)
    increases = Dict{Any, Int64}(() => 0)
    for subset in combinations(network.useful)
        increases[Tuple(subset)] = sum(network.flow_rates[valve] for valve in subset)
    end
    increases
end

"Part 1"
function part1()
    network = get_network()
    values = init_values_me_only(network)
    increases = calc_pressure_increases(network)
    for iteration = 1:30
        println(iteration)
        next_values = copy(values)
        for (state, _) in values
            max_value = values[state]
            # try turning this one on, if it has a flow
            if !in(state.my_location, state.turned_on) && in(state.my_location, network.useful)
                turned_on = Tuple(sort([state.turned_on..., state.my_location]))
                max_value = max(max_value, values[State(turned_on, state.my_location)])
            end
            # try moving to a neighbour
            for next in network.neighbours[state.my_location]
                max_value = max(max_value, values[State(state.turned_on, next)])
            end
            # apply the current pressure increase
            increase = increases[state.turned_on]
            # update the values
            next_values[state] = max_value + increase
        end
        values = next_values
    end
    val = values[State((), "AA")]
    println("PART 1: $val")
end

"Part 2"
function part2()
    network = get_network()
    values = init_values_with_elephant(network)
    increases = calc_pressure_increases(network)
    for iteration = 1:26
        println(iteration)
        next_values = copy(values)
        for (state, _) in values
            max_value = values[state]
            # try both of us turning one on (if we're in separate useful places)
            if (
                state.my_location != state.elephant_location &&
                !in(state.my_location, state.turned_on) &&
                in(state.my_location, network.useful) &&
                !in(state.elephant_location, state.turned_on) &&
                in(state.elephant_location, network.useful)
            )
                turned_on = Tuple(sort([state.turned_on..., state.my_location, state.elephant_location]))
                max_value = max(max_value, values[State(turned_on, state.my_location, state.elephant_location)])
            end
            # try just me turning one on and the elephant moving
            if (
                !in(state.my_location, state.turned_on) &&
                in(state.my_location, network.useful)
            )
                turned_on = Tuple(sort([state.turned_on..., state.my_location]))
                for next in [network.neighbours[state.elephant_location]..., state.elephant_location]
                    max_value = max(max_value, values[State(turned_on, state.my_location, next)])
                end
            end
            # try the elephant turning one on and me moving
            if (
                !in(state.elephant_location, state.turned_on) &&
                in(state.elephant_location, network.useful)
            )
                turned_on = Tuple(sort([state.turned_on..., state.elephant_location]))
                for next in [network.neighbours[state.my_location]..., state.my_location]
                    max_value = max(max_value, values[State(turned_on, next, state.elephant_location)])
                end
            end
            # try both of us moving
            for me_next in [network.neighbours[state.my_location]..., state.my_location]
                for elephant_next in [network.neighbours[state.elephant_location]..., state.elephant_location]
                    max_value = max(max_value, values[State(state.turned_on, me_next, elephant_next)])
                end
            end
            # apply the current pressure increase
            increase = increases[state.turned_on]
            # update the values
            next_values[state] = max_value + increase
        end
        values = next_values
    end
    val = values[State((), "AA", "AA")]
    println("PART 2: $val")
end

# 3 hours to code
part1()
part2()  # ...around 18 hours to run!
