module day02
using Lerche

grammar = """
  start: line*           -> sum
  line: command NEWLINE  -> cmd
  command:
    | "down " amount     -> dive
    | "up " amount       -> ascent
    | "forward " amount  -> forward
  amount: NUMBER         -> number

  %import common.NUMBER
  %import common.NEWLINE
"""

parser = Lark(grammar, parser = "lalr")

mutable struct Direct <: Transformer end
mutable struct Aimed <: Transformer
    aim::Int
    function Aimed()
        new(0)
    end
end
Course = Union{Direct,Aimed}

@rule sum(s::Course, cmds) = reduce(.+, cmds) |> prod
@rule cmd(s::Course, cmd) = cmd[1]
@rule dive(s::Direct, amount) = (amount[1], 0)
@rule ascent(s::Direct, amount) = (-amount[1], 0)
@rule forward(s::Direct, amount) = (0, amount[1])
@rule number(s::Course, nums) = parse(Int, nums[1])


@rule dive(s::Aimed, amount) = begin
    s.aim += amount[1]
    (0, 0)
end
@rule ascent(s::Aimed, amount) = begin
    s.aim -= amount[1]
    (0, 0)
end
@rule forward(s::Aimed, amount) = (s.aim * amount[1], amount[1])

function dive(course, transformer)
    p = Lerche.parse(parser, course)
    Lerche.transform(transformer(), p)
end

function run()
    course = read("$(@__DIR__)/../../input", String)
    println("The answer to the first part is $(dive(course, Direct))")
    println("The answer to the second part is $(dive(course, Aimed))")
end
end
