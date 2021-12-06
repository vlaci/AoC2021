using day06
using Test

@testset "Lanternfish" begin
    @test day06.countfish("3,4,3,1,2", 18) == 26
    @test day06.countfish("3,4,3,1,2", 80) == 5934
end
