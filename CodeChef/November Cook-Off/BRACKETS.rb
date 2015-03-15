t = gets.chomp.to_i

t.times do

	s = gets.chomp.split('')

	sum = 0

	i = 0

	m = -1

	while i < s.length
		if s[i] == '('
			sum+=1
		else
			sum-=1
		end

		if sum > m
			m = sum
		end

		i+=1
	end

	i = 0

	s1 = "(" * m + ")" * m

	puts s1

end





