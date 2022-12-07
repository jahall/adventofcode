using DataStructures

"Node representing a directory or file"
mutable struct Node
    name::String
    parent::Union{Node, Nothing}
    size::Int64
    children::Vector{Node}

    Node(name) = new(name, nothing, 0, [])
    Node(name, parent) = new(name, parent, 0, [])
    Node(name, parent, size) = new(name, parent, size, [])
end

"Is this node a directory?"
function isdir(node::Node)
    node.size == 0
end

"Traverse all nodes"
function traverse(node::Node)
    nodes = [node]
    for child in node.children
        append!(nodes, traverse(child))
    end
    nodes
end

"Total size of the node"
function size(node::Node)
    total = node.size
    for child in node.children
        total += size(child)
    end
    total
end

"Utility function to load the op queue"
function get_ops()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day7.txt")
    q = Queue{String}()
    open(path) do file
        for op in readlines(file)
            enqueue!(q, op)
        end
    end
    q
end

"Construct the tree"
function construct_tree()
    ops = get_ops()
    root = Node("/")
    cwd = root
    op = dequeue!(ops)

    while length(ops) > 0

        # Deal with directory changes
        if startswith(op, "\$ cd")
            if op[end] == '/'
                cwd = root
            elseif endswith(op, "..")
                cwd = cwd.parent
            else
                name = split(op, " ")[end]
                for child in cwd.children
                    if child.name == name
                        cwd = child
                        break
                    end
                end
            end
            op = dequeue!(ops)

        # Deal with list dirs
        elseif startswith(op, "\$ ls")
            while length(ops) > 0
                item = dequeue!(ops)
                if startswith(item, "\$")
                    op = item
                    break
                end

                size_, name = split(item, " ")
                size = (size_ == "dir") ? 0 : parse(Int64, size_)
                node = Node(name, cwd, size)
                push!(cwd.children, node)
            end
        end
    end
    root
end

"Part 1"
function part1()
    root = construct_tree()
    total = 0
    for node in traverse(root)
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
    root = construct_tree()
    required = 30000000 - (70000000 - size(root))
    smallest = 100000000
    for node in traverse(root)
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