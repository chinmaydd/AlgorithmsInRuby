t = gets.chomp.to_i
t.times do
	s = gets.chomp.split('')
	
	if s.length == 1
		puts '0'
	else
		count1 = 0
		count2 = 0
	
		for i in 0..s.length-1
			if i%2==0 && s[i]!='+'
				count1+=1
			elsif i%2==1 && s[i]!='-'
				count1+=1
			end
		end
	
		for i in 0..s.length-1
			if i%2==0 && s[i]!='-'
				count2+=1
			elsif i%2==1 && s[i]!='+'
				count2+=1
			end
		end

		puts [count1, count2].min
	end
	
end