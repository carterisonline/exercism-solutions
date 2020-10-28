function score(x, y)
    distance = √(x^2 + y^2)
    if distance ≤ 1
        return 10
    elseif distance ≤ 5
        return 5
    elseif distance ≤ 10
        return 1
    else
        return 0
    end
end