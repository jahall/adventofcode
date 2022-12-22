"Utility function to load the data"
function get_data()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day22.txt")
    open(path) do file
        return readlines(file)
    end
end

"A point"
struct Point
    x::Int64
    y::Int64
end

"Current state"
struct State
    location::Point
    facing::Char

    # I don't know why we need this but we do!
    State(location, facing) = new(location, facing)

    function State(points::Set{Point})
        x = min([p.x for p in points if p.y == 1]...)
        new(Point(x, 1), '>')
    end
end

"Get the password from the state"
function to_password(state::State)
    facing_to_num = Dict(['>' => 0, 'v' => 1, '<' => 2, '^' => 3])
    1000 * state.location.y + 4 * state.location.x + facing_to_num[state.facing]
end

"Turn in a particular direction"
function turn(state::State, direction::Char)
    if direction == 'X'
        return state
    end
    new_facings = Dict([
        ('>', 'L') => '^',
        ('^', 'L') => '<',
        ('<', 'L') => 'v',
        ('v', 'L') => '>',
        ('>', 'R') => 'v',
        ('^', 'R') => '>',
        ('<', 'R') => '^',
        ('v', 'R') => '<',
    ])
    State(state.location, new_facings[(state.facing, direction)])
end

"Get the moves"
function get_moves()
    line = get_data()[end]
    moves = [parse(Int64, num) for num in split(line, ['L', 'R'])]
    turns = filter([char for char in line]) do char
        char == 'L' || char == 'R'
    end
    push!(turns, 'X')  # just so lengths match
    moves, turns
end

# Part 1 ---------------------------------------------------------------

"The grid"
struct Grid
    open::Set{Point}
    solid::Set{Point}
    xmins::Vector{Int64}
    xmaxs::Vector{Int64}
    ymins::Vector{Int64}
    ymaxs::Vector{Int64}

    "Load the grid from data"
    function Grid()
        open::Set{Point} = Set()
        solid::Set{Point} = Set()
        data = get_data()
        data = view(data, 1:length(data) - 2)
        xmax, ymax = 0, 0
        for (y, row) in enumerate(data)
            ymax = max(y, ymax)
            for (x, char) in enumerate(row)
                xmax = max(x, xmax)
                if char == '.'
                    push!(open, Point(x, y))
                elseif char == '#'
                    push!(solid, Point(x, y))
                end
            end
        end
        xmins::Vector{Int64} = []
        xmaxs::Vector{Int64} = []
        ymins::Vector{Int64} = []
        ymaxs::Vector{Int64} = []
        valid = union(open, solid)
        for y = 1:ymax
            allx = [p.x for p in filter(p -> p.y == y, valid)]
            push!(xmins, min(allx...))
            push!(xmaxs, max(allx...))
        end
        for x = 1:xmax
            ally = [p.y for p in filter(p -> p.x == x, valid)]
            push!(ymins, min(ally...))
            push!(ymaxs, max(ally...))
        end
        new(open, solid, xmins, xmaxs, ymins, ymaxs)
    end
end

"Move in a particular direction"
function move(state::State, grid::Grid)
    xo = get(Dict(['<' => -1, '>' => 1]), state.facing, 0)
    yo = get(Dict(['^' => -1, 'v' => 1]), state.facing, 0)
    loc = Point(state.location.x + xo, state.location.y + yo)
    # wrap around if necessary
    if xo == 1 && loc.x > grid.xmaxs[loc.y]
        loc = Point(grid.xmins[loc.y], loc.y)
    elseif xo == -1 && loc.x < grid.xmins[loc.y]
        loc = Point(grid.xmaxs[loc.y], loc.y)
    elseif yo == 1 && loc.y > grid.ymaxs[loc.x]
        loc = Point(loc.x, grid.ymins[loc.x])
    elseif yo == -1 && loc.y < grid.ymins[loc.x]
        loc = Point(loc.x, grid.ymaxs[loc.x])
    end
    # move if its open space
    if in(loc, grid.open)
        state = State(loc, state.facing)
    end
    state
end

"Part 1"
function part1()
    grid = Grid()
    moves, turns = get_moves()
    state = State(grid.open)
    for (steps, direction) in zip(moves, turns)
        for _ = 1:steps
            next_state = move(state, grid)
            if next_state == state
                break
            end
            state = next_state
        end
        state = turn(state, direction)
    end
    password = to_password(state)
    println("PART 1: $password")
end

# Part 2 ---------------------------------------------------------------

