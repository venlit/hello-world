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
	
	println("enter amount of nums you want to enter (must be greater than 4)")
	total = parse(UInt32,readline())
	nums = []

	for i = 1:total
		println("enter number (positive or negative intergers): ")
		n = parse(Int32, readline())
		push!(nums, n)
	end
	
	println("all possible target values: ", scan2(nums))

	println("enter target value (positive or negative interger): ")
	target = parse(Int32, readline())

	println(scan(nums, target))

end
main()