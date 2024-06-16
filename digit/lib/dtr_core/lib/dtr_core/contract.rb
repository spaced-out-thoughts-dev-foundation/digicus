# frozen_string_literal: true

module DTRCore
  # Represents a contract in a DTR file.
  class Contract
    attr_reader :helpers, :interface, :name, :state, :user_defined_types

    def initialize(name, state, interface, user_defined_types, helpers)
      @name = name
      @state = state
      @interface = interface
      @user_defined_types = user_defined_types
      @helpers = helpers
    end

    def self.from_dtr(filepath)
      parser = DTRCore::Parser.new(filepath)

      new(parser.name_section, parser.state_section, parser.interface_section, parser.user_defined_types_section,
          parser.helpers_section)
    end

    def self.from_dtr_raw(content)
      parser = DTRCore::Parser.new('', content:)

      new(parser.name_section, parser.state_section, parser.interface_section, parser.user_defined_types_section,
          parser.helpers_section)
    end

    def ==(other)
      name == other.name &&
        state == other.state &&
        interface == other.interface &&
        user_defined_types == other.user_defined_types
    end

    def to_s
      return_string = ''

      return_string += name_to_s
      return_string += "#{state_to_s}\n"
      return_string += interface_to_s
      return_string += user_defined_types_to_s
      return_string += helpers_to_s

      return_string
    end

    private

    def name_to_s
      "[Contract]: #{@name}\n\n"
    end

    def state_to_s
      return '' if @state.nil?

      "[State]:\n#{@state&.map(&:to_s)&.join("\n")}\n:[State]\n"
    end

    def interface_to_s
      return '' if @state.nil?

      "[Interface]:\n#{@interface&.map(&:to_s)&.join("\n")}\n:[Interface]\n"
    end

    def user_defined_types_to_s
      return '' if @user_defined_types.nil?

      "[User Defined Types]:\n#{@user_defined_types&.map(&:to_s)&.join("\n")}\n:[User Defined Types]\n"
    end

    def helpers_to_s
      return '' if @helpers.nil?

      "[Helpers]:\n#{@helpers&.map(&:to_s)&.join("\n")}\n:[Helpers]\n"
    end
  end
end
