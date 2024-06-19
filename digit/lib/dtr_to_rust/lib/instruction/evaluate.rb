# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Evaluate instruction.
    class Evaluate < Handler
      def handle
        inputs = @instruction.inputs[1..]
        evaluated_method_name = @instruction.inputs[0]
        assignment = @instruction.assign

        assignment_rust = "let mut #{assignment} = "
        # TODO: make this less hacky evaluated_method_name.end_with?('set')
        body_rust = "#{Common::ReferenceAppender.call(evaluated_method_name)}(#{inputs_to_rust_string(inputs,
                                                                                                      evaluated_method_name.end_with?('set'))});"
        rust_string = "#{assignment.nil? ? '' : assignment_rust}#{body_rust}"

        form_rust_string(rust_string, @instruction.scope)
      end

      private

      def inputs_to_rust_string(inputs, ref_nums)
        inputs.map { |input| Common::ReferenceAppender.call(input, ref_nums:) }.join(', ')
      end
    end
  end
end
