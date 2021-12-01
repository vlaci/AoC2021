module day01
using IterTools

function read()
    lines = open("$(@__DIR__)/../../input") do f
        readlines(f)
    end
    parse.(Int, lines)
end

function depth(measurements)
    count(p -> p[2] - p[1] > 0, partition(measurements, 2, 1))
end

function run()
    measurements = read()
    println("The answer to the first part is $(depth(measurements))")
end
end
