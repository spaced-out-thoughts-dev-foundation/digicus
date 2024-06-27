# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the add instruction.
    class Subtract < Handler
      def handle
        if @assignment_name_to_scope_map[@instruction.assign] || @instruction.assign.include?('.')
          form_rust_string("#{@instruction.assign} = #{@instruction.inputs[0]} - #{@instruction.inputs[1]};")
        else
          form_rust_string("let mut #{@instruction.assign} = #{@instruction.inputs[0]} - #{@instruction.inputs[1]};")
        end
      end
    end
  end
end
