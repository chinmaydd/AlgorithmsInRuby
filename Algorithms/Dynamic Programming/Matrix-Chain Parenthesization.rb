# Implementing Matrix Chain Parenthesization

puts "Enter the number of matrices you want to multiply:"
n = gets.chomp.to_i

puts "Enter the values of matrix Rows and Collumns:"
p = gets.chomp.split(' ').map{|x| x.to_i }

# Martix chain order 
m = Array.new(n+1){Array.new(n+1, 0)}

for i in 1..n
	m[i][i] = 0
end

for l in 2..n # l is the chain length here
	for i in 1..n-l+1
		j = i+l-1
		m[i][j] = 10000000 #Any arbitrary large value

		for k in i..j-1
			q = m[i][k] + m[k+1][j] + p[i-1]*p[k]*p[j]
			if q<m[i][j]
				m[i][j] = q
			end
		end
	end
end

puts
puts m[1][n]
