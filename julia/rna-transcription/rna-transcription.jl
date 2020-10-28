function to_rna(dna)
    transcription = Dict('C' => 'G', 'G' => 'C', 'T' => 'A', 'A' => 'U')
    map((c)
        ->  try
                transcription[c]
            catch KeyError
                throw(ErrorException("$c is not a valid DNA value"))
            end,
        dna
    )
end