function populate()
	# n:UInt32 = readline()
	n = 3
	book = Dict{String, String}()
	strings = ["sam 99912222", "tom 11122222", "harry 12299933"]
	for i = 1:n
		words = collect(eachsplit(strings[i], " "))
		merge!(book, Dict(words[1] => words[2]))
	end
	return book
end
function call(book::Dict{String, String}, person::String)
	haskey(book, person) ? println(book[person]) : println("Not found")
end
function main()
	phoneBook = populate()

	people = ["sam", "edward", "harry"]

	for i in people
		call(phoneBook, i)
	end

end
main()
