t = gets.chomp.to_i
t.times do
	n = gets.chomp.to_i
	
	arr = gets.chomp.split(' ').map{|x| x.to_i}
	
	m = arr.max
	
	count = []
	
	for i in 0..100001
		count << 0
	end
	
	for i in arr
		count[i]+=1
	end
	
	max = -100001
	save = 0
	
	for i in 0..m
		if count[i]>max
			max = count[i]
			save = i
		end
	end
	
	temp = 0
	
	for i in arr
		if i!=save
			temp+=1
		end
	end
	
	puts temp
end