module day07
using Statistics

function adjustposition(input)
    pos = parse.(Int, split(input |> strip, ','))
    final = round(Int, median(pos))
    abs.(pos .- final) |> sum
end

function adjustposition2(input)
    pos = parse.(Int, split(input |> strip, ','))
    mn, mx = extrema(pos)
    map(mn:mx) do c
        fuelcost.(abs.(pos .- c)) |> sum
    end |> minimum
end

function fuelcost(d)
    cost = 0
    for c = 1:d
        cost += c
    end
    cost
end

function run()
    input = read("$(@__DIR__)/../../input", String)
    println("The answer to the first part is $(adjustposition(input))")
    println("The answer to the second part is $(adjustposition2(input))")
end

end
