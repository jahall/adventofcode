mutable struct Monkey
    items::Vector{Int64}
    op::String
    arg::Int64
    divisible_test::Int64
    throw_if_true::Int64
    throw_if_false::Int64
    count::Int64
end

"Monkey inspects item"
function inspect(monkey::Monkey, item::Int64)
    arg = (monkey.arg == -1) ? item : monkey.arg
    (monkey.op == "*") ? item * arg : item + arg
end

"The feeling of relief"
function relief(item::Int64)
    item ÷ 3
end

"The new feeling of relief"
function clever_relief(monkeys::Vector{Monkey}, item::Int64)
    mod(item, prod([m.divisible_test for m in monkeys]))
end

"Play a round of the game"
function play_round(monkeys::Vector{Monkey}, part::Int64)
    for monkey in monkeys
        for item in monkey.items
            item = inspect(monkey, item)
            item = (part == 1) ? relief(item) : clever_relief(monkeys, item)
            next = mod(item, monkey.divisible_test) == 0 ? monkey.throw_if_true : monkey.throw_if_false
            push!(monkeys[next].items, item)
            monkey.count += 1
        end
        monkey.items = []
    end
end

"Utility function to load the stuff"
function get_monkeys()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day11.txt")
    monkeys::Vector{Monkey} = []
    open(path) do file
        lines = readlines(file)
        for i = 1:7:length(lines)
            part = split(lines[i + 1], ": ")[end]
            items = [parse(Int64, n) for n in split(part, ", ")]

            parts = split(lines[i + 2], " ")
            op = parts[end - 1]
            arg = (parts[end] == "old") ? -1 : parse(Int64, parts[end])

            m = Monkey(
                items,
                op,
                arg,
                parse(Int64, split(lines[i + 3], " ")[end]),
                parse(Int64, split(lines[i + 4], " ")[end]) + 1,
                parse(Int64, split(lines[i + 5], " ")[end]) + 1,
                0,
            )
            push!(monkeys, m)
        end
    end
    monkeys
end

"Part 1"
function part1()
    monkeys = get_monkeys()
    for _ = 1:20
        play_round(monkeys, 1)
    end
    counts = sort([m.count for m in monkeys], rev=true)
    monkey_business_level = counts[1] * counts[2]
    println("PART 1: $monkey_business_level")
end

"Part 2"
function part2()
    monkeys = get_monkeys()
    for _ = 1:10_000
        play_round(monkeys, 2)
    end
    counts = sort([m.count for m in monkeys], rev=true)
    monkey_business_level = counts[1] * counts[2]
    println("PART 2: $monkey_business_level")
end

part1()
part2()