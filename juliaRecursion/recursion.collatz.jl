# clear screen
function clear_screen()
	run(`clear`)
    	flush(stdout)
end

# Calculate the collatz conjecture length of n
function collatz_length(n::Int)
	if n == 1	# base case 
		return 0
	elseif n % 2 == 0	# recursive cases
		return 1 + collatz_length(div(n, 2))
	else
		return 1 + collatz_length(3 * n + 1)
	end
end

# find the 10 smallest integers with the longest collatz sequence length
function find_the_top_10_collatz_integers(start, stop)
	# store at most 10 tuples
	top = Vector{Tuple{Int, Int}}()

	for n in start:stop
        	clength = collatz_length(n)

		# check if length is already in top
		idx_same = findfirst(p -> p[2] == clength, top)
        	if idx_same !== nothing
            	# same length and keep the smaller integer
            		if n < top[idx_same][1]
                		top[idx_same] = (n, clength)
            		end
            		continue
        	end

		# new length - if we have < 10 entries, just append
	 	if length(top) < 10
            		push!(top, (n, clength))
        	else
			# select the worst smallest length, if tie choose smallest integer
			worst_index = 1
            		for i in 2:length(top)
                		ni, li = top[i]		# ni = new integer, li = collatz length of current int
                		nw, lw = top[worst_index]
                		if li < lw || (li == lw && ni > nw)  # lw = length worst
                    			worst_index = i
                		end
            		end
            		worst_n, worst_l = top[worst_index]
		
			# replace
			if clength > worst_l || (clength == worst_l && n < worst_n)
                		top[worst_index] = (n, clength)
            		end
        	end
    	end
	
	# sort
	sort!(top, by = x -> (-x[2], x[1]))
    	return top
end

# main function
function main()
	if length(ARGS) != 2
		println("Usage: julia collatz.jl start stop")
		exit(1)
	end
	x = 0
	y = 0
	try
		x = parse(Int, ARGS[1])
		y = parse(Int, ARGS[2])
	catch e
		println("Error: ", e)
		exit(1)
	end

	# find min and max of the 2 integers to find the range
	start = min(x, y)
	stop = max(x, y)

	top_10 = find_the_top_10_collatz_integers(start, stop)
	
	clear_screen()
	
	println("Sorted based on sequence length")
	for (num, clength) in top_10
		println(lpad(string(num), 15), lpad(string(clength), 15))
	end
	println()
	println("Sorted based on integer size")
	temp_top_10 = copy(top_10)
	sort!(temp_top_10, by = x -> -x[1])
	for (num, clength) in temp_top_10
		println(lpad(string(num), 15), lpad(string(clength), 15))
	end
end

if abspath(PROGRAM_FILE) == @__FILE__
    main()
end
