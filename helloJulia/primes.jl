function primes(num)
	counter::Int32 = 1

	for i in 3:2:num
		y = true

		for j in 3:2:convert(Int32, round(i/2, digits = 0))
			if i % j == 0
				y = false
				break
			end
		end
		y ? counter += 1 : continue

	end

	return counter
end

function main()
	x = 1000000
	println(@time primes(x))
end
main()
