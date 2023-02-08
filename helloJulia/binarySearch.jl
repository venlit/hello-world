function binarySearch(arr, target)
	min = 1
	max = length(arr)

	while max >= min
		mid = convert(UInt64, round((max + min) / 2, digits = 0))

		if target > arr[mid]
			min = mid + 1
		elseif target < arr[mid]
			max = mid
		else
			return (arr[mid], convert(Int64, mid))
		end

	end

	return 0
end

function main()
	x = sort([-1, 5, 2, 3, 119,7, 290,1, 8,9,0,-10,-16,-43,-96,-43,-63,-7,-10005,-5,-3,-2,-5,-4,16,24,32,19,143,169,264,501,999,225,76,35,75,28,54])
	println(binarySearch(x, 999))
	# println(binarySearch(sort(x), -10005))

end

@time main()
