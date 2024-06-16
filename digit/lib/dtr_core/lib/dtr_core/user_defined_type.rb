# frozen_string_literal: true

require 'dtr_core/common'

module DTRCore
  # Represents a state in a DTR file.
  class UserDefinedType
    include ::DTRCore::Common

    attr_reader :attributes, :name

    def initialize(name, attributes)
      @name = name
      @attributes = attributes
    end

    def self.from_definition(definition)
      name = capture_name(definition)
      attributes = capture_attributes(definition)

      new(name, attributes)
    end

    def self.capture_attributes(definition)
      captured_definitions = definition.match(/\{(.+?)\}/m)&.captures&.first

      transform_attributes(captured_definitions.split("\n"))
    end

    def self.transform_attributes(capture_attribute_definition)
      capture_attribute_definition&.map { |x| x.split(':') }
                                  &.filter { |x| x.length > 1 }
                                  &.map { |x| { name: x[0].strip, type: x[1].strip } }
    end

    def self.capture_name(definition)
      definition.match(/\((\w+)\)/)&.captures&.first
    end

    def to_s
      "\t* (#{name})\n\t{\n#{@attributes.map { |x| "\t\t#{x[:name]}: #{x[:type]}" }.join("\n")}\n\t}"
    end

    def ==(other)
      name == other.name &&
        attributes == other.attributes
    end
  end
end
