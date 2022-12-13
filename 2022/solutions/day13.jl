"Utility function to load the stuff"
function get_pairs()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day13.txt")
    pairs::Vector{Tuple{Expr, Expr}} = []
    open(path) do file
        lines = readlines(file)
        for i = 1:3:length(lines)
            push!(pairs, (Meta.parse(lines[i]), Meta.parse(lines[i + 1])))
        end
    end
    pairs
end

"Check if one value is less than another"
function lt(left::Expr, right::Expr)
    n_left, n_right = length(left.args), length(right.args)
    n = min(n_left, n_right)
    for (l, r) in zip(view(left.args, 1:n), view(right.args, 1:n))
        if lt(l, r)
            return true
        elseif !eq(l, r)
            return false
        end
    end
    n_left < n_right
end

lt(left::Int64, right::Int64) = left < right
lt(left::Expr, right::Int64) = lt(left, Meta.parse("[$right]"))
lt(left::Int64, right::Expr) = lt(Meta.parse("[$left]"), right)

"Check if one value is equal to another"
function eq(left::Expr, right::Expr)
    n_left, n_right = length(left.args), length(right.args)
    if n_left != n_right
        return false
    end
    for (l, r) in zip(left.args, right.args)
        if !eq(l, r)
            return false
        end
    end
    true
end
eq(left::Int64, right::Int64) = left == right
eq(left::Expr, right::Int64) = eq(left, Meta.parse("[$right]"))
eq(left::Int64, right::Expr) = eq(Meta.parse("[$left]"), right)

"Part 1"
function part1()
    total = 0
    for (i, (l, r)) in enumerate(get_pairs())
        total += lt(l, r) ? i : 0
    end
    println("PART 1: $total")
end

"Part 2"
function part2()
    div1 = :([[2]])
    div2 = :([[6]])
    packets = [expr for pair in get_pairs() for expr in pair]
    append!(packets, [div1, div2])
    sort!(packets, lt=lt)
    loc1 = filter(x -> x[2] == div1, [x for x in enumerate(packets)])[1][1]
    loc2 = filter(x -> x[2] == div2, [x for x in enumerate(packets)])[1][1]
    decoder_key = loc1 * loc2
    println("PART 2: $decoder_key")
end

# 45 mins
part1()
part2()