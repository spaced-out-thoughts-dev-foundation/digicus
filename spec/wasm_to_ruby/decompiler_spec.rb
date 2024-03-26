# frozen_string_literal: true

require 'spec_helper'
require './lib/wasm_to_ruby/decompiler'

describe WasmToRuby::Decompiler do
  it 'is a class' do
    expect(described_class).to be_a(Class)
  end

  context 'hello world example' do
    it 'decompiles the hello world example' do
      wat = File.read('examples/simple.wat')
      decompiler = described_class.decompile(wat)

      multi_line_string = ''"
       module Contract
          class HelloContract
            def self.hello(word)
              puts \"Hello, \#{word}!\"
            end
          end
       end
        "''

      expect(decompiler).to eq(multi_line_string)
    end
  end
end
