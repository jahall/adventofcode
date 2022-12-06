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
    buffer = copy(view(stream, 1:ndistinct - 1))
    for i = ndistinct:length(stream)
        append!(buffer, stream[i])
        if length(Set(buffer)) == ndistinct
            return i
        end
        popfirst!(buffer)
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