using day07
using Test

@testset "The Treachery of Whales" begin
    @test day07.adjustposition("16,1,2,0,4,2,7,1,2,14") == 37
end
