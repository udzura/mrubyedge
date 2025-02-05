$memory = nil

LOGLEVEL_INFO = 2
LOGLEVEL_WARN = 1
LOGLEVEL_ERROR = 0

# pub enum WasmBotsError {
# 	EndOfFile,
# 	InvalidData,
# 	EndOfMessageList,
# }
ERROR_EOF = 0
ERROR_INVALID = 1
ERROR_END_OF_MESSAGE_LIST = 2

# pub enum MessageType {
# 	_Error,
# 	InitialParameters,
# 	PresentCircumstances,
# 	Wait,
# 	Resign,
# 	MoveTo,
# 	Open,
# 	Close,
# }
MESSAGE_TYPE_ERROR = 1
MESSAGE_TYPE_INITIAL_PARAMETERS = 2
MESSAGE_TYPE_PRESENT_CIRCUMSTANCES = 3
MESSAGE_TYPE_WAIT = 4
MESSAGE_TYPE_RESIGN = 5
MESSAGE_TYPE_MOVE_TO = 6
MESSAGE_TYPE_OPEN = 7
MESSAGE_TYPE_CLOSE = 8

# pub enum MoveResult {
# 	Succeeded = 0,
# 	Failed = 1,
# 	Invalid = 2,
# 	Error = 3,
# }
MOVE_RESULT_SUCCEEDED = 0
MOVE_RESULT_FAILED = 1
MOVE_RESULT_INVALID = 2
MOVE_RESULT_ERROR = 3

# pub enum TileType {
# 	Void = 0,
# 	Floor = 1,
# 	OpenDoor = 2,
# 	ClosedDoor = 3,
# 	Wall = 4,
# }
TILE_TYPE_VOID = 0
TILE_TYPE_FLOOR = 1
TILE_TYPE_OPEN_DOOR = 2
TILE_TYPE_CLOSED_DOOR = 3
TILE_TYPE_WALL = 4

# pub enum Direction {
# 	North = 0,
# 	Northeast = 1,
# 	East = 2,
# 	Southeast = 3,
# 	South = 4,
# 	Southwest = 5,
# 	West = 6,
# 	Northwest = 7,
# }
DIRECTION_NORTH = 0
DIRECTION_NORTHEAST = 1
DIRECTION_EAST = 2
DIRECTION_SOUTHEAST = 3
DIRECTION_SOUTH = 4
DIRECTION_SOUTHWEST = 5
DIRECTION_WEST = 6
DIRECTION_NORTHWEST = 7

class PresentCircumstances
  def initialize(last_tick_duration, last_move_result, hit_points, surroundings)
    @last_tick_duration = last_tick_duration
    @last_move_result = last_move_result
    @hit_points = hit_points
    @surroundings = surroundings
  end
end

class Brain
  def write_move!(direction)
    logFunction(LOGLEVEL_INFO, "direction: #{direction}")
    $memory[0..2] = [MESSAGE_TYPE_MOVE_TO, direction, 1].pack("C C C")
  end

  def on_tick(curcumstances)

    write_move!(DIRECTION_NORTH)
  end
end

$brain = Brain.new

def clientInitialize
  logFunction(LOGLEVEL_INFO, "Hello, world! This is made by #{RUBY_ENGINE}")
end

def setup(requested_size)
  logFunction(LOGLEVEL_INFO, "received setup with size: #{requested_size}")

  $memory = SharedMemory.new(requested_size)
  name = "mruby/edge wasmbot"
  $memory[0..17] = name
  # $memory[18..25] = "\0" * 8
  $memory[26..27] = [0].pack("S")
  $memory[28..29] = [1].pack("S")
  $memory[30..31] = [0].pack("S")
  $memory
end
  
def receiveGameParams(offset)
  logFunction(LOGLEVEL_INFO, "received parameters with offset: #{offset}")
  param = $memory[offset..(offset+10)].unpack("S S S S C C C")
  logFunction(LOGLEVEL_INFO, "param version: #{param[0]}")
  logFunction(LOGLEVEL_INFO, "param engine version: #{param[1]}.#{param[2]}.#{param[3]}")
  logFunction(LOGLEVEL_INFO, "param diagonal_movement: #{param[4]}")
  logFunction(LOGLEVEL_INFO, "param player_stride: #{param[5]}")
  logFunction(LOGLEVEL_INFO, "param player_open_reach: #{param[6]}")
  true
end
  
def tick(offset)
  param_pre = $memory[offset..(offset+8)].unpack("I C S S")

  surroundings_len = param_pre[3]
  $surroundings = []
  $offset = offset

  surroundings_len.times do |i|
    ptr = $offset + 9 + i
    tile = $memory[ptr..ptr].unpack("C")
    $surroundings.push tile[0]
  end
  curcumstances = PresentCircumstances.new(param_pre[0], param_pre[1], param_pre[2], $surroundings)
  $brain.on_tick(curcumstances)
end