"Representation of the grid as a cube"
struct Cube
    open::Set{Point}
    solid::Set{Point}
    jumps::Dict{State, State}

    function Cube()
        open::Set{Point} = Set()
        solid::Set{Point} = Set()
        data = get_data()
        data = view(data, 1:length(data) - 2)
        for (y, row) in enumerate(data)
            for (x, char) in enumerate(row)
                if char == '.'
                    push!(open, Point(x, y))
                elseif char == '#'
                    push!(solid, Point(x, y))
                end
            end
        end
        jumps::Dict{State, State} = Dict()
        if length(data) < 50
            # test data
            #
            #          +--+
            #          |C1|
            #    +--+--+--+
            #    |A2|B2|C2|
            #    +--+--+--+--+
            #          |C3|D3|
            #          +--+--+
            #
            update!(jumps, points(9:12, 1), '^', points(4:-1:1, 5), 'v')  # C1 top -> A2 top
            update!(jumps, points(9, 1:4), '<', points(5:8, 5), 'v')  # C1 left -> B2 top
            update!(jumps, points(12, 1:4), '>', points(16, 12:-1:9), '<')  # C1 right -> D3 right
            update!(jumps, points(1, 5:8), '<', points(16:-1:13, 12), '^')  # A2 left -> D3 bottom
            update!(jumps, points(1:4, 8), 'v', points(12:-1:9, 12), '^')  # A2 bottom -> C3 bottom
            update!(jumps, points(5:8, 8), 'v', points(9, 12:-1:9), '>')  # B2 bottom -> C3 left
            update!(jumps, points(12, 5:8), '>', points(16:-1:12, 9), 'v')  # C2 right -> D3 top
        else
            # real data
            #
            #       +--+--+
            #       |B1|C1|
            #       +--+--+
            #       |B2|
            #    +--+--+
            #    |A3|B3|
            #    +--+--+
            #    |A4|
            #    +--+
            #
            update!(jumps, points(51:100, 1), '^', points(1, 151:200), '>')  # B1 top -> A4 right
            update!(jumps, points(51, 1:50), '<', points(1, 150:-1:101), '>')  # B1 left -> A3 left
            update!(jumps, points(101:150, 1), '^', points(1:50, 200), '^')  # C1 top -> A4 bottom
            update!(jumps, points(150, 1:50), '>', points(100, 150:-1:101), '<')  # C1 right -> B3 right
            update!(jumps, points(101:150, 50), 'v', points(100, 51:100), '<')  # C1 bottom -> B2 right
            update!(jumps, points(51, 51:100), '<', points(1:50, 101), 'v')  # B2 left -> A3 top
            update!(jumps, points(51:100, 150), 'v', points(50, 151:200), '<')  # B3 bottom -> A4 right
        end
        new(open, solid, jumps)
    end
end

"Update the jumps dict with forward and reverse moves"
function update!(jumps, points1, d1, points2, d2)
    flip = Dict(['>' => '<', '<' => '>', '^' => 'v', 'v' => '^'])
    d1_back = flip[d1]
    d2_back = flip[d2]
    for (p1, p2) in zip(points1, points2)
        jumps[State(p1, d1)] = State(p2, d2)
        jumps[State(p2, d2_back)] = State(p1, d1_back)
    end
end

"Utility to return a list of points"
function points(xs, ys)
    if typeof(xs) === Int64
        return [Point(xs, y) for y in ys]
    else
        return [Point(x, ys) for x in xs]
    end
end

"Move in a particular direction"
function move(state::State, cube::Cube)
    next_state = nothing
    # check if we should jump to a new face
    if haskey(cube.jumps, state)
        next_state = cube.jumps[state]
    # otherwise do a vanilla move
    else
        xo = get(Dict(['<' => -1, '>' => 1]), state.facing, 0)
        yo = get(Dict(['^' => -1, 'v' => 1]), state.facing, 0)
        next_state = State(
            Point(state.location.x + xo, state.location.y + yo),
            state.facing,
        )
    end
    # then only move if its open space
    if in(next_state.location, cube.open)
        state = next_state
    end
    state
end

"Part 2"
function part2()
    cube = Cube()
    moves, turns = get_moves()
    state = State(cube.open)
    for (steps, direction) in zip(moves, turns)
        for _ = 1:steps
            next_state = move(state, cube)
            if next_state == state
                break
            end
            state = next_state
        end
        state = turn(state, direction)
    end
    password = to_password(state)
    println("PART 2: $password")
end

# 1 hour for part 1, 2 hours for part 2
part1()
part2()
