# Implementing Edit Distance Problem where Delete, Insert and replace have different costs

puts "Enter String 1"
s1 = gets.chomp

puts "Enter String 2"
s2 = gets.chomp

puts "Enter the values for Delete, Insert and Replace"
del, ins, rep = gets.chomp.split(' ').map{ |n| n.to_i }

m = s1.length
n = s2.length

arr = Array.new(m+1) { Array.new(n+1) }

for i in 0..m
	arr[i][0] = ins*i
end

for i in 0..n
	arr[0][i] = del*i
end

for j in 1..n
	for i in 1..m
		if s1[i] == s2[j]
			arr[i][j] = arr[i-1][j-1]
		else
			arr[i][j] = [arr[i-1][j] + del, arr[i][j-1] + ins, arr[i-1][j-1] + rep].min
		end
	end
end

puts arr[m][n]

