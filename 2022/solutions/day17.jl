"Struct for a point"
struct Point
    x::Int64
    y::Int64
end

"Struct for a rock structure."
struct Rock
    shape::Char
    points::Set{Point}
    xmin::Int64
    ymin::Int64
    xmax::Int64
    ymax::Int64

    function Rock(shape::Char, points::Set{Point})
        new(
            shape,
            points,
            min([p.x for p in points]...),
            min([p.y for p in points]...),
            max([p.x for p in points]...),
            max([p.y for p in points]...),
        )
    end

    function Rock(shape::Char, y_offset::Int64)
        if shape == '-'
            points = [
                Point(x + 2, y_offset + 1)
                for x in [1,2,3,4]
            ]
        elseif shape == '+'
            points = [
                Point(x + 2, y + y_offset)
                for (x, y) in [(1,2), (2,1), (2,2), (2,3), (3,2)]
            ]
        elseif shape == '⅃'
            points = [
                Point(x + 2, y + y_offset)
                for (x, y) in [(1,1), (2,1), (3,1), (3,2), (3,3)]
            ]
        elseif shape == '|'
            points = [
                Point(3, y + y_offset)
                for y in [1,2,3,4]
            ]
        elseif shape == '□'
            points = [
                Point(x + 2, y + y_offset)
                for x in [1,2]
                for y in [1,2]
            ]
        end
        # can't do `new` or we end up with random xmin, xmax, etc!!
        Rock(shape, Set(points))
    end
end

"Tower is a stack of rocks"
struct Tower
    points::Set{Point}
    rocks::Vector{Rock}

    Tower() = new(Set{Point}(), Vector{Rock}())
end

"Add a rock to the tower"
function add!(tower::Tower, rock::Rock)
    union!(tower.points, rock.points)
    push!(tower.rocks, rock)
end

"Does a rock overlap with another rock"
function overlaps(rock1::Rock, rock2::Rock)
    length(intersect(rock1.points, rock2.points)) > 0
end

"Does a rock overlap with a tower"
function overlaps(rock::Rock, tower::Tower)
    length(intersect(rock.points, tower.points)) > 0
end

"Move the rock"
function move(rock::Rock, direction::Char, left_wall::Int64, right_wall::Int64)
    if (
        (direction == '<' && rock.xmin > left_wall + 1) ||
        (direction == '>' && rock.xmax < right_wall - 1) ||
        (direction == 'v')
    )
        rock = Rock(rock.shape, Set(move(point, direction) for point in rock.points))
    end
    rock
end

"Move the point"
function move(point::Point, direction::Char)
    if direction == '<'
        point = Point(point.x - 1, point.y)
    elseif direction == '>'
        point = Point(point.x + 1, point.y)
    elseif direction == 'v'
        point = Point(point.x, point.y - 1)
    end
    point
end

"Utility function to load the movements"
function get_movements()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day17.txt")
    open(path) do file
        return readlines(file)[1]
    end
end

"Get next rock"
function next_rock(num::Int64, tower_height::Int64)
    shape = ['-', '+', '⅃', '|', '□'][mod(num - 1, 5) + 1]
    Rock(shape, tower_height + 3)
end

"Get next movement"
function next(movements::String, index::Int64)
    next_index = index < length(movements) ? index + 1 : 1
    next_index, movements[next_index]
end

"Pattern covers all horizontal space"
function covers_all_horizontal_space(rocks::Vector{Rock})
    length(Set(x for r in rocks for x = r.xmin:r.xmax)) == 7
end

"Calculate height of a pile of rocks"
function calc_height(rocks)
    max([r.ymax for r in rocks]...) - min([r.ymin for r in rocks]...) + 1
end

"Build a tower for a certain number of iterations"
function calc_tower_height(iterations::Int64)
    tower = Tower()
    height = 0
    movement_index, movements = 0, get_movements()

    repeating::Vector{Rock} = []
    repeat_overlap = 0
    completed = 0

    # 1. Simulate rocks falling until we identify a repeated pattern
    for num = 1:iterations

        # a. move the rock into place
        rock = next_rock(num, height)
        start_index = movement_index 
        while true
            # move left or right (if possible)
            movement_index, movement = next(movements, movement_index)
            next_rock = move(rock, movement, 0, 8)
            if !overlaps(next_rock, tower)
                rock = next_rock
            end
            # move down (if possible)
            next_rock = move(rock, 'v', 0, 8)
            if !overlaps(next_rock, tower) && next_rock.ymin > 0
                rock = next_rock
            else
                add!(tower, rock)
                height = max(height, rock.ymax)
                break
            end
        end
        completed += 1
        
        # b. try and find repeating pattern each time we wrap around
        if movement_index < start_index
            n = length(tower.rocks)
            found_match = false
            for size = 5:5:div(n, 2)
                prev = view(tower.rocks, n - 2 * size + 1:n - size)
                curr = view(tower.rocks, n - size + 1:n)
                if patterns_match(prev, curr)
                    found_match = true
                    repeating = [rock for rock in curr]
                    # there may be some overlap in how the ends of the pattern slot together
                    repeat_overlap = max([r.ymax for r in prev]...) - min([r.ymin for r in curr]...) + 1
                    n_repeating = length(repeating)
                    println("Found repeating pattern of size $n_repeating")
                    break
                end
            end
            if found_match
                break
            end
        end
    end

    # 2. Extrapolate out based on repeating pattern
    remaining = iterations - completed
    if remaining > 0
        n_repeating = length(repeating)
        (n_repeats, leftover) = divrem(remaining, n_repeating)

        # a. add on the full number of patterns that fit
        repeat_height = calc_height(repeating)
        height += n_repeats * (repeat_height - repeat_overlap)

        # b. then handle anything leftover
        if leftover > 0
            leftover_height = calc_height(view(repeating, 1:leftover))
            height += max(leftover_height - repeat_overlap, 0)
        end
    end

    height
end

"Check if two rock patterns match"
function patterns_match(prev, next)
    if length(prev) != length(next)
        return false
    end
    offset = next[1].ymin - prev[1].ymin
    for (r1, r2) in zip(prev, next)
        if r1.shape != r2.shape || r1.xmin != r2.xmin || r1.ymin != r2.ymin - offset
            return false
        end
    end
    true
end

"Nice utility to show current status"
function show(tower::Tower, rock::Rock)
    println()
    for y in max([point.y for point in tower.points]..., rock.ymax) + 1:-1:1
        print("|")
        for x in 1:7
            p = Point(x, y)
            print(in(p, rock.points) ? "@" : in(p, tower.points) ? "#" : ".")
        end
        println("|")
    end
    println("+-------+")
    println()
end

"Part 1"
function part1()
    height = calc_tower_height(2022)
    println("PART 1: $height")
end

"Part 2"
function part2()
    height = calc_tower_height(1_000_000_000_000)
    println("PART 2: $height")
end

# 5 hours (2 for part 1 and 3 for part 2)
part1()
part2()
