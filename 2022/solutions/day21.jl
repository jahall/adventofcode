"Utility function to load the stuff"
function get_droplets()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day21_test.txt")
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

# x mins
part1()
part2()
