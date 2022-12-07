mutable struct Node
    name
    parent
    children
    size::Int64
end

function path(node)
    if node == nothing
        return ""
    elseif node.name == "/"
        return node.name
    else
        return path(node.parent) * "/" * node.name
    end
end

function isdir(node::Node)
    node.size == 0
end

function size(node::Node)
    total = node.size
    for child in node.children
        total += size(child)
    end
    total
end

"Utility function to load stuff"
function get_ops()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day7.txt")
    open(path) do file
        return readlines(file)
    end
end

"Calculate the nodes"
function calc_nodes()
    nodes = Dict()
    loc = nothing
    ops = get_ops()
    index = 1
    while index <= length(ops)
        op = ops[index]

        if startswith(op, "\$ cd")
            if op[end] == '/'
                loc = Node("/", nothing, [], 0)
                nodes[path(loc)] = loc
            elseif endswith(op, "..")
                loc = nodes[path(loc.parent)]
            else
                name = split(op, " ")[end]
                for child in loc.children
                    if child.name == name
                        loc = child
                        break
                    end
                end
            end
            index += 1

        elseif startswith(op, "\$ ls")
            index += 1
            while index <= length(ops)
                f = ops[index]
                if startswith(f, "\$")
                    break
                end

                size_, name = split(f, " ")
                size = (size_ == "dir") ? 0 : parse(Int64, size_)
                node = Node(name, loc, [], size)
                push!(loc.children, node)
                nodes[path(node)] = node
                index += 1
            end
        end
    end
    nodes
end

"Part 1"
function part1()
    nodes = calc_nodes()
    total = 0
    for (_, node) in nodes
        if !isdir(node)
            continue
        end
        s = size(node)
        if s <= 100000
            total += s
        end
    end
    println("PART 1: $total")
end

"Part 2"
function part2()
    nodes = calc_nodes()
    required = 30000000 - (70000000 - size(nodes["/"]))
    smallest = 100000000
    for (_, node) in nodes
        if !isdir(node)
            continue
        end
        s = size(node)
        if (s >= required) && (s < smallest)
            smallest = s
        end
    end
    println("PART 2: $smallest")
end

part1()
part2()