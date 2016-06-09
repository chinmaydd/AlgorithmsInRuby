$a = []
$b = []

puts "Input Size:"
n = gets.chomp.to_i

$a = (1..1000).to_a.sample n

def merge(l, r, u)
	i = l
	j = r
	k = l
	while i<r && j<u
		if $a[i] <= $a[j]
			$b[k] = $a[i]
			i+=1
		else
			$b[k] = $a[j]
			j+=1
		end
		k+=1
	end

	while i<r
		$b[k] = $a[i]
		i+=1
		k+=1
	end

	while j<u
		$b[k] = $a[j]
		j+=1
		k+=1
	end

	for k in l..u-1
		$a[k] = $b[k]
	end
end

def sort(n)
	k = 1
	while k<n	
		i = 0
		while i+k<=n
			u = i+k*2
			if u>n
				u = n
			end
			merge(i, i+k, u)
			i = i+k*2
		end
		k*=2
	end
end

sort(n)

for i in $a
	puts i
end
