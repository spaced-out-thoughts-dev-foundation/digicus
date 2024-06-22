# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the assign instruction.
    class Assign < Handler
      def handle
        form_rust_string("let mut #{@instruction.assign} = #{@instruction.inputs[0]};")
      end
    end
  end
end
