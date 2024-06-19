# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class handles the error_with_message instruction.
    class ErrorWithMessage < Handler
      def handle
        form_rust_string("panic! #{inputs_to_rust_string(@instruction.inputs)};", @instruction.scope)
      end

      private

      def inputs_to_rust_string(inputs)
        inputs.map { |input| ref_appender(input) }.join(', ')
      end

      def ref_appender(input)
        decorated_input = Common::InputInterpreter.interpret(input)

        if decorated_input[:needs_reference] && decorated_input[:value] == 'env'
          "&#{decorated_input[:value]}"
        else
          decorated_input[:value]
        end
      end
    end
  end
end
