module day06
using DataStructures

mutable struct Fishes
    timetobirth::Int8
    count::Int
end

function countfish(input, days)
    parsed = parse.(Int8, split(input |> strip, ","))
    fishes = map((p)->Fishes(p.first, p.second), counter(parsed) |> collect)

    for d = 1:days
        births = 0
        for f = fishes
            if f.timetobirth == 0
                f.timetobirth = 6
                births += f.count
            else
                f.timetobirth -= 1
            end
        end
        if births > 0
            push!(fishes, Fishes(8, births))
        end

    end
    map(f->f.count, fishes) |> sum
end


function run()
    input = read("$(@__DIR__)/../../input", String)

    println("The answer to the first part is $(countfish(input, 80))")
    println("The answer to the first part is $(countfish(input, 256))")
end

end
