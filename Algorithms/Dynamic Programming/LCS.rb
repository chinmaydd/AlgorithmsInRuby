# Longest Common Substring

puts "Enter the first string:"
x = gets.chomp

puts "Enter second string:"
y = gets.chomp

m = x.length
n = y.length

c = Array.new(m+1){Array.new(n+1)}
b = Array.new(m+1){Array.new(n+1)}

#LCS - Length
for i in 0..m
	c[i][0] = 0
end

for j in 0..n
	c[0][j] = 0
end

for i in 1..m
	for j in 1..n
		if x[i-1]==y[j-1]
			c[i][j] = c[i-1][j-1]+1
			b[i][j] = 'w'
		elsif c[i-1][j] >= c[i][j-1]
			c[i][j] = c[i-1][j]
			b[i][j] = 'n'
		else
			c[i][j] = c[i][j-1]
			b[i][j] = ' '
		end
	end
end

def print_lcs(b, x, i, j)
	if i==0 or j==0
		return
	end
	if b[i][j]=='w'
		print_lcs(b, x, i-1, j-1)
		print x[i-1]
	elsif b[i][j]=='n'
		print_lcs(b, x, i-1, j)
	else
		print_lcs(b, x, i, j-1)
	end
end

print_lcs(b, x, m, n)




