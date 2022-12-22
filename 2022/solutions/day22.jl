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
end

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

"Part 1"
function part1()
    grid = Grid()
    moves, turns = get_moves()
    state = State(Point(grid.xmins[1], 1), '>')
    for (steps, direction) in zip(moves, turns)
        for _ = 1:steps
            next_state = move(state, grid)
            if next_state == state
                break
            end
            state = next_state
            println(state)
        end
        state = turn(state, direction)
        println(state)
    end
    facing_to_num = Dict(['>' => 0, 'v' => 1, '<' => 2, '^' => 3])
    password = 1000 * state.location.y + 4 * state.location.x + facing_to_num[state.facing]
    println("PART 1: $password")
end

"Part 2"
function part2()
    println("PART 2:")
end

# 1 hour for part 1
part1()
part2()
