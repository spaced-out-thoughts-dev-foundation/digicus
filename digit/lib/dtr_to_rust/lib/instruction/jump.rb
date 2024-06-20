# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the jump instruction.
    class Jump < Handler
      def handle
        if @instruction.inputs[0].nil?
          handle_unconditional_jump
        else
          handle_conditional_jump
        end
      end

      private

      def handle_conditional_jump
        form_rust_string("if #{@instruction.inputs[0]} {", @instruction.scope)
      end

      def handle_unconditional_jump
        form_rust_string('if true {', @instruction.scope)
      end
    end
  end
end
