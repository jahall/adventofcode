path = "C:\\Users\\7082tr\\code\\adventofcode\\2022\\data\\day3.txt"

function get_rucksacks()
    open(path) do file
        return readlines(file)
    end
end

function split(rucksack)
    n = length(rucksack) ÷ 2
    compartment1 = Set(view(rucksack, 1:n))
    compartment2 = Set(view(rucksack, n+1:2*n))
    (compartment1, compartment2)
end

priorities = Dict()
items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
for (priority, item) in enumerate(items)
    global priorities
    priorities[item] = priority
end

# Part 1
rucksacks = get_rucksacks()
score = 0
for rucksack in rucksacks
    global score
    c1, c2 = split(rucksack)
    i = first(intersect(c1, c2))
    score += priorities[i]
end
println("PART 1: $score")

# Part 2
score = 0
n = length(rucksacks)
for i = 1:3:n-2
    global score
    (r1, r2, r3) = view(rucksacks, i:i + 2)
    badge = first(intersect(r1, intersect(r2, r3)))
    score += priorities[badge]
end
println("PART 2: $score")