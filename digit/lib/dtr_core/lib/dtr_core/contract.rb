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

    def to_s
      return_string = ''

      return_string += name_to_s
      return_string += "#{state_to_s}\n"
      return_string += functions_to_s
      return_string += user_defined_types_to_s

      return_string
    end

    private

    def name_to_s
      "[Contract]: #{@name}\n\n"
    end

    def state_to_s
      return '' if @state.nil?

      "[State]:\n#{@state&.map(&:to_s)&.join("\n")}\n"
    end

    def functions_to_s
      "[InternalFunctions]:\n#{@functions&.map(&:to_s)&.join("\n")}\n:[InternalFunctions]\n"
    end

    def user_defined_types_to_s
      return "[UserDefinedTypes]::[UserDefinedTypes]\n" if @user_defined_types.nil?

      "[UserDefinedTypes]:\n#{@user_defined_types&.map(&:to_s)&.join("\n")}\n:[UserDefinedTypes]\n"
    end
  end
end
