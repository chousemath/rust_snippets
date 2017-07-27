require 'ffi'

module EmbedWaggle
  extend FFI::Library
  ffi_lib 'embed_waggle/target/release/libembed_waggle.dylib'
  attach_function :hello, [], :void
  attach_function :say, [:string], :void
  attach_function :sum, [:int, :int], :int
  attach_function :get_string, [:string], :pointer
end

EmbedWaggle.hello()
puts EmbedWaggle.sum(1, 2)
# EmbedWaggle.say("Repeated!")

puts 'done!'
