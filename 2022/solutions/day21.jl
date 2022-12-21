import Base: +, -, *, ÷

"Utility function to load the monkey operations"
function get_ops()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day21.txt")
    ops::Dict{String, Vector{String}} = Dict()
    open(path) do file
        for line in readlines(file)
            key, val = split(line, ": ")
            ops[key] = split(val, " ")
        end
    end
    ops
end

"Evalulate a particular node"
function eval(ops, monkey::String; cache=Dict())
    if haskey(cache, monkey)
        return cache[monkey]
    end
    op = ops[monkey]
    if length(op) == 1
        cache[monkey] = parse(Int64, op[1])
    else
        left, op, right = op
        left = eval(ops, left, cache=cache)
        right = eval(ops, right, cache=cache)
        if op == "+"
            cache[monkey] = left + right
        elseif op == "-"
            cache[monkey] = left - right
        elseif op == "*"
            cache[monkey] = left * right
        elseif op == "/"
            cache[monkey] = left ÷ right
        end
    end
    cache[monkey]    
end

"A polynomial"
struct Polynomial
    coeffs::Vector{Int64}
end

"Get coefficient at index ...or zero if doesn't exist"
function get(poly::Polynomial, index::Int64)
    index > length(poly.coeffs) ? 0 : poly.coeffs[index]
end

"Add two polynomials together"
function (+)(p1::Polynomial, p2::Polynomial)
    coeffs::Vector{Int64} = []
    for i = 1:max(length(p1.coeffs), length(p2.coeffs))
        push!(coeffs, get(p1, i) + get(p2, i))
    end
    Polynomial(coeffs)
end

"Subtract one polynomial from another"
function (-)(p1::Polynomial, p2::Polynomial)
    coeffs::Vector{Int64} = []
    for i = 1:max(length(p1.coeffs), length(p2.coeffs))
        push!(coeffs, get(p1, i) - get(p2, i))
    end
    Polynomial(coeffs)
end

"Multiply two polynomials together"
function (*)(p1::Polynomial, p2::Polynomial)
    coeffs::Vector{Int64} = []
    for (c1_deg, c1) in enumerate(p1.coeffs)
        for (c2_deg, c2) in enumerate(p2.coeffs)
            target = c1_deg + c2_deg - 1
            if target > length(coeffs)
                push!(coeffs, 0)
            end
            coeffs[target] += c1 * c2
        end
    end
    Polynomial(coeffs)
end

"Divide a polynomial by a scalar"
function (÷)(p::Polynomial, divisor::Int64)
    Polynomial([c ÷ divisor for c in p.coeffs])
end

"A value representing a polynomial divided by another polynomial"
struct Value
    num::Polynomial
    den::Polynomial

    Value(num::Int64) = new(Polynomial([num]), Polynomial([1]))
    Value(num::Vector{Int64}) = new(Polynomial(num), Polynomial([1]))
    Value(num::Polynomial) = new(num, Polynomial([1]))

    "Get rid of any common divisors"
    function Value(num::Polynomial, den::Polynomial)
        common = gcd(num.coeffs..., den.coeffs...)
        new(num ÷ common, den ÷ common)
    end
end

"Multiply two values"
function (*)(v1::Value, v2::Value)
    Value(v1.num * v2.num, v1.den * v2.den)
end

"Divide two values"
function (÷)(v1::Value, v2::Value)
    Value(v1.num * v2.den, v1.den * v2.num)
end

"Add two values together"
function (+)(v1::Value, v2::Value)
    Value((v1.num * v2.den) + (v2.num * v1.den), v1.den * v2.den)
end

"Subtract two values"
function (-)(v1::Value, v2::Value)
    Value((v1.num * v2.den) - (v2.num * v1.den), v1.den * v2.den)
end

"Evalulate a particular node...using polynomials based on the value of humn"
function eval_humn(ops, monkey::String; cache=Dict())
    if haskey(cache, monkey)
        return cache[monkey]
    end
    op = ops[monkey]
    if monkey == "humn"
        cache[monkey] = Value([0, 1])
    elseif length(op) == 1
        cache[monkey] = Value([parse(Int64, op[1])])
    else
        left, op, right = op
        left = eval_humn(ops, left, cache=cache)
        right = eval_humn(ops, right, cache=cache)
        if op == "+"
            cache[monkey] = left + right
        elseif op == "-"
            cache[monkey] = left - right
        elseif op == "*"
            cache[monkey] = left * right
        elseif op == "/"
            cache[monkey] = left ÷ right
        end
    end
    cache[monkey]    
end

"Part 1"
function part1()
    ops = get_ops()
    root = eval(ops, "root")
    println("PART 1: $root")
end

"Part 2"
function part2()
    ops = get_ops()
    left, _, right = ops["root"]
    left = eval_humn(ops, left)
    right = eval_humn(ops, right)
    # assumes that final numerator is linear
    n_humn, n = (left - right).num.coeffs
    result = n_humn ÷ (-n)
    println("PART 2: $result")
end

# 20 mins for part 1, 1 hour for part 2
part1()
part2()
