# frozen_string_literal: true

require 'spec_helper'
require './lib/wasm_to_ruby/s_expression_helper'

describe WasmToRuby::SExpressionHelper do
  context 'make_levels' do
    it 'minimum viable s-expression' do
      input = '(module)'
      levels = described_class.make_levels(input)
      expect(levels).to eq(['module'])
    end

    it 'nested s-expressions' do
      input = '(module (func))'
      levels = described_class.make_levels(input)
      expect(levels).to eq(['module', ['func']])
    end

    it 'nested s-expressions with multiple children' do
      input = '(module (func) (func))'
      levels = described_class.make_levels(input)
      expect(levels).to eq(['module', ['func'], ['func']])
    end

    it 'nested s-expressions with multiple children and grandchildren' do
      input = '(module (func (export "_start")))'
      levels = described_class.make_levels(input)
      expect(levels).to eq(['module', ['func', ['export "_start"']]])
    end

    it 'nested s-expressions with multiple children and grandchildren' do
      input = '(module (func (export "_start")) (func))'
      levels = described_class.make_levels(input)
      expect(levels).to eq(['module', ['func'], ['func', ['export "_start"']]])
    end

    it 'nested s-expressions with newlines are dropped' do
      input = "(module\n (func\n)\n)"
      levels = described_class.make_levels(input)
      expect(levels).to eq(['module', ['func']])
    end
  end
end
