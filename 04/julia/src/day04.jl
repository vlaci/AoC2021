module day04
using Lerche

grammar = """
  start: draws board+
  draws: number ("," number)* LF
  board: LF row+
  row: " "? number (" "+ number)* LF
  number: INT

  %import common.LF
  %import common.INT
"""

struct GameParser <: Transformer end

@rule start(s::GameParser, nums) = nums[1], convert(Vector{Matrix{Int}}, nums[2:end])
@rule draws(s::GameParser, nums) = convert(Vector{Int}, nums[1:end-1])
@rule board(s::GameParser, nums) = [nums[i][j] for i = 2:length(nums), j = 1:length(nums)-1]
@rule row(s::GameParser, nums) = convert(Vector{Int}, nums[1:end-1])
@rule number(s::GameParser, nums) = parse(Int, nums[1])

mutable struct Board
    board::Matrix{Union{Int,Nothing}}
    lookup::Dict{Int,Tuple{Int,Int}}

    function Board(board)
        s = size(board)[1]
        lookup = Dict()
        for i = 1:s, j = 1:s
            c = board[i, j]
            lookup[c] = (i, j)
        end
        new(board, lookup)
    end
end


mutable struct Bingo
    draws::Vector{Int}
    boards::Vector{Board}

    function Bingo(input)
        (draws, boards) = parsegame(input)
        new(draws, [Board(b) for b in boards])
    end
end

function play!(bingo::Bingo)
    for d in bingo.draws
        draw!(bingo, d)
        wb = winningboard(bingo)
        if !isnothing(wb)
            return sum(filter(!isnothing, wb.board)) * d
        end
    end
end

function playtolast!(bingo::Bingo)
    rv = missing
    while hasboardleft(bingo)
        rv = play!(bingo)
    end
    rv
end

function draw!(board::Board, number::Int)
    i = get(board.lookup, number, missing)
    if !ismissing(i)
        board.board[CartesianIndex(i)] = nothing
    end
end

function draw!(bingo::Bingo, number::Int)
    for board in bingo.boards
        draw!(board, number)
    end
end

function winning(board::Board)
    any(map(r -> all(isnothing, r), eachrow(board.board))) ||
        any(map(c -> all(isnothing, c), eachcol(board.board)))
end

function winningboard(bingo::Bingo)
    board = findfirst(winning, bingo.boards)
    if isnothing(board)
        return nothing
    end
    popat!(bingo.boards, board)
end

function hasboardleft(bingo::Bingo)
    !isempty(bingo.boards)
end

parser = Lark(grammar, parser = "lalr", transformer = GameParser())

function parsegame(input)
    Lerche.parse(parser, input)
end


function run()
    input = read("$(@__DIR__)/../../input", String)
    bingo = Bingo(input)

    println("The answer to the first part is $(play!(bingo))")
    println("The answer to the second part is $(playtolast!(bingo))")
end

end
