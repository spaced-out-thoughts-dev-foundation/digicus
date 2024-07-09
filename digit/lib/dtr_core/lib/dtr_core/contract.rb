# frozen_string_literal: true

module DTRCore
  # Represents a contract in a DTR file.
  class Contract
    attr_reader :helpers, :interface, :name, :state, :user_defined_types, :non_translatables

    def initialize(name, state, interface, user_defined_types, helpers, non_translatables)
      @name = name
      @state = state
      @interface = interface
      @user_defined_types = user_defined_types
      @helpers = helpers
      @non_translatables = non_translatables
    end

    def self.from_dtr(filepath)
      parser = DTRCore::Parser.new(filepath)

      new(parser.name_section, parser.state_section, parser.interface_section, parser.user_defined_types_section,
          parser.helpers_section, parser.non_translatable_section)
    end

    def self.from_dtr_raw(content)
      parser = DTRCore::Parser.new('', content:)

      new(parser.name_section, parser.state_section, parser.interface_section, parser.user_defined_types_section,
          parser.helpers_section, parser.non_translatable_section)
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
      return_string += non_translatables_to_s

      return_string
    end

    private

    def name_to_s
      "[Contract]: #{@name}\n\n"
    end

    def state_to_s
      return '' if @state.nil? || @state.empty?

      "[State]:\n#{@state&.map(&:to_s)&.join("\n")}\n:[State]\n"
    end

    def interface_to_s
      return '' if @interface.nil? || @interface.empty?

      "[Interface]:\n#{@interface&.map(&:to_s)&.join("\n")}\n:[Interface]\n"
    end

    def user_defined_types_to_s
      return '' if @user_defined_types.nil? || @user_defined_types.empty?

      "[User Defined Types]:\n#{@user_defined_types&.map(&:to_s)&.join("\n")}\n:[User Defined Types]\n"
    end

    def helpers_to_s
      return '' if @helpers.nil? || @helpers.empty?

      "[Helpers]:\n#{@helpers&.map(&:to_s)&.join("\n")}\n:[Helpers]\n"
    end

    def non_translatables_to_s
      return '' if @non_translatables.nil? || @non_translatables.empty?

      "[NonTranslatable]:\n#{@non_translatables}\n:[NonTranslatable]"
    end
  end
end
