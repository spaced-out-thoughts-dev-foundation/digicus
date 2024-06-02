# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Evaluate instruction.
    class Evaluate < Handler
      def handle
        form_rust_string(@instruction[:inputs][0], @instruction[:scope])
      end
    end
  end
end
