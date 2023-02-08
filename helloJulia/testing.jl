function main()
	x = 0
	for i in 1:10^8
		x += 1
	end
end
@time main()
