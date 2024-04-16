# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Type validator for DTR types.
  class TypeValidator
    include ::DTRCore::Common

    def initialize(type_name, initial_value)
      @type_name = type_name
      @initial_value = initial_value
    end

    def validate_then_coerce_initial_value!
      raise 'Missing Type Name.' if @type_name.nil?
      raise 'Missing Initial Value.' if @initial_value.nil?

      case @type_name
      when 'I32', 'I64', 'I256', 'U32', 'U64', 'U256'
        validate_numeric!

      # TODO: check type
      when 'Symbol'
        strip_and_remove_quotes(@initial_value)
      else
        raise 'Missing Invalid Type Name.'
      end
    end

    private

    def validate_numeric!
      raise 'Invalid initial value for type. Wrong type.' unless @initial_value =~ (/^[\-\.\d]\d*(\.?\d*)*/)

      raise "Invalid initial value for type #{@type_name}. Out of range." unless @initial_value.to_i.between?(
        DTRCore::Number.const_get(:"MIN_#{@type_name}"), DTRCore::Number.const_get(:"MAX_#{@type_name}")
      )

      @initial_value.to_i
    end
  end
end
