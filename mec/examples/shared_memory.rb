$memory = SharedMemory.new(8192)

def get_memory
  $memory
end

def read_array_from_memory
  result = $memory[0..4].unpack('c c c c')
  result[0] + result[1] + result[2] + result[3]
end