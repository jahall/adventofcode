"Utility function to load vector of rucksacks"
function get_rucksacks()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day3.txt")
    open(path) do file
        return readlines(file)
    end
end

"Split a rucksack into its two compartments"
function split(rucksack)
    n = length(rucksack) ÷ 2
    compartment1 = Set(view(rucksack, 1:n))
    compartment2 = Set(view(rucksack, n+1:2*n))
    (compartment1, compartment2)
end

"Get mapping of character to priority"
function get_priorities()
    priorities = Dict{Char, Integer}()
    items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
    for (priority, item) in enumerate(items)
        priorities[item] = priority
    end
    priorities
end

"Part 1"
function part1()
    rucksacks = get_rucksacks()
    priorities = get_priorities()
    score = 0
    for rucksack in rucksacks
        c1, c2 = split(rucksack)
        i = first(intersect(c1, c2))
        score += priorities[i]
    end
    println("PART 1: $score")
end

"Part 2"
function part2()
    rucksacks = get_rucksacks()
    priorities = get_priorities()
    score = 0
    n = length(rucksacks)
    for i = 1:3:n-2
        (r1, r2, r3) = view(rucksacks, i:i + 2)
        badge = first(intersect(r1, r2, r3))
        score += priorities[badge]
    end
    println("PART 2: $score")
end

part1()
part2()