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

struct Course <: Transformer end

parser = Lark(grammar, parser="lalr", transformer=Course())

@rule sum(s::Course, cmds) = reduce(.+, cmds)
@rule cmd(s::Course, cmd) = cmd[1]
@rule dive(s::Course, amount) = (amount[1], 0)
@rule ascent(s::Course, amount) = (-amount[1], 0)
@rule forward(s::Course, amount) = (0, amount[1])
@rule number(s::Course, nums) = parse(Int, nums[1])

function dive(course)
    Lerche.parse(parser, course) |> prod
end

function run()
    course = read("$(@__DIR__)/../../input", String)
    println("The answer to the first part is $(dive(course))")
end
end
