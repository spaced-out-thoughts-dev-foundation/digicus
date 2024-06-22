# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the jump instruction.
    class Jump < Handler
      def handle
        if @instruction.inputs.size == 1
          handle_unconditional_jump
        elsif @instruction.inputs.size == 2
          handle_conditional_jump
        else
          raise 'Invalid jump instruction'
        end
      end

      private

      def handle_conditional_jump
        form_rust_string("if #{@instruction.inputs[0]} {")
      end

      def handle_unconditional_jump
        form_rust_string('else {')
      end
    end
  end
end
