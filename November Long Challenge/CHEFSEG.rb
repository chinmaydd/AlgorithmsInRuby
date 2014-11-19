t = gets.chomp.to_i

t.times do
	inp = gets.chomp
	
	x,k = inp.split(" ")
	
	x = x.to_f
	k = k.to_f
	
	temp = Math.log(k,2).to_i
	#puts temp
	
	p = (k - (2**temp)).to_f
	
	ans = ((2.00 * p + 1.00) * x)/(2.00**(temp+1.00)).to_f
	
	puts '%.9f' % ans
end