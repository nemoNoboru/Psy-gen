require 'ffi'

class Psygen < FFI::AutoPointer
  def self.release(ptr)
    Binding.free(ptr)
  end

  def nextGen
    Binding.nextGen(self)
  end

  module Binding
    extend FFI::Library
    ffi_lib './libpsygen.so'

    attach_function :new, 	:newPsygen, 	[:int32], 	Psygen
    attach_function :nextGen, 	:next,		[Psygen], 	:int32
    attach_function :free, 	:psygen_free, 	[Psygen], 	:void
  end
end

t = Psygen::Binding.new 100_000
var = 1
while var >= 0
var = t.nextGen
#puts var
end

