using day02
using Test

@testset "Dive" begin
    @test day02.dive("""
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    """, day02.Direct) == 150

    @test day02.dive("""
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
    """, day02.Aimed) == 900
end
