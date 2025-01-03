class Object
  def hello
    puts "world"
    123
  end
end

def main
  obj = Object.new
  debug obj do
    [1]
  end
  obj.hello
end

main