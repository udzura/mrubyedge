def main
  arr = []
  arr[0] = 1
  arr << 2
  arr.push 3

  arr.each do |i|
    p "number is: #{i}"
  end
end
