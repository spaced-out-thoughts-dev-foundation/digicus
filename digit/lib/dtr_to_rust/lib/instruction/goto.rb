# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the goto instruction.
    class Goto < Handler
      def handle
        form_rust_string("goto: #{@instruction.inputs[0]}")
      end
    end
  end
end
