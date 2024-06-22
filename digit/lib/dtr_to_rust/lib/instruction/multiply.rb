# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the add instruction.
    class Multiply < Handler
      def handle
        form_rust_string("#{@instruction.assign} = #{@instruction.inputs[0]} * #{@instruction.inputs[1]};")
      end
    end
  end
end
