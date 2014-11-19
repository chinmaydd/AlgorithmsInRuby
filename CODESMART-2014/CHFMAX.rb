t = gets.chomp.to_i

t.times do
	
	s = gets.chomp
	
	arr = Array.new(26, 0)
	i=0
	
	while i < s.length
		arr[s[i].ord-97]+=1
		i+=1
	end
	
	maximum = -1
	ascii = 1000
	
	i = 0
	
	while i < 26
		if (arr[i] > maximum)||(arr[i]==maximum&&(i maximum = arr[i]
			ascii = i
		end
		i+=1
	end
	
	i = 0
	
	s.gsub!((ascii+97).chr, '?') #global substitution in array
	
	puts s
end