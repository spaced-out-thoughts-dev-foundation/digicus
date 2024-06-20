# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Divide do
  describe '#handle' do
    it 'returns the correct Rust code' do
      instruction = DTRCore::Instruction.new('divide', %w[foo bar], 'foo', 0)
      expect(described_class.handle(instruction, [], [], false)).to eq('        foo = foo / bar;')
    end
  end
end
