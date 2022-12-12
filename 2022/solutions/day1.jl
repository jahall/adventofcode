"Read input lines"
function get_lines()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day1.txt")
    open(path) do file
        return readlines(file)
    end
end

"Part 1"
function part1()
    lines = get_lines()
    biggest = 0
    current = 0
    for line in lines
        if line == ""
            current = 0
        else
            current += parse(Int64, line)
            if current > biggest
                biggest = current
            end
        end
    end
    println("PART 1: $biggest")
end

"Part 2"
function part2()
    lines = get_lines()
    elves_calories = []
    current = 0
    for line in lines
        if line == ""
            push!(elves_calories, current)
            current = 0
        else
            current += parse(Int64, line)
        end
    end
    push!(elves_calories, current)
    sort!(elves_calories, rev=true)
    solution = sum(view(elves_calories, 1:3))
    println("PART 2: $solution")
end

# <30 mins
part1()
part2()
