"Utility function to load the stuff"
function get_program()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day10.txt")
    open(path) do file
        return readlines(file)
    end
end

"Run the program"
function run_program()
    result = [1]
    cycle = 1
    register = 1
    for line in get_program()
        if line == "noop"
            cycle += 1
        else
            value = parse(Int64, split(line, " ")[end])
            cycle += 1
            push!(result, register)
            register += value
            cycle += 1
        end
        push!(result, register)
    end
    result
end

"Part 1"
function part1()
    result = run_program()
    total = sum([cycle * result[cycle] for cycle in [20, 60, 100, 140, 180, 220]])
    println("PART 1: $total")
end

"Part 2"
function part2()
    println("PART 2:")
    sprite_positions = run_program()
    for start in [1, 41, 81, 121, 161, 201]
        println()
        for (i, pos) in enumerate(view(sprite_positions, start:start + 39))
            pixel = (i >= pos && i <= pos + 2) ? "#" : "."
            print(pixel)
        end
    end
    println()
    println()
end

part1()
part2()