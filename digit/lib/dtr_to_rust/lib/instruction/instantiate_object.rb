# frozen_string_literal: true

module DTRToRust
  module Instruction
    # This class is responsible for generating Rust code for the LogString instruction.
    class InstantiateObject < Handler
      def handle
        case @instruction.inputs[0]
        when 'List'
          handle_list
        when 'UDT'
          handle_udt
        else
          raise "Unknown object type: #{@instruction.inputs[0]}"
        end
      end

      private

      def handle_list
        form_rust_string("let mut #{@instruction.assign} = vec![#{inputs_to_rust_string(@instruction.inputs[1..])}];",
                         @instruction.scope)
      end

      def handle_udt
        assignment = "let mut #{@instruction.assign} = "
        udt = "#{@instruction.inputs[1]}{"
        inputs = inputs_to_rust_string(@instruction.inputs[2..])
        end_ = '};'
        form_rust_string("#{assignment}#{udt}#{inputs}#{end_}", @instruction.scope)
      end

      def inputs_to_rust_string(inputs)
        inputs.map { |input| handle_input(input) }.join(', ')
      end

      def handle_input(input)
        decorated_input = Common::InputInterpreter.interpret(input)

        if decorated_input[:type] == 'string'
          "symbol_short!(#{input})"
        elsif decorated_input[:needs_reference] && input == 'env'
          "&#{input}"
        else
          input
        end
      end
    end
  end
end
