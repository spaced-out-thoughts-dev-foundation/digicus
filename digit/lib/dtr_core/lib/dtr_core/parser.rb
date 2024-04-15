# frozen_string_literal: true

module DTRCore
  # Parses a DTR file and returns a Contract object.
  class Parser
    def initialize(file_path)
      raise "Unable to find file: #{file_path}." unless File.exist?(file_path)

      @content = File.read(file_path)

      # nil is a placeholder for the actual values
      # thus, the actual values are not known yet
      # and so if we see nil we know that section
      # was not included in the dtr file
      @sections = {
        contract_name: nil,
        state: nil,
        functions: nil
      }
    end

    def self.parse(file_path)
      new(file_path).parse
    end

    def parse
      parse_contract_name_section
      parse_state_section
      parse_function_section

      DTRCore::Contract.new(sections[:contract_name], sections[:state], sections[:functions])
    end

    private

    attr_reader :content
    attr_accessor :sections

    def parse_contract_name_section
      # [Contract]: CONTRACT_NAME
      contract_name_pattern = /\[Contract\]:\s*(.+)/

      contract_name_section = content.match(contract_name_pattern)&.captures&.first

      raise 'Missing contract name.' if contract_name_section.nil?

      sections[:contract_name] = contract_name_section
    end

    def parse_state_section
      # Regular expression pattern to match state definitions
      state_pattern = /\[State\]:\s*((?:\s*\*\s*\[.+?\]\n(?:\s*  \* .+\n?)*)*)/

      # Extract the state section using the pattern
      state_section = content.match(state_pattern)&.captures&.first

      return if state_section.nil?

      state_definitions = state_section
                          .split(/\n\s*\*\s*\[/).map { |x| "[#{x.strip}" }
                          .map { |definition| state_definition_to_state_object(definition) }

      raise 'Empty state section.' if state_definitions.empty?

      sections[:state] = state_definitions
    end

    def clean_state_definition_name(definition)
      definition
        &.gsub('*', '')
        &.gsub("\n", '')
        &.gsub('[', '')
        &.strip
    end

    def state_definition_to_state_object(definition)
      name = clean_state_definition_name definition[/^\[([^\]]+)\]/, 1]

      type = definition[/Type:\s*(\w+)/, 1]

      initial_value_raw = definition[/Initial Value:\s*(.+)/, 1]

      validate_type_name_and_initial_value!(type, initial_value_raw)

      DTRCore::State.new(name, type, coerce_initial_value(type, initial_value_raw))
    end

    def parse_function_section; end

    def coerce_initial_value(type_name, initial_value)
      case type_name
      # TODO: ensure size is correct
      # TODO: check type
      # TODO: ensure unsigned vs signed
      when 'I32', 'I64', 'I256', 'U32', 'U64', 'U256'
        initial_value.to_i

      # TODO: check type
      when 'Symbol'
        initial_value
          &.gsub('"', '')
          &.gsub("'", '')
      else
        raise 'Missing Invalid Type Name.'
      end
    end

    def validate_type_name_and_initial_value!(type_name, initial_value)
      raise 'Missing Type Name.' if type_name.nil?
      raise 'Missing Initial Value.' if initial_value.nil?
    end
  end
end
