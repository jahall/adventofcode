"Struct for an elf's location"
struct Elf
    x::Int64
    y::Int64
end

"Utility function to load the stuff"
function get_elves()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day23.txt")

    elves::Set{Elf} = Set()
    open(path) do file
        for (y, row) in enumerate(readlines(file))
            for (x, char) in enumerate(row)
                if char == '#'
                    push!(elves, Elf(x, y))
                end
            end
        end
    end
    elves
end

"Get all neighbors of an elf"
function neighbors(elf::Elf; direction = "all")
    offsets = []
    if direction == "all"
        offsets = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
    elseif direction == "north"  # y = -1
        offsets = [(-1, -1), (0, -1), (1, -1)]
    elseif direction == "south"  # y = 1
        offsets = [(-1, 1), (0, 1), (1, 1)]
    elseif direction == "west"  # x = -1
        offsets = [(-1, -1), (-1, 0), (-1, 1)]
    elseif direction == "east"  # x = 1
        offsets = [(1, -1), (1, 0), (1, 1)]
    end
    Set{Elf}(Elf(elf.x + x, elf.y + y) for (x, y) in offsets)
end

"Propose next location"
function propose(elf::Elf, elves::Set{Elf}, directions::Vector{String})
    moves = Dict(
        "all" => (0, 0),
        "north" => (0, -1),
        "south" => (0, 1),
        "west" => (-1, 0),
        "east" => (1, 0),
    )
    for direction in [["all"]; directions]
        nbrs = neighbors(elf, direction=direction)
        if length(nbrs ∩ elves) == 0
            x, y = moves[direction]
            return Elf(elf.x + x, elf.y + y)
        end
    end
    elf
end

"Find duplicate locations"
function find_duplicates(proposed::Dict{Elf, Elf})
    used::Set{Elf} = Set()
    dups::Set{Elf} = Set()
    for elf in values(proposed)
        if elf ∈ used
            push!(dups, elf)
        else
            push!(used, elf)
        end
    end
    dups
end

"Calculate num empty tiles"
function calc_empty(elves::Set{Elf})::Int64
    xmin = min([elf.x for elf in elves]...)
    xmax = max([elf.x for elf in elves]...)
    ymin = min([elf.y for elf in elves]...)
    ymax = max([elf.y for elf in elves]...)
    area = (xmax - xmin + 1) * (ymax - ymin + 1)
    area - length(elves)
end

"Show current layout"
function show(elves::Set{Elf})
    xmin = min([elf.x for elf in elves]...)
    xmax = max([elf.x for elf in elves]...)
    ymin = min([elf.y for elf in elves]...)
    ymax = max([elf.y for elf in elves]...)
    for y = ymin:ymax
        println()
        for x = xmin:xmax
            print(Elf(x, y) ∈ elves ? "#" : ".")
        end
    end
    println()
end

"Simulate one round"
function simulate(elves::Set{Elf}, directions::Vector{String})::Set{Elf}
    # 1. Propose locations
    proposed = Dict{Elf, Elf}(
        elf => propose(elf, elves, directions)
        for elf in elves
    )
    # 2. Cull the locations
    duplicates = find_duplicates(proposed)
    next_elves = Set(
        proposed[elf] ∈ duplicates ? elf : proposed[elf]
        for elf in elves
    )
    # 3. Cycle the directions
    push!(directions, popfirst!(directions))
    next_elves
end

"Part 1"
function part1()
    elves = get_elves()
    directions::Vector{String} = ["north", "south", "west", "east"]
    for _ = 1:10
        elves = simulate(elves, directions)
    end
    empty = calc_empty(elves)
    println("PART 1: $empty")
end

"Part 2"
function part2()
    elves = get_elves()
    directions::Vector{String} = ["north", "south", "west", "east"]
    round = 0
    while true
        round += 1
        next_elves = simulate(elves, directions)
        if elves == next_elves
            elves = next_elves
            break
        end
        elves = next_elves
    end
    println("PART 2: $round")
end

# 1 hour
part1()
part2()
