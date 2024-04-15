# frozen_string_literal: true

module DTRCore
  # Represents a state in a DTR file.
  class State
    attr_reader :name, :type, :initial_value

    def initialize(name, type, initial_value)
      @name = name
      @type = type
      @initial_value = initial_value
    end

    def ==(other)
      name == other.name &&
        type == other.type &&
        initial_value == other.initial_value
    end
  end
end
