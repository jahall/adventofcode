
path = "C:\\Users\\7082tr\\code\\adventofcode\\2022\\data\\day1.txt"

# Part 1
open(path) do file
    lines = readlines(file)
    biggest = 0
    current = 0
    for i in 1:length(lines)
        if lines[i] == ""
            current = 0
        else
            current += parse(Int64, lines[i])
            if current > biggest
                biggest = current
            end
        end
    end
    print("PART 1: ", biggest, "\n")
end

# Part 2
mydata = open(path) do file
    lines = readlines(file)
    elves_calories = []
    current = 0
    for i in 1:length(lines)
        if lines[i] == ""
            push!(elves_calories, current)
            current = 0
        else
            current += parse(Int64, lines[i])
        end
    end
    push!(elves_calories, current)
    sort!(elves_calories)
    n = length(elves_calories)
    solution = sum(view(elves_calories, n-2:n))
    print("PART 2: ", solution)
end