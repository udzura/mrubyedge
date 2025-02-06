def do_times
  result = 100
  3.times do |i|
    result += 100
  end
  
  result
end

puts "t1 = #{do_times}"

def do_times_nest
  result = 200
  3.times do |i|
    3.times do |j|
      result += 200
    end
  end
  
  result
end

puts "t2 = #{do_times_nest}"