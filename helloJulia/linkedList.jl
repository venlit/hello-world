mutable struct Node
    Data
    Next
  
    function Node(Data, Next)
        if Data == nothing
            new(Data)
        end
        new(Data, Next)
    end
end

struct List
    Head::Node
    Body::Vector{Node}
    arr::Vector
    function List()
        arr = []
        Head = Node(nothing, nothing)
        new(arr)
    end

    function List(arr)
        x = 1
        Body = []
        if length(arr) == 1
            Head = Node(arr[1], nothing)
            new(Head)
        end
            
        Head = Node(arr[1], 2)

        for i = 3:2:length(arr) 
            i + 1 > length(arr) ? t = nothing : t = i + 1
            n = Node(arr[i], t) 
            push!(Body, n)

        end
        length(arr) % 2 == 0 && push!(Body, Node(nothing, nothing)) 
        new(Head, Body)


    end
        
end

struct Rational
    n::Int32
    d::Int32
    function Rational()
        n = 0
        new(n)
    end
    function Rational(n)
        d = 1
        new(n)
    end
    function Rational(n, d)
        if n == d
            n = 1
            d = 1
            new(n)
        end
        if n == 0
            new(n)
        end
        if d == 0
            error("can't divide by 0")
        end
        abs(d) > abs(n) ? z = abs(n) : z = abs(d)
        for i = z:-1:1
            if d % i == 0 && n % i == 0
                n /= i
                d /= i
                break
            end
            
        end        

        if n <0 && d > 0
            new(n, d)
        elseif d < 0 && n > 0 || d < 0 && n < 0
            new(-n, -d)
        else
            new(n,d)
        end
    end
end


function isEmpty(l)
    if l.Head == nothing
        return true
    end
    return false
end

function main()
    x = Rational(-10, -10)
    println(x)
end
main()