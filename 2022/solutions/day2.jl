function get_lines()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day2.txt")
    open(path) do file
        return readlines(file)
    end
end


"Utility function to calculate score given me and an oponent"
function getscore(me, op)
    scores = Dict('A' => 1, 'B' => 2, 'C' => 3)
    score = scores[me]
    if me == op
        score += 3
    elseif (me == 'A' && op == 'C') || (me == 'B' && op == 'A') || (me == 'C' && op == 'B')
        score += 6
    end
    score
end


# Part 1
lines = get_lines()
remap = Dict('X' => 'A', 'Y' => 'B', 'Z' => 'C')
total = 0
for line in lines
    global total
    me = remap[line[end]]
    op = line[1]
    total += getscore(me, op)
end
println("PART 1: $total")


# Part 2
to_lose = Dict('A' => 'C', 'B' => 'A', 'C' => 'B')
to_win = Dict('C' => 'A', 'A' => 'B', 'B' => 'C')
total = 0
for line in lines
    global total
    me = line[end]
    op = line[1]
    if me == 'X'
        me = to_lose[op]
    elseif me == 'Y'
        me = op
    else
        me = to_win[op]
    end
    total += getscore(me, op)
end
println("PART 2: $total")
