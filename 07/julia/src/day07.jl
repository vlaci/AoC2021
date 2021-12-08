module day07
using Statistics

function adjustposition(input)
    pos = parse.(Int, split(input |> strip, ','))
    final = round(Int, median(pos))
    abs.(pos .- final) |> sum
end

function run()
    input = read("$(@__DIR__)/../../input", String)
    println("The answer to the first part is $(adjustposition(input))")
end

end
