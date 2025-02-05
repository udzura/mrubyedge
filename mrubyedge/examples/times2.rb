def do_times
  result = 100
  3.times do |i|
    puts "result = #{result}"
  end
  
  result
end

do_times

def do_times_nest
  result = 200
  3.times do |i|
    3.times do |j|
      puts "result = #{result}"
    end
  end
  
  result
end

do_times_nest