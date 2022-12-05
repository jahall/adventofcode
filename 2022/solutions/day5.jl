using DataStructures

"Utility function to load stacks"
function get_stacks()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day5.txt")
    open(path) do file
        lines = readlines(file)
        stacks = Array{Any}(undef, 9)
        for i = 1:9
            stack = Stack{AbstractChar}()
            for j = 8:-1:1
                item = lines[j][4 * i - 2]
                if item == ' '
                    break
                end
                push!(stack, item)
            end
            stacks[i] = stack
        end
        return stacks
    end
end

"Utility function to load stack operations"
function get_ops()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day5.txt")
    ops = []
    open(path) do file
        for line in readlines(file)[11:end]
            parts = split(line, " ")
            op = (
                parse(Int64, parts[2]),
                parse(Int64, parts[4]),
                parse(Int64, parts[6])
            )
            push!(ops, op)
        end
    end
    ops
end

"Part 1"
function part1()
    stacks = get_stacks()
    for (num, source, target) in get_ops()
        for i in 1:num
            val = pop!(stacks[source])
            push!(stacks[target], val)
        end
    end
    code = string([pop!(stack) for stack in stacks]...)
    println("PART 1: $code")
end

"Part 2"
function part2()
    stacks = get_stacks()
    for (num, source, target) in get_ops()
        vals = [pop!(stacks[source]) for _ = 1:num]
        [push!(stacks[target], val) for val in vals[end:-1:1]]
    end
    code = string([pop!(stack) for stack in stacks]...)
    println("PART 2: $code")
end

part1()
part2()