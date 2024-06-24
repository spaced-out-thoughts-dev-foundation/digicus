# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the assign instruction.
    class Assign < Handler
      def handle
        if @instruction.assign.include?('.') || @instruction.assign == 'Thing_to_return'
          form_rust_string("#{@instruction.assign} = #{@instruction.inputs[0]};")
        else
          form_rust_string("let mut #{@instruction.assign} = #{@instruction.inputs[0]};")
        end
      end
    end
  end
end
