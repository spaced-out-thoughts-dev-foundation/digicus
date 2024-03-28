# frozen_string_literal: true

require './lib/wasm_to_ruby/s_expression_helper'

module WasmToRuby
  class Decorator
    def initialize(wat)
      @wat = wat
    end

    def self.decorate(wat)
      new(wat).decorate
    end

    def decorate
      decorate_each_s_exp
    end

    private

    attr_reader :wat

    def decorate_each_s_exp
      body = []

      SExpressionHelper.make_levels(wat).each do |thing|
        body << parse_module_sub_thing_full(thing)
      end

      body
    end

    def parse_module_sub_thing(thing)
      if thing.include?('func')
        parse_function(thing)
      elsif thing.include?('module')
        parse_module(thing)
      elsif thing.include?('export')
        parse_export(thing)
      elsif thing.include?('memory')
        parse_memory(thing)
      elsif thing.include?('data')
        parse_data(thing)
      elsif thing.include?('type')
        parse_type(thing)
      elsif thing.include?('import')
        parse_import(thing)
      elsif thing.include?('global')
        parse_global(thing)
      elsif thing.include?('param')
        parse_param(thing)
      elsif thing.include?('result')
        parse_result(thing)
      elsif thing.start_with?(';')
        parse_comment(thing)
      else
        parse_statement(thing)
      end
    end

    def parse_module_sub_thing_full(thing)
      return parse_module_sub_thing(thing) unless thing.is_a?(Array)
      
      thing.map do |sub_thing|
        parse_module_sub_thing_full(sub_thing)
      end
    end

    def parse_function(value)
      { type: 'function', value: value}
    end

    def parse_module(value)
      { type: 'module', value: value}
    end

    def parse_export(value)
      { type: 'export', value: value}
    end

    def parse_memory(value)
      { type: 'memory', value: value}
    end

    def parse_data(value)
      { type: 'data', value: value}
    end

    def parse_import(value)
      { type: 'import', value: value}
    end

    def parse_type(value)
      { type: 'type', value: value}
    end

    def parse_global(value)
      { type: 'global', value: value}
    end

    def parse_param(value)
      { type: 'param', value: value}
    end

    def parse_result(value)
      { type: 'result', value: value}
    end

    def parse_comment(value)
      { type: 'comment', value: value}
    end

    def parse_statement(value)
      { type: 'statement', value: value}
    end
  end
end
