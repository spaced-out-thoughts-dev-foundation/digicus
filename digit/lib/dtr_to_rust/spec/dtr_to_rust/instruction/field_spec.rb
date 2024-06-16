# frozen_string_literal: true

require 'spec_helper'

describe DTRToRust::Instruction::Field do
  describe '#handle' do
    it 'returns the correct Rust code when assign exists' do
      instruction = {
        instruction: 'field',
        inputs: %w[state count],
        assign: 'BINARY_EXPRESSION_LEFT',
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('        let mut BINARY_EXPRESSION_LEFT = state.count;')
    end

    it 'returns the correct Rust code when there is no assign' do
      instruction = {
        instruction: 'field',
        inputs: %w[state count],
        assign: nil,
        scope: 0
      }

      expect(described_class.handle(instruction)).to eq('        state.count;')
    end
  end
end
