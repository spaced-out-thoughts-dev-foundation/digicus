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
      validate_input!

      return validate_integer! if %w[Integer BigInteger Float].include?(@type_name)
      return validate_string! if ['String'].include?(@type_name)
      return validate_address! if ['Address'].include?(@type_name)
      return validate_boolean! if ['Boolean'].include?(@type_name)

      raise 'Missing Invalid Type Name.'
    end

    private

    def validate_input!
      raise 'Missing Type Name.' if @type_name.nil?
      raise 'Missing Initial Value.' if @initial_value.nil?
    end

    # TODO: implement deeper validation for Address type
    # TODO: confirm this works for non-Stellar addresses
    def validate_address!
      raise "Invalid initial value for Address: #{@initial_value}. Wrong type." unless @initial_value.length == 56

      @initial_value
    end

    def validate_string!
      unless @initial_value.is_a?(String) && @initial_value.match(/".*"/)
        raise "Invalid initial value for String: #{@initial_value}. Wrong type."
      end

      @initial_value.strip
    end

    def validate_boolean!
      unless %w[true
                false].include?(@initial_value)
        raise "Invalid initial value for Boolean: #{@initial_value}. Wrong type."
      end

      @initial_value == 'true'
    end

    def validate_integer!
      unless @initial_value =~ (/^[\-\.\d]\d*(\.?\d*)*/)
        raise "Invalid initial value for #{@type_name}: #{@initial_value}. Wrong type."
      end

      raise "Invalid initial value for type #{@type_name}. Out of range." unless @initial_value.to_i.between?(
        DTRCore::Number.const_get(:"MIN_#{@type_name}"), DTRCore::Number.const_get(:"MAX_#{@type_name}")
      )

      handle_each_numeric_type!
    end

    def handle_each_numeric_type!
      case @type_name
      when 'Integer', 'BigInteger'
        raise "Invalid initial value for #{@type_name}: #{@initial_value}. Wrong type." unless non_float_integer?

        @initial_value.to_i
      when 'Float'
        raise "Invalid initial value for #{@type_name}: #{@initial_value}. Wrong type." unless floaty_float?

        @initial_value.to_f
      end
    end

    def non_float_integer?
      @initial_value =~ (/^[\-\.\d]\d*(\.?\d*)*/) && !@initial_value.include?('.')
    end

    def floaty_float?
      @initial_value =~ (/^[\-\.\d]\d*(\.?\d*)*/) && @initial_value.include?('.')
    end
  end
end
