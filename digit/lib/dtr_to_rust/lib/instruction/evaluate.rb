# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the Evaluate instruction.
    class Evaluate < Handler
      def handle
        inputs = @instruction[:inputs][1..]
        evaluated_method_name = @instruction[:inputs][0]
        assignment = @instruction[:assign]

        rust_string = if assignment.nil?
                        "#{evaluated_method_name}(#{inputs_to_rust_string(inputs)});"
                      else
                        "let mut #{assignment} = #{evaluated_method_name}(#{inputs_to_rust_string(inputs)});"
                      end

        form_rust_string(rust_string, @instruction[:scope])
      end

      private

      def inputs_to_rust_string(inputs)
        inputs.map { |input| ref_appender(input) }.join(', ')
      end

      def ref_appender(input)
        decorated_input = Common::InputInterpreter.interpret(input)

        if decorated_input[:needs_reference]
          "&#{decorated_input[:value]}"
        else
          decorated_input[:value]
        end
      end
    end
  end
end
