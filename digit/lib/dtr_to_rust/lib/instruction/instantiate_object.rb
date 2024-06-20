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

      def udt_name_fix(udt)
        if udt.name.end_with?('_STRUCT') || udt.name.end_with?('_ENUM')
          udt.name.split('_')[0..-2].join('_')
        else
          udt.name
        end
      end

      def handle_udt
        udt_found = @user_defined_types.filter { |udt| udt_name_fix(udt) == @instruction.inputs[1] }

        assignment = "let mut #{@instruction.assign} = "
        udt = "#{@instruction.inputs[1]}{"
        inputs = inputs_to_rust_string(@instruction.inputs[2..], udt_found[0].attributes.map { |x| x[:name] })
        end_ = '};'
        form_rust_string("#{assignment}#{udt}#{inputs}#{end_}", @instruction.scope)
      end

      def inputs_to_rust_string(inputs, udt_type_names)
        inputs_to_return = []
        inputs.each_with_index do |input, index|
          inputs_to_return << handle_input(input, udt_type_names[index])
        end

        inputs_to_return.join(', ')
      end

      def handle_input(input, udt_type_name)
        decorated_input = Common::InputInterpreter.interpret(input)

        value = if decorated_input[:type] == 'string'
                  "symbol_short!(#{input})"
                elsif decorated_input[:needs_reference] && input == 'env'
                  "&#{input}"
                else
                  input
                end

        "#{udt_type_name}: #{value}"
      end
    end
  end
end
