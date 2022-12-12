using DataStructures

"Utility function to load the stuff"
function get_grid()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day12.txt")
    to_num(char) = Int64(codepoint(char)) - 96
    grid::Vector{Vector{Int64}} = []
    start, target = (-1, -1), (-1, -1)
    open(path) do file
        for (i, line) in enumerate(readlines(file))
            push!(grid, [])
            for (j, char) in enumerate(line)
                if char == 'S'
                    start = (i, j)
                    val = 1
                elseif char == 'E'
                    target = (i, j)
                    val = 26
                else
                    val = to_num(char)
                end
                push!(grid[end], val)
            end
        end
    end
    grid, start, target
end

"Get neighbors"
function neighbors(grid, point)
    r, c = point
    rows, cols = length(grid), length(grid[1])
    neighbors = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
    is_valid(p) = (p[1] >= 1) && (p[1] <= rows) && (p[2] >= 1) && (p[2] <= cols)
    neighbors = filter(is_valid, neighbors)
    return [((r, c), grid[r][c]) for (r, c) in neighbors]
end

"Find shortest path - Djikastra"
function find_shortest_path(grid, start, target)
    pq = PriorityQueue()
    visited = Set([start])
    enqueue!(pq, (start, 1, 0), 0)
    shortest = -1
    while length(pq) > 0
        (node, h, path) = dequeue!(pq)
        if node == target
            shortest = path
            break
        end
        for (neighbor, n_h) in neighbors(grid, node)
            if n_h <= h + 1 && !in(neighbor, visited)
                push!(visited, neighbor)
                enqueue!(pq, (neighbor, n_h, path + 1), path + 1)
            end
        end
    end
    shortest
end

"Part 1"
function part1()
    grid, start, target = get_grid()
    path = find_shortest_path(grid, start, target)
    println("PART 1: $path")
end

"Part 2"
function part2()
    grid, _, target = get_grid()
    shortest = -1
    for (i, row) in enumerate(grid)
        for (j, h) in enumerate(row)
            if h == 1
                path = find_shortest_path(grid, (i, j), target)
                if path != -1 && (shortest == -1 || path < shortest)
                    shortest = path
                end
            end
        end
    end
    println("PART 2: $shortest")
end

# 40 mins
part1()
part2()