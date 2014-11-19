t = gets.chomp.to_i

t.times do

	inp = gets.chomp.split(" ").map(&:to_i)
	
	n = inp[0]
	k = inp[1]
	
	r = n-k
	
	ans = (k*r*(r-1))/2 + (r*(r-1)*(r-2))/6 + (r*(k-1)*k)/2
	
	puts ans
end