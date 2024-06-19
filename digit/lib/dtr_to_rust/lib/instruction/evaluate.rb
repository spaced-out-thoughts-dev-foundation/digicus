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
        body_rust = "#{ref_appender(evaluated_method_name)}(#{inputs_to_rust_string(inputs)});"
        rust_string = "#{assignment.nil? ? '' : assignment_rust}#{body_rust}"

        form_rust_string(rust_string, @instruction.scope)
      end

      private

      def inputs_to_rust_string(inputs)
        inputs.map { |input| ref_appender(input) }.join(', ')
      end

      def ref_appender(input)
        decorated_input = Common::InputInterpreter.interpret(input)

        # HACK: is likely chain of method calls
        # ex:  env.storage().instance().get(COUNTER).unwrap_or(0);
        #      --> env.storage().instance().get(&COUNTER).unwrap_or(0);
        if decorated_input[:type] != 'number' &&
           decorated_input[:value].include?('.') &&
           decorated_input[:value].include?('()')
          more_complex_ref_appender(decorated_input)
        elsif decorated_input[:needs_reference]
          "&#{decorated_input[:value]}"
        else
          decorated_input[:value]
        end
      end

      def more_complex_ref_appender(decorated_input)
        decorated_input[:value].split('.').map do |x|
          if call_with_input?(input)
            matches = x.scan(/\((.*?)\)/)
            things = matches
                     .flatten
                     .filter(&method(:not_empty_string?))
                     .map(&method(:wild_stuff))
                     .join(', ')
            "#{x.split('(')[0]}(#{things})"
          else
            x
          end
        end.join('.')
      end

      def not_empty_string?(input)
        input != ''
      end

      def wild_stuff(input)
        input.split(',').map { |x| ref_appender(x.strip) }.join(', ')
      end

      def call_with_input?(input)
        input.include?('(') && input.end_with?(')') && !input.end_with?('()')
      end
    end
  end
end
