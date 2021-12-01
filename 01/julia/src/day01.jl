module day01
using IterTools

function read()
    lines = open("$(@__DIR__)/../../input") do f
        readlines(f)
    end
    parse.(Int, lines)
end

function depth(measurements)
    count(p -> p[2] > p[1], partition(measurements, 2, 1))
end

function depth(measurements, windowsize)
    sums₁ = map(sum, partition(measurements, windowsize, 1))
    sums₂ = map(sum, partition(measurements[2:end], windowsize, 1))
    count(p -> p[2] > p[1], zip(sums₁, sums₂))
end

function run()
    measurements = read()
    println("The answer to the first part is $(depth(measurements))")
    println("The answer to the first part is $(depth(measurements, 3))")
end
end
