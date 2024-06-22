# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Field do
  describe '#handle' do
    it 'returns the correct Rust code when assign exists' do
      instruction = DTRCore::Instruction.new('field', %w[state count], 'BINARY_EXPRESSION_LEFT', 0)

      expect(described_class.handle(instruction, 0, [], [],
                                    false)).to eq('        let mut BINARY_EXPRESSION_LEFT = state.count;')
    end

    it 'returns the correct Rust code when there is no assign' do
      instruction = DTRCore::Instruction.new('field', %w[state count], nil, 0)

      expect(described_class.handle(instruction, 0, [], [], false)).to eq('        state.count;')
    end
  end
end
