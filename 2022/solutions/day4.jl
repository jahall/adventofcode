"The Range struct"
struct Range
    start::Int64
    stop::Int64
end

"Check if one range fully contains the other"
function contains(outer::Range, inner::Range)
    (inner.start >= outer.start) && (inner.stop <= outer.stop)
end

"Check if two ranges overlap"
function overlaps(r1::Range, r2::Range)
    !((r1.start > r2.stop) || (r1.stop < r2.start))
end

"Utility function to load vector of pairs"
function get_range_pairs()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day4.txt")
    open(path) do file
        lines = readlines(file)
        pairs = [split(line, ",") for line in lines]
        return [
            (
                Range(parse(Int64, split(p1, "-")[1]), parse(Int64, split(p1, "-")[2])),
                Range(parse(Int64, split(p2, "-")[1]), parse(Int64, split(p2, "-")[2]))
            )
            for (p1, p2) in pairs
        ]
    end
end

"Part 1"
function part1()
    pairs = get_range_pairs()
    score = 0
    for (r1, r2) in pairs
        if contains(r1, r2) || contains(r2, r1)
            score += 1
        end
    end
    println("PART 1: $score")
end

"Part 2"
function part2()
    pairs = get_range_pairs()
    score = sum(overlaps(pair...) ? 1 : 0 for pair in pairs)
    println("PART 2: $score")
end

# <30 mins
part1()
part2()