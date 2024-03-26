# frozen_string_literal: true

module WasmToRuby
  class Decompiler
    def initialize(wat)
      @wat = wat
    end

    def self.decompile(wat)
      new(wat).decompile
    end

    def decompile
      foobar
    end

    private

    attr_reader :wat

    def foobar
      parse_module(wat)
    end

    def parse_module(_wat)
      # module = Module.new
      # module.name = wat[:name]
      # module.functions = parse_functions
      # module

      { type: 'module', body: parse_body }
    end

    def parse_body
      body = []
      body << parse_module_sub_thing
      body
    end

    def parse_module_sub_thing
      if wat.include?('func')
        parse_function
      elsif wat.include?('export')
        parse_export
      elsif wat.include?('memory')
        parse_memory
      elsif wat.include?('data')
        parse_data
      elsif wat.include?('type')
        parse_type
      elsif wat.include?('import')
        parse_import
      end
    end

    def parse_function
      { type: 'function' }
    end

    def parse_export
      { type: 'export' }
    end

    def parse_memory
      { type: 'memory' }
    end

    def parse_data
      { type: 'data' }
    end

    def parse_import
      { type: 'import' }
    end

    def parse_type
      { type: 'type' }
    end
  end
end
