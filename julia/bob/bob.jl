function bob(stimulus::String)
    z = 0
    if endswith(rstrip(stimulus), '?')
        z = 1
    elseif isempty(filter(c -> !isspace(c), stimulus))
        return "Fine. Be that way!"
    end
    if uppercase(stimulus) == stimulus && !isempty(filter(c -> isletter(c), stimulus))
        if z == 1
            return "Calm down, I know what I'm doing!"
        else
            return "Whoa, chill out!"
        end
    elseif z == 1
        return "Sure."
    end
    "Whatever."
end
