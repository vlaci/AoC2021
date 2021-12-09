using day09
using Test

@testset "Smoke Basin" begin
    m = day09.parsemap("""
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
        """)
    @test day09.neighbours(m, 1, 1) == [(2, 1), (1, 2)]
    @test day09.neighbours(m, 2, 2) == [(1, 2), (3, 2), (2, 1), (2, 3)]
    mins = day09.minima(m)
    @test mins == [(1, 2), (1, 10), (3, 3), (5, 7)]
    @test day09.risk(m, mins) == 15

    @test day09.basin(m, 1, 2) == Set([(1, 2), (1, 1), (2, 1)])
    @test day09.basin(m, 1, 10) ==
          Set([(1, 10), (2, 10), (1, 9), (3, 10), (2, 9), (1, 8), (1, 7), (2, 7), (1, 6)])
    @test day09.basins(m, mins) == 1134
end
