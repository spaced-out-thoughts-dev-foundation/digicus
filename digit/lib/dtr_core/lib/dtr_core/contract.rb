# frozen_string_literal: true

module DTRCore
  # Represents a contract in a DTR file.
  class Contract
    attr_reader :functions, :name, :state, :user_defined_types

    def initialize(name, state, functions, user_defined_types)
      @name = name
      @state = state
      @functions = functions
      @user_defined_types = user_defined_types
    end

    def self.from_dtr(filepath)
      parser = DTRCore::Parser.new(filepath)

      new(parser.name_section, parser.state_section, parser.function_section, parser.user_defined_types_section)
    end

    def self.from_dtr_raw(content)
      parser = DTRCore::Parser.new('', content:)

      new(parser.name_section, parser.state_section, parser.function_section, parser.user_defined_types_section)
    end

    def ==(other)
      name == other.name &&
        state == other.state &&
        functions == other.functions &&
        user_defined_types == other.user_defined_types
    end
  end
end
