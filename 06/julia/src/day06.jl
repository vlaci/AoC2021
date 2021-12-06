module day06

function countfish(input, days)
    fishes = parse.(Int, split(input |> strip, ","))

    for d = 1:days
        @show d
        births = 0
        for (i,f) = enumerate(fishes)
            if f == 0
                fishes[i] = 6
                births += 1
            else
                fishes[i] = f - 1
            end
        end
        resize!(fishes, length(fishes) + births)
        for i = (length(fishes)-births+1):length(fishes)
            fishes[i] = 8
        end
    end
    length(fishes)
end


function run()
    input = read("$(@__DIR__)/../../input", String)

    println("The answer to the first part is $(countfish(input, 80))")
end

end
