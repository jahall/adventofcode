using DataStructures

"A point in 3d space"
struct Point
    x::Int64
    y::Int64
    z::Int64
end

"A bounding box"
struct Bbox
    xmin::Int64
    xmax::Int64
    ymin::Int64
    ymax::Int64
    zmin::Int64
    zmax::Int64

    Bbox(points::Vector{Point}) = new(
        min([p.x for p in points]...),
        max([p.x for p in points]...),
        min([p.y for p in points]...),
        max([p.y for p in points]...),
        min([p.z for p in points]...),
        max([p.z for p in points]...),
    )
end

"Is a point outside this bounding box?"
function outside(point::Point, bbox::Bbox)
    (
        (point.x < bbox.xmin) ||
        (point.x > bbox.xmax) ||
        (point.y < bbox.ymin) ||
        (point.y > bbox.ymax) ||
        (point.z < bbox.zmin) ||
        (point.z > bbox.zmax)
    )
end

"Find all immediate neighbors of a point in space"
function neighbors(point::Point)
    [
        Point(
            point.x + x,
            point.y + y,
            point.z + z,
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
            Point([
                parse(Int64, part) for part in split(line, ",")
            ]...)
            for line in readlines(file)
        ]
    end
end

"Find all exposed faces"
function calc_exposed_faces(droplets)
    droplet_set = Set(droplets)
    exposed = 0
    for droplet in droplets
        for nbr in neighbors(droplet)
            if !in(nbr, droplet_set)
                exposed += 1
            end
        end
    end
    exposed
end

"Find all hidden faces"
function calc_hidden_faces(droplets)
    # 1. map coordinates to num of faces they cover
    covered = find_covered(droplets)
    # 2. get bounding box
    bbox = Bbox(droplets)
    # 3. find exposed vs hidden
    droplet_set = Set(droplets)
    exposed::Set{Point} = Set()
    hidden::Set{Point} = Set()
    num_hidden = 0
    for (point, _) in covered
        if !in(point, exposed) && !in(point, hidden)
            investigate!(bbox, droplet_set, exposed, hidden, point)
        end
        if in(point, hidden)
            num_hidden += covered[point]
        end
    end
    num_hidden
end

"Get a mapping from point in space to number of faces it covers"
function find_covered(droplets)
    droplet_set = Set(droplets)
    covered = Dict{Point, Int64}()
    for droplet in droplets
        for nbr in neighbors(droplet)
            if !in(nbr, droplet_set)
                if haskey(covered, nbr)
                    covered[nbr] += 1
                else
                    covered[nbr] = 1
                end
            end
        end
    end
    covered
end

"Investigate a single point by expanding out the search radius until we're sure its exposed or hidden"
function investigate!(
    bbox::Bbox,
    droplets::Set{Point},
    exposed::Set{Point},
    hidden::Set{Point},
    point::Point,
)
    q = Queue{Point}()
    investigated = Set{Point}([point])
    enqueue!(q, point)
    while length(q) > 0
        p = dequeue!(q)
        # if any point is in exposed or is outside the bbox, then all investigated are exposed
        if in(p, exposed) || outside(p, bbox)
            union!(exposed, investigated)
            return
        end
        # now add all neighbors to the queu
        for nbr in neighbors(p)
            if !in(nbr, droplets) && !in(nbr, investigated)
                enqueue!(q, nbr)
                union!(investigated, Set([nbr]))
            end
        end
    end
    # if we exhausted all neighbors, then all these points are hidden
    union!(hidden, investigated)
    return
end

"Part 1"
function part1()
    droplets = get_droplets()
    exposed = calc_exposed_faces(droplets)
    println("PART 1: $exposed")
end

"Part 2"
function part2()
    droplets = get_droplets()
    exposed = calc_exposed_faces(droplets) - calc_hidden_faces(droplets)
    println("PART 2: $exposed")
end

# 20 mins for part 1, 1 hour for part 2 (plus lots of pre-thinking)
part1()
part2()
