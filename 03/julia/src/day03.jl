module day03

function Report(input)
    lines = split(input |> strip, "\n")
    d = [[c == '1' for c in l] for l in lines]
    hcat(d...)
end

function bindiag(report)
    threshold = size(report, 2) / 2

    frequencies = map(eachrow(report)) do entry
        sum(entry) > threshold
    end

    bitstoint(frequencies) * bitstoint(.!frequencies)
end

function bitstoint(bits)
    rv = 0
    for (e, d) in bits |> reverse |> enumerate
        if d
            rv += 2^(e - 1)
        end
    end
    rv
end

function sensorrating(pred::Function, report)
    sieve = fill(true, size(report, 2))
    for row in eachrow(report)
        if pred(row .& sieve, sum(sieve))
            sieve .&= row
        else
            sieve .&= .!row
        end

        if sum(sieve) == 1
            break
        end
    end

    report[:, findfirst(sieve)] |> bitstoint
end

function lifesupportrating(report)
    o2 = sensorrating(report) do bits, remaining
        sum(bits) >= remaining / 2
    end
    co2 = sensorrating(report) do bits, remaining
        sum(bits) < remaining / 2
    end

    o2 * co2
end

function run()
    report = read("$(@__DIR__)/../../input", String) |> Report
    println("The answer to the first part is $(bindiag(report))")
    println("The answer to the second part is $(lifesupportrating(report))")
end
end
