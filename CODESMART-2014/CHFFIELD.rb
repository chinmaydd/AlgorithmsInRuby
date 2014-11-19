n = 1000001

nums = [0, 0, *2..n]

(2..Math.sqrt(n)).each do |i|    #implementing the sieve of Erathosnes
	(i**2..n).step(i){|m| nums[m] = 0} if nums[i]
end

t = gets.chomp.to_i

t.times do
	
	inp = gets.chomp.split(" ").map(&:to_i)
	
	l = inp[0]
	b = inp[1]
	i = l
	
	while i >= 1
		if nums[i]==0
			break
		end
		i-=1
	end
	
	lfinal = i
	i=b
	
	while i >= 1
		if nums[i]==0
		break
		end
	i-=1
	end
	
	bfinal = i
	
	puts lfinal*bfinal
end