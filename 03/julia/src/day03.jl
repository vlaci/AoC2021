module day03

function bindiag(report)
    entries = split(strip(report), "\n")
    frequencies = fill(0, length(entries[1]))

    for e in entries
        for (i, c) in enumerate(e)
            if c == '1'
                frequencies[i] += 1
            end
        end
    end
    fₘₐₓ = 0
    fₘᵢₙ = 0

    for (e,d) in frequencies |> reverse |> enumerate
        delta = 2^(e-1)
        if d > length(entries) / 2
            fₘₐₓ += delta
        else
            fₘᵢₙ += delta
        end
    end

    (fₘₐₓ, fₘᵢₙ)
end

function run()
    report = read("$(@__DIR__)/../../input", String)
    println("The answer to the first part is $(bindiag(report) |> prod)")
end
end
