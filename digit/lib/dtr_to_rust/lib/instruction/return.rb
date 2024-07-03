# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Return instruction.
    class Return < Handler
      def handle
        form_rust_string("return #{@instruction.inputs[0]};")
      end
    end
  end
end
