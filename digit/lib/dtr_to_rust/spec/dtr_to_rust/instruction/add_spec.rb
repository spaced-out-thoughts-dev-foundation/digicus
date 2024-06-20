# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Add do
  describe '#handle' do
    it 'returns the correct Rust code' do
      instruction = DTRCore::Instruction.new('add', %w[foo bar], 'foo', 0)
      expect(described_class.handle(instruction)).to eq('        foo = foo + bar;')
    end
  end
end