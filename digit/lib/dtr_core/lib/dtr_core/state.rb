# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Represents a state in a DTR file.
  class State
    include ::DTRCore::Common

    attr_reader :name, :type, :initial_value

    def initialize(name, type, initial_value)
      @name = name
      @type = type
      @initial_value = initial_value
    end

    def sanitize
      @name = clean_name @name
    end

    def self.from_definition(definition)
      not_yet_cleaned_name = definition[/^\[([^\]]+)\]/, 1]

      type = definition[/Type:\s*(\w+)/, 1]

      initial_value = DTRCore::TypeValidator.new(type, definition[/Initial Value:\s*(.+)/, 1])
                                            .validate_then_coerce_initial_value!

      state_object = new(not_yet_cleaned_name, type, initial_value)

      state_object.sanitize

      state_object
    end

    def ==(other)
      name == other.name &&
        type == other.type &&
        initial_value == other.initial_value
    end
  end
end
