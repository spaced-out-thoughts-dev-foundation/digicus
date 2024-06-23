# frozen_string_literal: true

module DTRToRust
  module Instruction
    class EndOfIterationCheck < Handler
      def handle
        form_rust_string('if !iteration_finished {')
      end
    end
  end
end
