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

"Cliff as a collection of rocks."
struct Cliff
    rocks::Vector{Rock}
    floor::Int64

    Cliff(rocks) = new(
        rocks,
        max([rock.ymax for rock in rocks]...) + 2,
    )
end

"Utility function to load the cliff"
function get_cliff()
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
    Cliff(rocks)
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
    false
end

"Is this grain of sand in the rocks?"
function in(cliff::Cliff, sand::Sand)
    for rock in cliff.rocks
        if in(rock, sand)
            return true
        end
    end
    sand.y == cliff.floor
end

"Is this grain of sand in the pile of sand?"
function in(pile::Vector{Sand}, sand::Sand)
    for s in pile
        if s == sand
            return true
        end
    end
    false
end

"Simulate dropping a grain of sand."
function pour_sand!(cliff::Cliff, pile::Vector{Sand})
    sand = Sand(500, 0)
    while true
        come_to_rest = true
        for (xo, yo) in [(0, 1), (-1, 1), (1, 1)]
            next = Sand(sand.x + xo, sand.y + yo)
            if !in(cliff, next) && !in(pile, next)
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
function show(cliff::Cliff, pile::Vector{Sand})
    xmin = min([rock.xmin for rock in cliff.rocks]...) - 5
    xmax = max([rock.xmax for rock in cliff.rocks]...) + 5
    ymax = max([rock.ymax for rock in cliff.rocks]...) + 2
    for y = 1:ymax
        println()
        for x = xmin:xmax
            p = Sand(x, y)
            if p == Sand(500, 1)
                print("x")
            elseif in(cliff, p)
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

show(cliff::Cliff) = show(cliff, [])

"Part 1"
function part1()
    cliff = get_cliff()
    pile::Vector{Sand} = []
    num_at_rest = 0
    while true
        pour_sand!(cliff, pile)
        if pile[end].y == cliff.floor - 1
            break
        end
        num_at_rest += 1
    end
    show(cliff, pile)
    println("PART 1: $num_at_rest")
end

"Part 2"
function part2()
    cliff = get_cliff()
    pile::Vector{Sand} = []
    num_at_rest = 0
    while true
        finished = pour_sand!(cliff, pile)
        num_at_rest += 1
        if finished
            break
        end
    end
    println("PART 2: $num_at_rest")
end

# 1.5 - 2 hours
part1()
part2()  # takes just under a minute
