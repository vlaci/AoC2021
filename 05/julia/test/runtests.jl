using day05
using Test

@testset "Hydrothermal Venture" begin
    @test day05.fillline((1, 1), (1, 3), false) |> Set == [(1, 1), (1, 2), (1, 3)] |> Set
    @test day05.fillline((9, 7), (7, 7), false) |> Set == [(7, 7), (8, 7), (9, 7)] |> Set

    @test day05.fillline((1, 1), (3, 3), false) |> isempty
    @test day05.fillline((9, 7), (7, 9), false) |> isempty

    @test day05.fillline((1, 1), (3, 3), true) |> Set == [(1, 1), (2, 2), (3, 3)] |> Set
    @test day05.fillline((9, 7), (7, 9), true) |> Set == [(7, 9), (8, 8), (9, 7)] |> Set

    @test day05.countoverlaps(
        """
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        """,
        false,
    ) == 5
    @test day05.countoverlaps(
        """
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        """,
        true,
    ) == 12
end
