"Sand coordinates."
struct Sand
    x::Int64
    y::Int64
end

"Rock structure."
struct Rock
    edges::Vector{Tuple{Int64, Int64}}
    xmin::Int64
    xmax::Int64
    ymin::Int64
    ymax::Int64

    Rock(edges) = new(
        edges,
        min([x for (x, y) in edges]...),
        max([x for (x, y) in edges]...),
        min([y for (x, y) in edges]...),
        max([y for (x, y) in edges]...),
    )
end

"Utility function to load the stuff"
function get_rocks()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day14.txt")
    rocks::Vector{Rock} = []
    open(path) do file
        for line in readlines(file)
            rock = Rock(
                [
                    (parse(Int64, split(pair, ",")[1]), parse(Int64, split(pair, ",")[2]))
                    for pair in split(line, " -> ")
                ]
            )
            push!(rocks, rock)
        end
    end
    rocks
end

"Get level of the floor."
function floor(rocks::Vector{Rock})
    max([rock.ymax for rock in rocks]...) + 2
end

"Is this grain of sand in the rock?"
function in(rock::Rock, sand::Sand)
    if (sand.x < rock.xmin) || (sand.x > rock.xmax) || (sand.y < rock.ymin) || (sand.y > rock.ymax)
        return false
    end
    n = length(rock.edges)
    for ((x1, y1), (x2, y2)) in zip(view(rock.edges, 1:n - 1), view(rock.edges, 2:n))
        x1, x2 = sort([x1, x2])
        y1, y2 = sort([y1, y2])
        if (sand.x >= x1) && (sand.x <= x2) && (sand.y >= y1) && (sand.y <= y2)
            return true
        end
    end
    return false
end

"Is this grain of sand in the rocks?"
function in(rocks::Vector{Rock}, sand::Sand)
    for rock in rocks
        if in(rock, sand)
            return true
        end
    end
    return sand.y == floor(rocks)
end

"Is this grain of sand in the pile of sand?"
function in(pile::Vector{Sand}, sand::Sand)
    for s in pile
        if s == sand
            return true
        end
    end
    return false
end

"Simulate dropping a grain of sand."
function pour_sand!(rocks::Vector{Rock}, pile::Vector{Sand})
    sand = Sand(500, 0)
    while true
        come_to_rest = true
        for (xo, yo) in [(0, 1), (-1, 1), (1, 1)]
            next = Sand(sand.x + xo, sand.y + yo)
            if !in(rocks, next) && !in(pile, next)
                come_to_rest = false
                sand = next
                break
            end
        end
        if come_to_rest
            push!(pile, sand)
            return sand == Sand(500, 0)
        end
    end
end

"Pretty viz"
function show(rocks::Vector{Rock}, pile::Vector{Sand})
    xmin = min([rock.xmin for rock in rocks]...) - 5
    xmax = max([rock.xmax for rock in rocks]...) + 5
    ymax = max([rock.ymax for rock in rocks]...) + 2
    for y = 1:ymax
        println()
        for x = xmin:xmax
            p = Sand(x, y)
            if p == Sand(500, 1)
                print("x")
            elseif in(rocks, p)
                print("#")
            elseif in(pile, p)
                print("o")
            else
                print(".")
            end
        end
    end
    println()
    println()
end

show(rocks::Vector{Rock}) = show(rocks, [])

"Part 1"
function part1()
    rocks = get_rocks()
    pile::Vector{Sand} = []
    num_at_rest = 0
    while true
        pour_sand!(rocks, pile)
        if pile[end].y == floor(rocks) - 1
            break
        end
        num_at_rest += 1
    end
    show(rocks, pile)
    println("PART 1: $num_at_rest")
end

"Part 2"
function part2()
    rocks = get_rocks()
    pile::Vector{Sand} = []
    num_at_rest = 0
    while true
        finished = pour_sand!(rocks, pile)
        num_at_rest += 1
        if finished
            break
        end
    end
    println("PART 2: $num_at_rest")
end

# x mins
part1()
part2()
