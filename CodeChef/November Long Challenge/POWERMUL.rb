t = gets.chomp.to_i

t.times do

	inp = gets.chomp.split(" ")

	n = inp[0].to_i
	m = inp[1].to_i
	q = inp[2].to_i

	arr = []
	arr[1]=1

	for i in 2..n
		arr[i] = arr[i-1]* (i**i)
	end

	q.times do
		r = gets.chomp.to_i
		ans = (arr[n]/(arr[n-r]*arr[r]))%m
		puts ans
	end
end