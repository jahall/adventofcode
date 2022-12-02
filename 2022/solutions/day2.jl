path = "C:\\Users\\7082tr\\code\\adventofcode\\2022\\data\\day2.txt"

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
open(path) do file
    lines = readlines(file)

    remap = Dict('X' => 'A', 'Y' => 'B', 'Z' => 'C')
    total = 0
    for line in lines
        me = remap[line[end]]
        op = line[1]
        total += getscore(me, op)
    end
    println("PART 1: $total")
end


# Part 2
open(path) do file
    lines = readlines(file)

    to_lose = Dict('A' => 'C', 'B' => 'A', 'C' => 'B')
    to_win = Dict('C' => 'A', 'A' => 'B', 'B' => 'C')
    total = 0
    for line in lines
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
end