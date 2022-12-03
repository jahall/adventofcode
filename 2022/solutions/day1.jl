function get_lines()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day1.txt")
    open(path) do file
        return readlines(file)
    end
end

# Part 1
lines = get_lines()
biggest = 0
current = 0
for i in 1:length(lines)
    global current, biggest
    if lines[i] == ""
        current = 0
    else
        current += parse(Int64, lines[i])
        if current > biggest
            biggest = current
        end
    end
end
println("PART 1: $biggest")

# Part 2
elves_calories = []
current = 0
for i in 1:length(lines)
    global current
    if lines[i] == ""
        push!(elves_calories, current)
        current = 0
    else
        current += parse(Int64, lines[i])
    end
end
push!(elves_calories, current)
sort!(elves_calories, rev=true)
solution = sum(view(elves_calories, 1:3))
println("PART 2: $solution")
