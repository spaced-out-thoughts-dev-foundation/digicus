# frozen_string_literal: true

module DTRToRust
  module Common
    # This class is responsible for interpreting the input string.
    class InputInterpreter
      def initialize(input)
        @input = input
      end

      def self.interpret(input)
        new(input).interpret
      end

      def interpret
        if number?(@input)
          number_return(@input)
        elsif string?(@input)
          string_return(@input)
        elsif boolean?(@input)
          boolean_return(@input)
        else
          variable_return(@input)
        end
      end

      private

      ## Variable ##
      def variable?(input)
        !string?(input) && !number?(input)
      end

      def variable_return(_input)
        { value: @input, type: 'variable', needs_reference: !@input.start_with?('&') }
      end

      ## String ##
      def string?(input)
        input.is_a?(String) && (input.match?(/".*"/) || input.match?(/'.*'/))
      end

      def string_return(_input)
        { value: @input, type: 'string', needs_reference: false }
      end

      ## Number ##
      def number?(input)
        input.is_a?(Numeric) || input&.match?(/^\s*\d+\.?\d*\s*$/)
      end

      def number_return(_input)
        { value: contains_decimal?(@input.to_s) ? @input.to_f : @input.to_i, type: 'number', needs_reference: false }
      end

      def contains_decimal?(str)
        # Define a regular expression pattern for a decimal number
        decimal_pattern = /\d+\.\d+/

        # Check if the string matches the pattern
        !!(str =~ decimal_pattern)
      end

      ## Boolean ##
      def boolean?(input)
        input.is_a?(TrueClass) || input.is_a?(FalseClass) || input&.match?(/true|false/)
      end

      def boolean_return(_input)
        { value: @input, type: 'boolean', needs_reference: false }
      end
    end
  end
end
