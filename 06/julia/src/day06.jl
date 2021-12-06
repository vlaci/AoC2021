module day06

function countfish(input, days)
    parsed = parse.(Int8, split(input |> strip, ","))
    fishes = fill(0, 9)
    for t in parsed
        fishes[t+1] += 1
    end

    for d = 1:days
        births = fishes[1]
        for t = 1:8
            fishes[t] = fishes[t+1]
        end
        fishes[9] = births
        fishes[7] += births
    end
    sum(fishes)
end


function run()
    input = read("$(@__DIR__)/../../input", String)

    println("The answer to the first part is $(countfish(input, 80))")
    println("The answer to the first part is $(countfish(input, 256))")
end

end
