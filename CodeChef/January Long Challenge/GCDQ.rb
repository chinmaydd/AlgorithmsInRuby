t = gets.chomp.to_i

t.times do
	n,q = gets.chomp.split(' ').map{|x| x.to_i}
	arr = gets.chomp.split(' ').map{|x| x.to_i}
	
	g1 = Array.new(0,n)
	g2 = Array.new(0,n)
	
	g1[0] = arr[0]
	g2[n-1] = arr[n-1]
	
	for i in 1..n-1
		g1[i] = arr[i].gcd(g1[i-1])
		g2[n-i-1] = arr[n-i-1].gcd(g2[n-i])
	end
	
	q.times do
		a,b = gets.chomp.split(' ').map{|x| x.to_i}
		if a==1
			puts g2[b]
		elsif b==n
			puts g1[a-2]
		else
			puts g1[a-2].gcd(g2[b])
		end
	end
end