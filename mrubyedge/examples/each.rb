def do_each
  j = 0
  [1, 2, 3].each do |i|
    j += i
    puts "j = #{j}"
  end

  j = 0
  [1, 2, 3].each do |i|
    [10, 20, 30].each do |k|
      j += i + k
      puts "j = #{j}"
    end
  end
end

do_each