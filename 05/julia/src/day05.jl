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

function fillmap(m)
    rv = []
    for (i, line) in enumerate(m)
        ((x₁,y₁), (x₂,y₂)) = line
        if x₁ != x₂ && y₁ != y₂
            continue
        end
        if x₁ > x₂
            x₁, x₂ = x₂, x₁
        end
        if y₁ > y₂
            y₁, y₂ = y₂, y₁
        end

        append!(rv, [(x,y) for x = x₁:x₂, y = y₁:y₂])
    end
    rv
end

function countoverlaps(input)
    m = parsemap(input)
    f = fillmap(m)
    count(c->c>1, values(counter(f)))
end

function run()
    input = read("$(@__DIR__)/../../input", String)

    println("The answer to the first part is $(countoverlaps(input))")
end
end
