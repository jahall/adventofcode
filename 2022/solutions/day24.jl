using DataStructures

import Base: +, -

"Handy point struct"
struct Point
    x::Int64
    y::Int64
end

(+)(a::Point, b::Point) = Point(a.x + b.x, a.y + b.y)
(-)(a::Point, b::Point) = Point(a.x - b.x, a.y - b.y)
(+)(a::Point, b::Tuple{Int64, Int64}) = Point(a.x + b[1], a.y + b[2])
(-)(a::Point, b::Tuple{Int64, Int64}) = Point(a.x - b[1], a.y - b[2])

"Holder for useful metadata about the scenario"
struct Scenario
    start::Point
    target::Point
    nrows::Int64
    ncols::Int64
    steps::Int64
    free_space::Vector{Set{Point}}

    function Scenario(layout::Vector{String})
        nrows = length(layout) - 2
        ncols = length(layout[1]) - 2
        new(
            Point(1, 0),
            Point(ncols, nrows + 1),
            nrows,
            ncols,
            lcm(nrows, ncols),
            find_free_space(layout, nrows, ncols),
        )
    end
end

"Utility function to load the stuff"
function get_layout()::Vector{String}
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day24.txt")
    open(path) do file
        return readlines(file)
    end
end

"Least common multiplier"
lcm(a::Int64, b::Int64)::Int64 = (a * b) ÷ gcd(a, b) 

"Simulate blizzards until they repeat and find free space at each step"
function find_free_space(layout::Vector{String}, nrows::Int64, ncols::Int64)::Vector{Set{Point}}
    steps = lcm(nrows, ncols)
    all_points = Set(Point(x, y) for x = 1:ncols for y = 1:nrows)
    free = [copy(all_points) for _ = 1:steps]
    for x = 1:ncols
        for y = 1:nrows
            p = Point(x, y)
            direction = layout[y + 1][x + 1]
            if direction != '.'
                update_free_space(free, p, direction, nrows, ncols)
            end
        end
    end
    free
end

"Update all free space based on this blizzard"
function update_free_space(free::Vector{Set{Point}}, p::Point, direction::Char, nrows::Int64, ncols::Int64)
    for points in free
        pop!(points, p, nothing)
        if direction == 'v'
            p += (0, 1)
            p = (p.y == nrows + 1) ? Point(p.x, 1) : p
        elseif direction == '^'
            p += (0, -1)
            p = (p.y == 0) ? Point(p.x, nrows) : p
        elseif direction == '>'
            p += (1, 0)
            p = (p.x == ncols + 1) ? Point(1, p.y) : p
        elseif direction == '<'
            p += (-1, 0)
            p = (p.x == 0) ? Point(ncols, p.y) : p
        end
    end
end

"Current state"
struct State
    location::Point
    mod_time::Int64
end

"Neighbors available at next timestep"
function neighbors(state::State, scenario::Scenario)::Vector{State}
    next_time = (state.mod_time == scenario.steps) ? 1 : state.mod_time + 1
    free = scenario.free_space[next_time]
    nbrs::Vector{State} = []
    for offset in [(0, 0), (0, 1), (0, -1), (1, 0), (-1, 0)]
        loc = state.location + offset
        if loc ∈ free || loc == scenario.start || loc == scenario.target
            push!(nbrs, State(loc, next_time))
        end
    end
    nbrs
end

"Find shortest path - Djikastra"
function find_shortest_path(scenario::Scenario, start::State, target::Point)::Int64
    pq = PriorityQueue()
    visited = Set([start])
    enqueue!(pq, (start, 0), 0)
    while length(pq) > 0
        (state, steps) = dequeue!(pq)
        if state.location == target
            return steps
        end
        for nbr in neighbors(state, scenario)
            if nbr ∉ visited
                push!(visited, nbr)
                enqueue!(pq, (nbr, steps + 1), steps + 1)
            end
        end
    end
end

"Part 1"
function part1()
    scenario = Scenario(get_layout())
    steps = find_shortest_path(scenario, State(scenario.start, 1), scenario.target)
    println("PART 1: $steps")
end

"Part 2"
function part2()
    scenario = Scenario(get_layout())
    # 1. Go out
    steps = find_shortest_path(scenario, State(scenario.start, 1), scenario.target)
    # 2. Come back
    mod_time = mod(steps, scenario.steps) + 1
    steps += find_shortest_path(scenario, State(scenario.target, mod_time), scenario.start)
    # 3. Go out again!
    mod_time = mod(steps, scenario.steps) + 1
    steps += find_shortest_path(scenario, State(scenario.start, mod_time), scenario.target)
    println("PART 2: $steps")
end

# 1.5 hours
part1()
part2()
