t = gets.chomp.to_i

m=0

while m < t
	s = gets.chomp
	
	str = s.split("")
	
	len = s.length
	
	k=len-1
	
	flag=0
	i=0
	
	while 1 
		if str[i]!=str[k]
			flag=1
			break;
		end
	i+=1
	k-=1
	
	end
	
	if flag==1
		temp1 = s[i..k-1]
		temp2 = s[i+1..k]
		if temp1==temp1.reverse||temp2==temp2.reverse
			flag=0
		else
			flag=1
		end
	end
	
	if flag==0
		puts "YES"
	else
		puts "NO"
	end
	
	m+=1
end