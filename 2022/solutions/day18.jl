"A point in 3d space"
struct Droplet
    x::Int64
    y::Int64
    z::Int64
end

function neighbors(droplet::Droplet)
    [
        Droplet(
            droplet.x + x,
            droplet.y + y,
            droplet.z + z,
        )
        for (x, y, z) in [
            (1, 0, 0),  # right face
            (-1, 0, 0), # left face
            (0, 1, 0),  # back face
            (0, -1, 0), # front face
            (0, 0, 1),  # top face
            (0, 0, -1), # bottom face
        ]
    ]
end

"Utility function to load the lava droplets"
function get_droplets()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day18.txt")
    open(path) do file
        return [
            Droplet([
                parse(Int64, part) for part in split(line, ",")
            ]...)
            for line in readlines(file)
        ]
    end
end

"Part 1"
function part1()
    droplets = Set(get_droplets())
    exposed = 0
    for droplet in droplets
        for nbr in neighbors(droplet)
            if !in(nbr, droplets)
                exposed += 1
            end
        end
    end
    println("PART 1: $exposed")
end

"Part 2"
function part2()
    println("PART 2:")
end

# 20 mins for part 1
part1()
part2()
