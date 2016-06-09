# Implementing Rod Cutting Problem

puts "Enter the value correspoding to the index as length"
p = gets.chomp.split(' ').map{ |n| n.to_i }
n = p.length
p.unshift(0)

r = []
r[0] = 0

for j in 1..n
	q = -1
	for i in 1..j
		q = [q, p[i] + r[j-i] ].max
	end
	r[j] = q
end

# Printing output
for i in 0..n
	print "#{i}   "
end

puts

for i in 0..n
	print "#{r[i]}   "
end
