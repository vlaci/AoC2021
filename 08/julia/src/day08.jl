module day08

using Lerche

grammar = """
  start: display*  -> all
  display: pattern output NEWLINE
  pattern: segment+ "| "  -> all
  output: segment+        -> all
  segment: SEGMENT WS_INLINE?
  SEGMENT: /[abcdefg]+/
  %import common.NEWLINE
  %import common.WS_INLINE
"""

struct Notes <: Transformer end

struct Display
    patterns::Vector{String}
    output::Vector{String}
end

@rule all(s::Notes, tokens) = tokens
@rule display(s::Notes, tokens) = Display(tokens[1], tokens[2])
@rule segment(s::Notes, tokens) = convert(String, tokens[1])

parser = Lark(grammar, parser = "lalr", transformer = Notes())

struct Displays
    displays::Vector{Display}
    function Displays(input)
        new(Lerche.parse(parser, input))
    end
end

function outputs(ds::Displays)
    map(d -> d.output, ds.displays) |> Iterators.flatten |> collect
end

function countoutputs(ds::Displays)
    count(l -> l in [2, 3, 4, 7], length.(outputs(ds)))
end

function run()
    input = read("$(@__DIR__)/../../input", String)
    ds = Displays(input)
    println("The answer to the first part is $(countoutputs(ds))")
end
end
