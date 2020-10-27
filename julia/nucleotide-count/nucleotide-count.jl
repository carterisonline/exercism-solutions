"""
    count_nucleotides(strand)

The frequency of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
    strandcount = Dict('A' => 0, 'C' => 0, 'G' => 0, 'T' => 0)
    for c in strand
        try
            strandcount[c] += 1;
        catch KeyError
            throw(DomainError("$c is not a nucleotide"))
        end
    end
    println(strandcount)
    strandcount
end
