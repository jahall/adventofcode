"Utility function to load stuff"
function get_datastream()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day6.txt")
    open(path) do file
        return collect(readlines(file)[1])
    end
end

"Common functionality"
function find_marker(stream, ndistinct)
    for i = 1:length(stream) - ndistinct + 1
        marker = i + ndistinct - 1
        if length(Set(view(stream, i:marker))) == ndistinct
            return marker
        end
    end
end

"Part 1"
function part1()
    datastream = get_datastream()
    index = find_marker(datastream, 4)
    println("PART 1: $index")
end

"Part 2"
function part2()
    datastream = get_datastream()
    index = find_marker(datastream, 14)
    println("PART 2: $index")
end

part1()
part2()