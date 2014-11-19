t = gets.chomp.to_i
flag=0
#flag=0 means that root is black

t.times do
		inp = gets.chomp

		if inp=="Qi"&&flag==1
			flag=0
			next
		elsif inp=="Qi"&&flag==0
			flag=1
			next
		end

		arr = inp.split(" ")
		x = arr[1].to_i
		y = arr[2].to_i

		levelx = Math.log2(x).to_i
		levely = Math.log2(y).to_i

		if x==y
			if (arr[0]=="Qb"&&flag==0&&levelx%2==0)||(arr[0]=="Qr"&&flag==0&&levelx%2==1)||(arr[0]=="Qb"&&flag==1&&levelx%2==1)||(arr[0]=="Qr"&&flag==1&&levelx%2==10)
				puts "1"
			else
				puts "0"
			end
			next
		end



		x = x.to_s(2)
		y = y.to_s(2)

		i = 0

		s = String.new
		minimum = [x.length , y.length].min

		while i < minimum
			if x[i]!=y[i]
				break
			end
			s.concat(x[i])
			i+=1
		end

		ancestor = s.to_i(2)

		levelanc = Math.log2(ancestor).to_i

		colorx=0
		colory=0
		coloranc=0

		#if color is 0 , it means that the node is black

		no_of_red=0
		no_of_black=0

		if levelx%2==1
			colorx=1
			no_of_red+=1
		else
			no_of_black+=1
		end

		if levely%2==1
			colory=1
			no_of_red+=1
		else
			no_of_black+=1
		end

		if levelanc%2==1
			coloranc=1
			no_of_red+=1
		else
			no_of_black+=1
		end

		#puts "#{no_of_red} #{no_of_black}"

		if s==x&&coloranc==1
			no_of_red-=1
		elsif s==x&&coloranc==0
			no_of_black-=1
		elsif s==y&&coloranc==1
			no_of_red-=1
		elsif s==y&&coloranc==0
			no_of_black-=1
		end

		#puts "#{no_of_red} #{no_of_black}"
			

		if coloranc==colorx&&colorx==0
			if levelx - levelanc>=2
				no_of_black+= (levelx - levelanc)/2 - 1
			end
			no_of_red+= (levelx - levelanc)/2
		elsif coloranc==colorx&&colorx==1
			if levelx - levelanc>=2
				no_of_red += (levelx - levelanc)/2 - 1
			end
			no_of_black+= (levelx - levelanc)/2
		else
			no_of_black+= (levelx - levelanc)/2
			no_of_red+= (levelx - levelanc)/2
		end

		#puts "#{no_of_red} #{no_of_black}"


		if coloranc==colory&&colory==0
			if levely - levelanc>=2
				no_of_black+= (levely - levelanc)/2 - 1
			end
			no_of_red+= (levely - levelanc)/2
		elsif coloranc==colory&&colory==1
			if levely - levelanc>=2
				no_of_red+= (levely - levelanc)/2 - 1
			end
			no_of_black += (levely - levelanc)/2
		else
			no_of_black+= (levely - levelanc)/2
			no_of_red+= (levely - levelanc)/2
		end

		if arr[0]=="Qr"&&flag==0||arr[0]=="Qb"&&flag==1
			puts no_of_red
		else
			puts no_of_black
		end
end