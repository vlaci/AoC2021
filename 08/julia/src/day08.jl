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
    patterns::Vector{Set{Char}}
    output::Vector{Set{Char}}
end

@rule all(s::Notes, tokens) = tokens
@rule display(s::Notes, tokens) = Display(tokens[1], tokens[2])
@rule segment(s::Notes, tokens) = Set(tokens[1])

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

function untangledisplay(patterns)
    #   aaaa
    #  b    c
    #  b    c
    #   dddd
    #  e    f
    #  e    f
    #   gggg

    one, = filter(p -> length(p) == 2, patterns)   #  cf
    seven, = filter(p -> length(p) == 3, patterns) # acf
    four, = filter(p -> length(p) == 4, patterns) # bcdf
    eight, = filter(p -> length(p) == 7, patterns) # abcdefg


    zero_six_nine = filter(p -> length(p) == 6, patterns)

    nine, = filter(x -> four ⊆ x, zero_six_nine)

    six, = filter(x -> one ⊈ x, zero_six_nine)

    zero, = filter(x -> x != six && x != nine, zero_six_nine)

    two_three_five = filter(p -> length(p) == 5, patterns)

    three, = filter(x -> one ⊆ x, two_three_five)

    five, = filter(x -> x ⊆ six, two_three_five)

    two, = filter(x -> x != three && x != five, two_three_five)

    Dict(
        zero => 0,
        one => 1,
        two => 2,
        three => 3,
        four => 4,
        five => 5,
        six => 6,
        seven => 7,
        eight => 8,
        nine => 9,
    )
end


function decode(d::Display)
    m = untangledisplay(d.patterns)
    sum(m[o] * 10^(length(d.output) - i) for (i, o) in enumerate(d.output))
end

function decodesum(d::Displays)
    sum(decode(d) for d in d.displays)
end

function run()
    input = read("$(@__DIR__)/../../input", String)
    ds = Displays(input)
    println("The answer to the first part is $(countoutputs(ds))")
    println("The answer to the second part is $(decodesum(ds))")
end
end
