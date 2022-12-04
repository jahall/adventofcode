"Utility function to load vector of pairs"
function get_range_pairs()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day4.txt")
    open(path) do file
        lines = readlines(file)
        pairs = [split(line, ",") for line in lines]
        return [
            (
                (parse(Int64, split(p1, "-")[1]), parse(Int64, split(p1, "-")[2])),
                (parse(Int64, split(p2, "-")[1]), parse(Int64, split(p2, "-")[2]))
            )
            for (p1, p2) in pairs
        ]
    end
end

# Part 1
pairs = get_range_pairs()
score = 0
for ((p1_s, p1_e), (p2_s, p2_e)) in pairs
    global score
    if (p1_s >= p2_s) && (p1_e <= p2_e)
        score += 1
    elseif (p2_s >= p1_s) && (p2_e <= p1_e)
        score += 1
    end
end
println("PART 1: $score")

# Part 2
score = 0
println("PART 2: $score")