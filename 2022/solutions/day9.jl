"Utility function to load the movements"
function get_movements()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day9.txt")
    open(path) do file
        return [
            (split(line, " ")[1], parse(Int64, split(line, " ")[2]))
            for line in readlines(file)
        ]
    end
end

"Move head in a particular direction"
function move_head(h, dir)
    if dir == "U"
        return (h[1] + 1, h[2])
    elseif dir == "D"
        return (h[1] - 1, h[2])
    elseif dir == "L"
        return (h[1], h[2] - 1)
    elseif dir == "R"
        return (h[1], h[2] + 1)
    end
end

"Move tail to follow the head"
function move_tail(t, h)
    diff = (t[1] - h[1], t[2] - h[2])
    if diff == (-2, 0)
        # move up
        return (t[1] + 1, t[2])
    elseif diff == (2, 0)
        # move down
        return (t[1] - 1, t[2])
    elseif diff == (0, 2)
        # move left
        return (t[1], t[2] - 1)
    elseif diff == (0, -2)
        # move right
        return (t[1], t[2] + 1)
    elseif diff == (-2, -2) || diff == (-1, -2) || diff == (-2, -1)
        # move diagonally up and right
        return (t[1] + 1, t[2] + 1)
    elseif diff == (2, -2) || diff == (1, -2) || diff == (2, -1)
        # move diagonally down and right
        return (t[1] - 1, t[2] + 1)
    elseif diff == (2, 2) || diff == (1, 2) || diff == (2, 1)
        # move diagonally down and left
        return (t[1] - 1, t[2] - 1)
    elseif diff == (-2, 2) || diff == (-2, 1) || diff == (-1, 2)
        # move diagonally up and left
        return (t[1] + 1, t[2] - 1)
    else
        # do nothing
        return t
    end
end

"Part 1"
function part1()
    head, tail = (1,1), (1,1)
    visited = Set()
    push!(visited, tail)
    for (dir, num) in get_movements()
        for _ = 1:num
            head = move_head(head, dir)
            tail = move_tail(tail, head)
            push!(visited, tail)
        end
    end
    num = length(visited)
    println("PART 1: $num")
end

"Part 2"
function part2()
    head = (1,1)
    tails = [(1,1) for _ = 1:9]
    visited = Set()
    push!(visited, tails[end])
    for (dir, num) in get_movements()
        for _ = 1:num
            head = move_head(head, dir)
            next = head
            for (i, tail) in enumerate(tails)
                tails[i] = move_tail(tail, next)
                next = tails[i]
            end
            push!(visited, tails[end])
        end
    end
    num = length(visited)
    println("PART 2: $num")
end

# 30 mins
part1()
part2()