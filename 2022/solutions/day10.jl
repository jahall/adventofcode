"Utility function to load the stuff"
function get_stuff()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day10.txt")
    open(path) do file
        return readlines(file)
    end
end

"Part 1"
function part1()
    println("PART 1:")
end

"Part 2"
function part2()
    println("PART 2:")
end

part1()
part2()