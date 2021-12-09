module day09

const Map = Matrix{Int}

function parsemap(input)::Map
    rows = [
        reshape([parse(Int, c) for c in collect(r)], 1, :) for
        r in split(strip(input), '\n')
    ]
    vcat(rows...)
end

function neighbours(m::Map, x, y)
    D = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    [
        (x + Δx, y + Δy) for (Δx, Δy) = D if
        x + Δx > 0 && y + Δy > 0 && x + Δx <= size(m, 1) && y + Δy <= size(m, 2)
    ]
end

function minima(m::Map)
    rv = []
    for x = 1:size(m, 1), y = 1:size(m, 2)
        if all(m[x, y] .< m[CartesianIndex.(neighbours(m, x, y))])
            push!(rv, (x, y))
        end
    end
    rv
end

function risk(m::Map, coords)
    rv = 0
    for (x, y) in coords
        rv += m[x, y] + 1
    end
    rv
end

function run()
    input = read("$(@__DIR__)/../../input", String)
    m = parsemap(input)
    mins = minima(m)
    println("The answer to the first part is $(risk(m, mins))")
end
end
