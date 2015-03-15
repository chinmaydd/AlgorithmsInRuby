puts "Input Size:"

n = gets.chomp.to_i

$a = (0..999).to_a.sample n

def partition(l, h)
	x = $a[h]
	i = l-1

	for j in l..h-1
		if $a[j] <= x
			i+=1
			$a[i], $a[j] = $a[j], $a[i]
		end
	end
	$a[i+1], $a[h] = $a[h], $a[i+1]
	return i+1
end

def quickiter(l, h)
	stack = []

	stack.push(l)
	stack.push(h)

	while !stack.empty?

		h = stack.pop
		l = stack.pop

		p = partition(l, h)

		if p-1 > l
			stack.push(l)
			stack.push(p-1)
		end

		if p+1 < h
			stack.push(p+1)
			stack.push(h)
		end
	end
end

quickiter(0, n-1)

for i in $a
	puts i
end