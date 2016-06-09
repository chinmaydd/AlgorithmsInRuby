# Implementing Knapsack Problem 

 puts "Input value array:"
 val = gets.chomp.split(' ').map{ |n| n.to_i }

 puts "Input weight array:"
 weight = gets.chomp.split(' ').map{ |n| n.to_i }

 puts "Input weight of the Knapsack"
 w = gets.chomp.to_i

 n = val.length

 knap = Array.new(n+1){ Array.new(w+1) }

for i in 0..n
	for j in 0..w
		if i==0 or w==0
			knap[i][j] = 0
		elsif weight[i-1] <= j
			knap[i][j] = [val[i-1] + knap[i-1][j-weight[i-1]], knap[i-1][j]].max
		else
			knap[i][j] = knap[i-1][j]
		end
	end
end

puts knap[n][w]