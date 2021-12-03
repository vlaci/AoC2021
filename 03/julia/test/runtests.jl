using day03
using Test

@testset "Binary Diagnostics" begin
    report = day03.Report("""
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
        """)

    @test day03.bindiag(report) == 22 * 9
    @test day03.lifesupportrating(report) == 23 * 10
end
