# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Parses a DTR file and returns a Contract object.
  class Parser
    include ::DTRCore::Common

    def initialize(file_path, content: nil)
      if content
        @content = content
      else
        raise "Unable to find file: #{file_path}." unless File.exist?(file_path)

        @content = File.read(file_path)
      end
    end

    attr_reader :content
    attr_accessor :sections

    def name_section
      return @name_section if @name_section

      name_section = first_match_for_content(/\[Contract\]:\s*(.+)/)

      raise 'Missing contract name.' if name_section.nil?

      @name_section ||= name_section.strip
    end

    def state_section
      return @state_definitions if @state_definitions

      state_section = first_match_for_content(/\[State\]:\s*((?:\s*\*\s*\[.+?\]\n(?:\s*\*.+\n?)*)*)/)

      return nil if state_section.nil?

      state_definitions = state_section
                          .split(/\n\s*\*\s*\[/).map { |x| "[#{x.strip}" }
                          .map { |definition| DTRCore::State.from_definition(definition) }

      raise 'Empty state section.' if state_definitions.empty?

      @state_section ||= state_definitions
    end

    def function_section
      return @function_definitions if @function_definitions

      function_section = first_match_for_content(/\[InternalFunctions\]:(?<all>.*):\[InternalFunctions\]/m)

      return nil if function_section.nil?

      function_definitions = function_section.split('-()').map do |x|
        DTRCore::Function.from_definition(x.strip.to_s)
      end

      function_definitions.reject! { |x| x.name.nil? }

      raise 'Empty function section.' if function_definitions.empty?

      @function_section ||= function_definitions
    end

    def user_defined_types_section
      return @user_defined_types if @user_defined_types

      user_defined_types_regex = /\[User Defined Types\]:([\s\S]*?)\s*:\[User Defined Types\]/
      user_defined_types_section_parsed_out = first_match_for_content(user_defined_types_regex)

      return nil if user_defined_types_section_parsed_out.nil?

      user_defined_types = user_defined_types_section_parsed_out
                           .split(/\n\s*\*\s*\(/).map { |x| "(#{x.strip}" }
                           .filter { |x| x.length > 1 }
                           .map { |definition| DTRCore::UserDefinedType.from_definition(definition) }

      @user_defined_types_section ||= user_defined_types
    end
  end
end
