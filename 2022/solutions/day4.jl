"Utility function to load vector of stuff"
function get_stuff()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day4.txt")
    open(path) do file
        return readlines(file)
    end
end

# Part 1
stuff = get_stuff()
score = 0
println("PART 1: $score")

# Part 2
score = 0s
println("PART 2: $score")