function peak(arr)
	min = 1
	max = length(arr)

	while max >= min
		mid = convert(Int8, round((max + min)/2, digits = 0))

		if mid == 1 || mid == length(arr) 
			return (arr[mid], mid)
		end

		if arr[mid] < arr[mid - 1]
			max = mid-1 
		elseif arr[mid] < arr[mid + 1]
			min = mid+1 
		else
			return (arr[mid], mid)
		end

	end
	return 0
end

function main()
	nums = [1, 3, 5, 6, 7, 9, 10, 12]
	nums2 = [15, 14, 13, 12, 9, 8, 7, 6, 5, 4, 3, 2, 1]

	println(peak(vcat(nums, nums2)))
	println(peak(vcat(reverse(nums2), reverse(nums))))
	println(peak(reverse(nums2, 1, 6)))
end

@time main()
