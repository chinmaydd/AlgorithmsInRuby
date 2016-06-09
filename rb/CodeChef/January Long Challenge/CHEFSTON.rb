t = gets.chomp.to_i

t.times do
	n,k = gets.chomp.split(' ').map{|x| x.to_i}
	a = gets.chomp.split(' ').map{|x| x.to_i}
	b = gets.chomp.split(' ').map{|x| x.to_i}
	max = 0
	
	for i in 0..n-1
		temp = (k/a[i]).to_i
		prof = temp * b[i]
		if prof > max
			max = prof
		end
	end

	puts max
end