
"Utility function to load the stuff"
function get_sensor_beacon_pairs()
    root = dirname(dirname(@__FILE__))
    path = joinpath(root, "data", "day15.txt")
    pairs = []
    open(path) do file
        for line in readlines(file)
            parts = split(line, " ")
            sensor = (
                parse(Int64, view(parts[3], 3:length(parts[3]) - 1)),
                parse(Int64, view(parts[4], 3:length(parts[4]) - 1)),
            )
            beacon = (
                parse(Int64, view(parts[9], 3:length(parts[9]) - 1)),
                parse(Int64, view(parts[10], 3:length(parts[10]))),
            )
            push!(pairs, (sensor, beacon))
        end
    end
    pairs
end

"Manhatten distance between two points"
function manhatten(c1::Tuple{Int64, Int64}, c2::Tuple{Int64, Int64})
    abs(c1[1] - c2[1]) + abs(c1[2] - c2[2])
end

"Collapse a bunch of segments"
function collapse(segments::Vector{Tuple{Int64, Int64}})
    segments = sort(segments)
    collapsed = [segments[1]]
    for (left, right) in view(segments, 2:length(segments))
        prev_left, prev_right = collapsed[end]
        if left > prev_right + 1
            # start a new segment
            push!(collapsed, (left, right))
        else
            # append to prev segment
            collapsed[end] = (prev_left, max(prev_right, right))
        end
    end
    collapsed
end

"Find the covered x coordinates"
function find_covered(pairs, row)
    ranges::Vector{Tuple{Int64, Int64}} = []
    for (sensor, beacon) in pairs
        (x, y) = sensor
        dist = manhatten(sensor, beacon)
        offset = abs(y - row)
        if offset <= dist
            i = dist - offset
            # (x, x) at furthest reach and (x - dist, x + dist) when on same row
            push!(ranges, (x - i, x + i))
        end
    end
    collapse(ranges)
end

"Find location"
function find_loc(range::Tuple{Int64, Int64}, segments::Vector{Tuple{Int64, Int64}})
    for (_, r) in segments
        if r >= range[1] && r <= range[2]
            return r + 1
        end
    end
    -1
end

"Part 1"
function part1()
    pairs = get_sensor_beacon_pairs()
    # row 10 is used for the test, and 2M for the real thing
    row = (length(pairs) == 14) ? 10 : 2_000_000
    covered = find_covered(pairs, row)
    total = 0
    for (left, right) in covered
        total += right - left + 1
    end
    beacons = Set([beacon for (_, beacon) in pairs])
    total -= length(filter(b -> b[2] == row, beacons))
    println("PART 1: $total")
end

"Part 2"
function part2()
    pairs = get_sensor_beacon_pairs()
    # row 20 is used for the test, and 4M for the real thing
    size = (length(pairs) == 14) ? 20 : 4_000_000
    for y = 0:size
        covered = find_covered(pairs, y)
        x = find_loc((0, size), covered)
        if x > -1
            tuning_freq = 4_000_000 * x + y
            println("PART 2: $tuning_freq")
            return
        end
    end
end

# 1 hour 10 mins
part1()
part2()
