using Statistics
using Printf

function isPrime(num)
	for i in 2:convert(Int64, round(num/2, digits = 0))
		if num % i == 0
			return false
		end
	end
	return true
end

function primes(num)
	counter::Int8 = 1

	for i in 3:2:num
		isPrime(i) ? counter += 1 : continue
	end
	return counter
end

function main()
	x = 100
	println(primes(x))
end

@time main()
