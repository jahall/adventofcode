"Utility function to load the grid"
function get_grid()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day8.txt")
    open(path) do file
        return [[parse(Int64, c) for c in line] for line in readlines(file)]
    end
end

"Part 1"
function part1()
    set = Set()
    grid = get_grid()
    n, m = length(grid), length(grid[1])
    for i = 1:n
        level = -1
        for j = 1:m - 1
            h = grid[i][j]
            if h > level
                level = h
                push!(set, (i, j))
            end
            if h == 9
                break
            end
        end
        level = -1
        for j = m:-1:2
            h = grid[i][j]
            if h > level
                level = h
                push!(set, (i, j))
            end
            if h == 9
                break
            end
        end
    end
    for j = 1:m
        level = -1
        for i = 1:n - 1
            h = grid[i][j]
            if h > level
                level = h
                push!(set, (i, j))
            end
            if h == 9
                break
            end
        end
        level = -1
        for i = n:-1:2
            h = grid[i][j]
            if h > level
                level = h
                push!(set, (i, j))
            end
            if h == 9
                break
            end
        end
    end
    num = length(set)
    println("PART 1: $num")
end

"Calculate scenic score"
function calc_scenic_score(grid, i, j)
    h = grid[i][j]
    left, right, top, bottom = 0, 0, 0, 0
    for ii = i - 1:-1:1
        left += 1
        if grid[ii][j] >= h
            break
        end
    end
    for ii = i + 1:length(grid)
        right += 1
        if grid[ii][j] >= h
            break
        end
    end
    for jj = j - 1:-1:1
        top += 1
        if grid[i][jj] >= h
            break
        end
    end
    for jj = j + 1:length(grid[1])
        bottom += 1
        if grid[i][jj] >= h
            break
        end
    end
    return left * right * top * bottom
end

"Part 2"
function part2()
    grid = get_grid()
    n, m = length(grid), length(grid[1])
    best = 0
    for i = 1:n
        for j = 1:m
            score = calc_scenic_score(grid, i, j)
            if score > best
                best = score
            end
        end
    end
    println("PART 2: $best")
end

# <30 mins
part1()
part2()