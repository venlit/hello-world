function move(c, size)
	if c['a'] < size - 3 && c['b'] == size - 2
		c['a'] += 1
		c['b'] = c['a'] + 1
		c['c'] = c['b'] + 1
		c['d'] = c['c'] + 1
	end
	if c['b'] < size - 2 && c['c'] == size - 1
		c['b'] += 1
		c['c'] = c['b'] + 1
		c['d'] = c['c'] + 1
	end
	if c['c'] < size - 1 && c['d'] == size
		c['c'] += 1
		c['d'] = c['c'] + 1
	end
	if c['d'] < size
		c['d'] += 1
	end

end

function presence(arr, x)
	absence = true
	for i in arr
		if issetequal(i, x) 
			absence = false
			break
		end
	end
	return absence
end	

function scan(nums, target)
	c = Dict('a' => 1, 'b' => 2, 'c' => 3, 'd' => 4)
	arr = []
	while c['a'] < length(nums) - 3

		if nums[c['a']] + nums[c['b']] + nums[c['c']] + nums[c['d']] == target
			x = collect(Int32, (nums[c['a']], nums[c['b']], nums[c['c']], nums[c['d']]))
			presence(arr, x) && push!(arr, x)
		end

		move(c, length(nums))
	end

	return arr
end

function scan2(nums)
	c = Dict('a' => 1, 'b' => 2, 'c' => 3, 'd' => 4)
	arr = []

	while c['a'] < length(nums) - 3
		if !(nums[c['a']] + nums[c['b']] + nums[c['c']] + nums[c['d']] in arr)
			push!(arr, nums[c['a']] + nums[c['b']] + nums[c['c']] + nums[c['d']])
		end
		move(c, length(nums))
	end

	return arr

end

function main()
	
	nums = []
	
	println("Enter all the numbers you want to use, separated by a comma. \nThere has to be more than 4:")
	str = readline()
	numbers = collect(split(str, ","))

	for i in numbers
		push!(nums, parse(Int32, i))
	end

	println("all possible target values: ", scan2(nums))

	println("enter target value (positive or negative interger): ")
	target = parse(Int32, readline())

	println(scan(nums, target))

end
main()
