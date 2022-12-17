"Struct for a point"
struct Point
    x::Int64
    y::Int64
end

"Struct for a rock structure."
struct Rock
    points::Set{Point}
    xmin::Int64
    ymin::Int64
    xmax::Int64
    ymax::Int64

    Rock(points::Set{Point}) = new(
        points,
        min([p.x for p in points]...),
        min([p.y for p in points]...),
        max([p.x for p in points]...),
        max([p.y for p in points]...),
    )

    function Rock(shape::Char, y_offset::Int64)
        if shape == '-'
            points = [
                Point(x + 2, y_offset)
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
                Point(2, y + y_offset)
                for y in [1,2,3,4]
            ]
        elseif shape == '□'
            points = [
                Point(x + 2, y + y_offset)
                for x in [1,2]
                for y in [1,2]
            ]
        end
        new(Set(points))
    end
end

"Tower is a stack of rocks"
mutable struct Tower
    rocks::Vector{Rock}
    height::Int64

    Tower() = new(Vector{Rock}[], 0)
end

"Add a rock to the tower, and update its height"
function add(rock::Rock, tower::Tower)
    push!(tower.rocks, rock)
    if rock.ymax > tower.height
        tower.height = rock.ymax
    end
end

"Does a rock overlap with another rock"
function overlaps(rock1::Rock, rock2::Rock)
    length(intersect(rock1.points, rock2.points)) > 0
end

"Does a rock overlap with the tower"
function overlaps(rock::Rock, tower::Tower)
    for tower_rock in tower.rocks
        if overlaps(rock, tower_rock)
            return true
        end
    end
    false
end

"Move the rock"
function move(rock::Rock, direction::Char, left_wall::Int64, right_wall::Int64)
    if (
        (direction == '<' && rock.xmin > left_wall + 1) ||
        (direction == '>' && rock.xmax < right_wall - 1) ||
        (direction == 'v')
    )
        rock = Rock(Set(move(point, direction) for point in rock.points))
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
    path = joinpath(root, "data", "day17_test.txt")
    open(path) do file
        return readlines(file)[1]
    end
end

"Get next rock"
function next_rock(num::Int64, tower_height::Int64)
    shape = ['-', '+', '⅃', '|', '□'][mod(num - 1, 5) + 1]
    Rock(shape, tower_height + 4)
end

"Get next movement"
function next(movements::String, index::Int64)
    next_index = index < length(movements) ? index + 1 : 1
    next_index, movements[next_index]
end


"Part 1"
function part1()
    tower = Tower()
    movements = get_movements()
    movement_index = 1
    for num = 1:2022
        println(num)
        rock = next_rock(num, tower.height)
        while true
            # move left or right (if possible)
            movement_index, movement = next(movements, movement_index)
            next_rock = move(rock, movement, 0, 8)
            if !overlaps(rock, tower)
                rock = next_rock
            end
            # move down (if possible)
            next_rock = move(rock, 'v', 0, 8)
            if !overlaps(rock, tower) && rock.ymin > 0
                rock = next_rock
            else
                add(rock, tower)
                break
            end
        end
    end
    height = tower.height
    println("PART 1: $height")
end

"Part 2"
function part2()
    println("PART 2:")
end

# 1 hour so far...
part1()
part2()
