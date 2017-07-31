require 'ffi'

module EmbedWaggle
  extend FFI::Library
  ffi_lib 'embed_waggle/target/release/libembed_waggle.dylib'
  attach_function :hello, [], :void
  attach_function :say, [:string], :void
  attach_function :sum, [:int, :int], :int
  attach_function :sub, [:int, :int], :int
  attach_function :div, [:float, :float], :float
  attach_function :fact, [:int], :int
  attach_function :get_string, [:string], :pointer
end

EmbedWaggle.hello()
puts EmbedWaggle.sum(1, 2)
puts EmbedWaggle.sub(33, 22)
puts EmbedWaggle.div(100, 25)
puts EmbedWaggle.fact(-4)
EmbedWaggle.say("Repeated!")

puts 'done!'
