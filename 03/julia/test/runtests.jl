using day03
using Test

@testset "Binary Diagnostics" begin
    @test day03.bindiag("""
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
        """) ==
        (22, 9)
end
