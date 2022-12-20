"Handy holder for an index and a value"
struct Item
    index::Int64
    value::Int64
end

"Utility function to load the data"
function get_data()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day20.txt")
    open(path) do file
        return [parse(Int64, num) for num in readlines(file)]
    end
end

"Unmix the sequence"
function unmix(seq::Vector{Int64}, decryption_key::Int64, cycles::Int64)
    # perform the unmixing operations
    raw = [Item(i, val * decryption_key) for (i, val) in enumerate(seq)]
    mix = copy(raw)
    for _ = 1:cycles
        for item in raw
            curr_loc = findfirst(x -> x == item, mix)
            shift!(mix, curr_loc, item.value)
        end
    end
    mix = [item.value for item in mix]
    # put the zero at the start
    put_zero_at_start(mix)
end

"Unmix the sequence...without the decryption_key and only once"
function unmix(seq::Vector{Int64})
    unmix(seq, 1, 1)
end

"Fetch value at a particular index after 0"
function fetch(seq::Vector{Int64}, index::Int64)
    seq[mod(index, length(seq)) + 1]
end

"Put the zero at the start"
function put_zero_at_start(seq::Vector{Int64})
    start = findfirst(x -> x == 0, seq)
    if start > 1
        n = length(seq)
        seq = [view(seq, start:n); view(seq, 1:start - 1)]
    end
    seq
end

"Shift a value in a vector"
function shift!(seq::Vector, index::Int64, by::Int64)
    if by == 0
        return seq
    end
    # first remove the item from the list
    value = seq[index]
    deleteat!(seq, index)
    # then calculate new index and insert
    n = length(seq)
    new_index = mod(index + by - 1, n) + 1
    insert!(seq, new_index, value)
    seq
end

"Part 1"
function part1()
    unmixed = unmix(get_data())
    answer = sum(fetch(unmixed, i) for i in (1000, 2000, 3000))
    println("PART 1: $answer")
end

"Part 2"
function part2()
    unmixed = unmix(get_data(), 811589153, 10)
    answer = sum(fetch(unmixed, i) for i in (1000, 2000, 3000))
    println("PART 2: $answer")
end

# 2 hours for part 1 ...5 mins for part 2
part1()
part2()
