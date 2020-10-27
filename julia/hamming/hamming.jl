"Your optional docstring here"
function distance(a, b)
    if length(a) != length(b)
        throw(ArgumentError("$a and $b are not the same length and cannot be indexed."))
    end

    s = length(a)
    for (i, c) in enumerate(a)
        if a[i] == b[i]
            s -= 1
        end
    end
    s
end
