# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the label instruction.
    class Label < Handler
      def handle
        form_rust_string("label: #{@instruction.inputs[0]}")
      end
    end
  end
end
