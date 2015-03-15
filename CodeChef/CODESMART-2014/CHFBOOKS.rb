t = gets.chomp.to_i

t.times do

	inp = gets.chomp.split(" ").map(&:to_i)
	
	n = inp[0]
	m = inp[1]
	
	arr = gets.chomp.split(" ").map(&:to_i)
	
	arr.sort!
	
	i=0
	sum1=0
	sum2=0
	
	while i < m
		sum1+=arr[i]
		i+=1
	end
	
	while i < n
		sum2+=arr[i]
		i+=1
	end
	
	ans = sum2 - sum1
	
	puts ans.abs
end