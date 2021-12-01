using day01
using Test

@testset "Depth" begin
    @test day01.depth([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]) == 7
end
