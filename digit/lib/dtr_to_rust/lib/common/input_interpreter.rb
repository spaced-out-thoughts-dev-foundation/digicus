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
        if variable?(@input)
          variable_return(@input)
        elsif string?(@input)
          string_return(@input)
        elsif number?(@input)
          number_return(@input)
        end
      end

      private

      ## Variable ##
      def variable?(input)
        !string?(input) && !number?(input)
      end

      def variable_return(_input)
        { value: @input, type: 'variable', needs_reference: true }
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
        input.is_a?(Numeric)
      end

      def number_return(_input)
        { value: @input, type: 'number', needs_reference: false }
      end
    end
  end
end
