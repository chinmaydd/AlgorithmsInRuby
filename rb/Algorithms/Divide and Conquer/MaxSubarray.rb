class Integer
  N_BYTES = [42].pack('i').size
  N_BITS = N_BYTES * 16
  MAX = 2 ** (N_BITS - 2) - 1
  MIN = -MAX - 1
end

def FindMaxCrossingSubarray(l, m, h)
	left_sum = Integer::MIN
	sum = 0
	for i in (m).downto(l)
		sum += A[i]
		if left_sum < sum
			left_sum = sum
			maxleft = i
		end
	end

	right_sum = Integer::MIN
	sum = 0
	for i in (m+1).upto(h)
		sum += A[i]
		if right_sum < sum
			right_sum = sum
			maxright = i
		end
	end

	return maxleft, maxright, (left_sum + right_sum)

end

def FindMaxSubarray(l, h)
	if h == l
		return l, h, A[l]
	else
		m = (h+l)/2
		left_low, left_high, l_sum = FindMaxSubarray(l, m)
		right_low, right_high, r_sum = FindMaxSubarray(m+1, h)
		cross_low, cross_high, c_sum = FindMaxCrossingSubarray(l, m, h)

		if l_sum > r_sum && l_sum > c_sum
			return left_low, left_high, l_sum
		elsif r_sum > l_sum && r_sum > c_sum
			return right_low, right_high, r_sum
		else
			return cross_low, cross_high, c_sum
		end
	end
end

puts "Enter the number of elements of the array:"
n = gets.chomp.to_i

puts "Enter the elements of the array:"
A = gets.chomp.split(' ').map{|x| x.to_i}

l, h, sum = FindMaxSubarray(0, n-1)

puts
puts "Start Index: #{l}"
puts "Ending Index: #{h}"
puts "Maximum Sum; #{sum}"
puts





	
			

