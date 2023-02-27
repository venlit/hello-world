function main()
	nums = [0, 12,1, 4,5,3,2,4,2,16,3,2]
	population = Dict{Int32, Int32}()
	for i in nums
		haskey(population, i) ? population[i] += 1 : merge!(population, Dict(i => 1))	
	end
	println(findmax(population)[2])
end
main()
