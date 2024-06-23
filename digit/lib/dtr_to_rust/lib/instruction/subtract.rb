# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the add instruction.
    class Subtract < Handler
      def handle
        # TODO: fix this, depends if this is init or not
        form_rust_string("#{@instruction.assign} = #{@instruction.inputs[0]} - #{@instruction.inputs[1]};")
      end
    end
  end
end
