# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the and instruction.
    class And < Handler
      def handle
        inputs = @instruction.inputs
        assignment = @instruction.assign

        assignment_rust = "let #{assignment} = "
        body_rust = "#{inputs[0]} && #{inputs[1]};"
        rust_string = "#{assignment.nil? ? '' : assignment_rust}#{body_rust}"

        form_rust_string(rust_string, @instruction.scope)
      end
    end
  end
end
