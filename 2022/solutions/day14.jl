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
    return false
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

"Simulate dropping a grain of sand."
function pour_sand!(rocks::Vector{Rock}, pile::Vector{Sand})
    sand = Sand(500, 0)
    ymax = max([rock.ymax for rock in rocks]...)
    while true
        come_to_rest = true
        for (xo, yo) in [(0, 1), (-1, 1), (1, 1)]
            next = Sand(sand.x + xo, sand.y + yo)
            if !in(rocks, next) && !in(pile, next)
                come_to_rest = false
                sand = next
            end
        end
        if come_to_rest
            push!(pile, sand)
            return true
        end
        if sand.y > ymax
            # now below all rock formations
            return false
        end
    end
end

"Part 1"
function part1()
    rocks = get_rocks()
    pile::Vector{Sand} = []
    num_at_rest = 0
    while true
        come_to_rest = pour_sand!(rocks, pile)
        if !come_to_rest
            break
        end
        num_at_rest += 1
    end
    println("PART 1: $num_at_rest")
end

"Part 2"
function part2()
    println("PART 2:")
end

# x mins
part1()
part2()