# frozen_string_literal: true

module DTRToRust
  module Instruction
    class Increment < Handler
      def handle
        form_rust_string("increment: #{@instruction.inputs[0]}")
      end
    end
  end
end
