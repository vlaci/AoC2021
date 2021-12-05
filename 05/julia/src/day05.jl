module day05
using DataStructures
using Lerche

grammar = """
  start: line*
  line: coordinate " -> " coordinate NEWLINE
  coordinate: int "," int
  int: INT

  %import common.INT
  %import common.NEWLINE
"""


mutable struct Lines <: Transformer end
parser = Lark(grammar, parser = "lalr", transformer = Lines())

@rule start(s::Lines, tokens) = collect(tokens)
@rule line(s::Lines, tokens) = (tokens[1], tokens[2])
@rule coordinate(s::Lines, tokens) = (tokens[1], tokens[2])
@rule int(s::Lines, tokens) = parse(Int, tokens[1])


function parsemap(input)
    Lerche.parse(parser, input)
end

function fillmap(m, includediag)
    rv = []
    for (i, line) in enumerate(m)
        ((x₁, y₁), (x₂, y₂)) = line
        append!(rv, fillline(line..., includediag))
    end
    rv
end

function fillline(b, e, includediag)
    (x₁, y₁), (x₂, y₂) = b, e

    Δx = sign(x₂ - x₁)
    Δy = sign(y₂ - y₁)
    if !includediag && Δx != 0 && Δy != 0
        return []
    end

    [(x₁ + Δx * i, y₁ + Δy * i) for i = 0:max(abs(x₂ - x₁), abs(y₂ - y₁))]
end

function countoverlaps(input, includediag)
    m = parsemap(input)
    f = fillmap(m, includediag)
    count(c -> c > 1, values(counter(f)))
end

function run()
    input = read("$(@__DIR__)/../../input", String)

    println("The answer to the first part is $(countoverlaps(input, false))")
    println("The answer to the first part is $(countoverlaps(input, true))")
end
end
