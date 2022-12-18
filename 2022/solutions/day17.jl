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

    function Rock(points::Set{Point})
        #println([p.x for p in points], [p.y for p in points])
        new(
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
        Rock(Set(points))
    end
end

"Tower is a stack of rocks"
mutable struct Tower
    points::Set{Point}
    height::Int64

    Tower() = new(Set{Point}(), 0)
end

"Add a rock to the tower, and update its height"
function add(rock::Rock, tower::Tower)
    union!(tower.points, rock.points)
    if rock.ymax > tower.height
        tower.height = rock.ymax
    end
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

"Build a tower for a certain number of iterations"
function build_tower(iterations::Int64)
    tower = Tower()
    movements = get_movements()
    movement_index = 0
    for num = 1:iterations
        rock = next_rock(num, tower.height)
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
                add(rock, tower)
                break
            end
        end
    end
    tower
end

"Nice utility to show current status"
function show(tower::Tower, rock::Rock)
    println()
    for y in max(tower.height, rock.ymax) + 1:-1:1
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
    tower = build_tower(2022)
    height = tower.height
    println("PART 1: $height")
end

"Part 2"
function part2()
    tower = build_tower(1_000_000_000_000)
    height = tower.height
    println("PART 2: $height")
end

# 2 hours for part 1
part1()
#part2()
