# frozen_string_literal: true

module DTRCore
  # Represents a contract in a DTR file.
  class Contract
    attr_reader :functions, :name, :state

    def initialize(name, state, functions)
      @name = name
      @state = state
      @functions = functions
    end

    def self.from_dtr(filepath)
      parser = DTRCore::Parser.new(filepath)

      new(parser.name_section, parser.state_section, parser.function_section)
    end

    def ==(other)
      name == other.name &&
        state == other.state &&
        functions == other.functions
    end
  end
end
