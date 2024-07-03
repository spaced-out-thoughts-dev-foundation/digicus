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

      name_section = capture_section(/\[Contract\]:\s*(.+)/)

      raise 'Missing contract name.' if name_section.nil?

      @name_section ||= name_section.strip
    end

    def state_section
      return @state_definitions if @state_definitions

      state_section = capture_section(/\[State\]:\s*((?:\s*\*\s*\[.+?\]\n(?:\s*\*.+\n?)*)*)\s*:\[State\]/)

      return nil if state_section.nil?

      state_definitions = state_section
                          .split(/\n\s*\*\s*\[/).map { |x| "[#{x.strip}" }
                          .map { |definition| DTRCore::State.from_definition(definition) }

      @state_section ||= state_definitions
    end

    def interface_section
      return @function_definitions if @function_definitions

      interface_section = capture_section(/\[Interface\]:(?<all>.*):\[Interface\]/m)

      return nil if interface_section.nil?

      function_definitions = interface_section.split('-()').map do |x|
        DTRCore::Function.from_definition(x.strip.to_s)
      end

      function_definitions.reject! { |x| x.name.nil? }

      @interface_section ||= function_definitions
    end

    def user_defined_types_section
      return @user_defined_types if @user_defined_types

      user_defined_types_regex = /\[User Defined Types\]:([\s\S]*?)\s*:\[User Defined Types\]/
      user_defined_types_section_parsed_out = capture_section(user_defined_types_regex)

      return nil if user_defined_types_section_parsed_out.nil?

      user_defined_types = user_defined_types_section_parsed_out
                           .split(/\n\s*\*\s*\(/).map { |x| "(#{x.strip}" }
                           .filter { |x| x.length > 1 }
                           .map { |definition| DTRCore::UserDefinedType.from_definition(definition) }

      @user_defined_types_section ||= user_defined_types
    end

    def helpers_section
      return @helpers_section if @helpers_section

      helpers_section = capture_section(/\[Helpers\]:(?<all>.*)\s*:\[Helpers\]/m)

      return nil if helpers_section.nil?

      function_definitions = helpers_section.split('-()').map do |x|
        DTRCore::Function.from_definition(x.strip.to_s)
      end

      function_definitions.reject! { |x| x.name.nil? }

      @helpers_section ||= function_definitions
    end

    def non_translatable_section
      return @non_translatable_section if @non_translatable_section

      non_translatable_section = capture_section(/\[NonTranslatable\]:(?<all>.*)\s*:\[NonTranslatable\]/m)

      return nil if non_translatable_section.nil?

      @non_translatable_section = non_translatable_section
    end
  end
end
