import Base.+

"Snafu number"
struct Snafu
    value::String
end

"Utility function to load the snafus"
function get_snafus()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day25.txt")
    open(path) do file
        return [Snafu(req) for req in readlines(file)]
    end
end

"Convert snafu number to integer"
function to_int(snafu::Snafu)::Int64
    num = 0
    to_num = Dict('=' => -2, '-' => -1, '0' => 0, '1' => 1, '2' => 2)
    for (pow, char) in enumerate(reverse(snafu.value))
        num += to_num[char] * 5 ^ (pow - 1)
    end
    num
end

"Convert integer to snafu number"
function to_snafu(num::Int64)::Snafu
    remainder = num
    value::Vector{Char} = []
    to_char = Dict(0 => '0', 1 => '1', 2 => '2', 3 => '=', 4 => '-')
    while remainder != 0
        bucket = mod(remainder, 5)
        remainder -= bucket
        remainder ÷= 5
        if bucket ∈ [3, 4]
            remainder += 1
        end
        push!(value, to_char[bucket])
    end
    Snafu(string(reverse(value)...))
end

"Neat override of addition functionality"
function (+)(x::Snafu, y::Snafu)::Snafu
    to_snafu(to_int(x) + to_int(y))
end

"Part 1"
function part1()
    code = sum(get_snafus()).value
    println("PART 1: $code")
end

"Part 2"
function part2()
    println("PART 2: DONE!!")
end

# 1 hour
part1()
part2()
