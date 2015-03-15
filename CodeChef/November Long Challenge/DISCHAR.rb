t = gets.chomp.to_i

i=0

while i p = []
	
	count=0
	
	str = gets.chomp
	
	str.each_byte do |c|
		k = c - 97
		#puts k
		if p[k]!=1
			count +=1
			p[k]=1
		end
	end

	puts count
	i+=1

end