# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the add instruction.
    class Add < Handler
      def handle
        form_rust_string("#{@instruction.assign} = #{@instruction.inputs[0]} + #{@instruction.inputs[1]};",
                         @instruction.scope)
      end
    end
  end
end